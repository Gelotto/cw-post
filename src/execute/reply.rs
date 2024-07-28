use crate::{error::ContractError, msg::ReplyMsg, state::init_node};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_reply(
    ctx: Context,
    node_init_args: ReplyMsg,
) -> Result<Response, ContractError> {
    let Context { deps, info, env } = ctx;

    init_node(deps.storage, &info.sender, env.block.time, node_init_args)?;

    Ok(Response::new().add_attributes(vec![attr("action", "reply")]))
}
