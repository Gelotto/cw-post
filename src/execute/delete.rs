use crate::{error::ContractError, msg::DeleteMsg};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_delete_node(
    ctx: Context,
    msg: DeleteMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let node_id = msg.node_id;

    Ok(Response::new().add_attributes(vec![attr("action", "delete")]))
}
