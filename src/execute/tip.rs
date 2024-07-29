use crate::{
    error::ContractError,
    math::{add_u128, mul_ratio_u128},
    models::{Config, FeeParams},
    msg::TipMsg,
    state::{NodeHeader, CONFIG, NODE_HEADER, NODE_ROYALTIES, ROYALTIES},
};
use cosmwasm_std::{attr, Addr, Response, Storage, Uint128};

use super::Context;

pub fn exec_tip(
    ctx: Context,
    msg: TipMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let TipMsg { node_id, tip_amount } = msg;
    let config = CONFIG.load(deps.storage)?;
    let NodeHeader {
        created_by: tip_recipient,
        ..
    } = NODE_HEADER.load(deps.storage, &node_id)?;

    apply_tip_if_exists(
        deps.storage,
        Response::new().add_attribute("action", "tip"),
        &config,
        tip_recipient,
        tip_amount,
        &node_id,
    )
}

pub fn apply_tip_if_exists(
    store: &mut dyn Storage,
    resp: Response,
    config: &Config,
    tip_recipient: Addr,
    tip_amount: Uint128,
    node_id: &String,
) -> Result<Response, ContractError> {
    // Increment aggregate total tip amount
    ROYALTIES.update(store, |n| -> Result<_, ContractError> { add_u128(n, tip_amount) })?;

    // Increment node-specific tip amount
    NODE_ROYALTIES.update(store, node_id, |n| -> Result<_, ContractError> {
        add_u128(n.unwrap_or_default(), tip_amount)
    })?;

    // Calc fee and node-creator royalties, adding transfer msgs to Response
    let (royalty_amount, fee_amount) = process_tip_amount(tip_amount, &config)?;
    let mut resp = resp;

    if !fee_amount.is_zero() {
        // We can unwrap fee_recipient following process_tip_amount
        let fee_recipient = config.fee_recipient.to_owned().unwrap();
        resp = resp
            .add_submessage(config.token.transfer(&fee_recipient, fee_amount)?)
            .add_attribute("fee_amount", fee_amount.to_string())
            .add_attribute("fee_recipient", fee_recipient.to_string());
    }

    Ok(resp
        .add_attributes(vec![attr("royalty_amount", royalty_amount.to_string())])
        .add_submessage(config.token.transfer(&tip_recipient, royalty_amount)?))
}

fn process_tip_amount(
    tip_amount: Uint128,
    config: &Config,
) -> Result<(Uint128, Uint128), ContractError> {
    let Config {
        fees: FeeParams { tip_pct: fee_pct, .. },
        fee_recipient,
        ..
    } = config;
    let fee_amount = if fee_recipient.is_some() && !fee_pct.is_zero() {
        mul_ratio_u128(tip_amount, *fee_pct, 1_000_000u128)?
    } else {
        Uint128::zero()
    };
    Ok((tip_amount - fee_amount, fee_amount))
}
