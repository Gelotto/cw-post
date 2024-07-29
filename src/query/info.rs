use crate::{
    error::ContractError,
    msg::InfoResponse,
    state::{CONFIG, COUNTERS, NODE_HEADER, NUM_NODES_COUNTER_KEY, OPERATOR, ROYALTIES},
};

use super::{nodes::build_node, ReadonlyContext};

pub const PREVIEW_REPLY_COUNT: usize = 10;

pub fn query_info(ctx: ReadonlyContext) -> Result<InfoResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;

    let n_nodes = COUNTERS.load(deps.storage, NUM_NODES_COUNTER_KEY)?;
    let royalties = ROYALTIES.load(deps.storage)?;
    let operator = OPERATOR.load(deps.storage)?;
    let config = CONFIG.load(deps.storage)?;

    Ok(InfoResponse {
        n_nodes,
        operator,
        royalties,
        config,
    })
}
