use crate::{error::ContractError, msg::ReactMsg};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_toggle_reaction(
    ctx: Context,
    msg: ReactMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    Ok(Response::new().add_attributes(vec![attr("action", "react")]))
}
