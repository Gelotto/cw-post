use cosmwasm_std::Uint128;

use crate::{error::ContractError, math::mul_u128, models::FeeParams, msg::NodeCostSubtotals};

pub const UNIT_TEXT_LENGTH: usize = 280;

pub fn compute_node_cost(
    fees: &FeeParams,
    is_update: bool,
    body_len: usize,
    n_tags: usize,
    n_links: usize,
) -> Result<(Uint128, NodeCostSubtotals), ContractError> {
    let tag_fee = mul_u128(fees.tag, n_tags as u128)?;
    let link_fee = mul_u128(fees.link, n_links as u128)?;
    let text_fee = mul_u128(fees.text, (body_len / UNIT_TEXT_LENGTH) as u128)?;
    let creation_fee = if is_update {
        Uint128::zero()
    } else {
        Uint128::from(fees.creation)
    };

    Ok((
        tag_fee + link_fee + text_fee + creation_fee,
        NodeCostSubtotals {
            creation: creation_fee,
            body: text_fee,
            tags: tag_fee,
            links: link_fee,
        },
    ))
}
