use crate::{error::ContractError, models::Config, state::CONFIG};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_configure(
    ctx: Context,
    config: Config,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attributes(vec![attr("action", "set_config")]))
}
