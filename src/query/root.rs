use cosmwasm_std::{Order, StdResult};

use crate::{
    error::ContractError,
    models::Node,
    msg::{ConfigResponse, RootResponse},
    state::{CONFIG, IX_RANKED_PARENT_CHILD_ID, NODE_HEADER, NODE_STATUS},
};

use super::{nodes::build_node, ReadonlyContext};

pub const PREVIEW_REPLY_COUNT: usize = 10;

pub fn query_root(ctx: ReadonlyContext) -> Result<RootResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let root = build_node(deps.storage, NODE_HEADER.load(deps.storage, &"1".to_owned())?)?.unwrap();
    let mut replies: Vec<Node> = Vec::with_capacity(PREVIEW_REPLY_COUNT);

    for result in IX_RANKED_PARENT_CHILD_ID
        .keys(deps.storage, None, None, Order::Descending)
        .collect::<Vec<StdResult<_>>>()
    {
        let (_, _, child_id) = result?;

        if NODE_STATUS.has(deps.storage, &child_id) {
            continue;
        }

        let header = NODE_HEADER.load(deps.storage, &child_id)?;

        if let Some(node) = build_node(deps.storage, header)? {
            replies.push(node);
        }

        if replies.len() == PREVIEW_REPLY_COUNT {
            break;
        }
    }
    Ok(RootResponse { root, replies })
}
