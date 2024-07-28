use crate::{
    error::ContractError,
    fees::compute_node_cost,
    msg::{CostQueryArgs, CostResponse},
    state::CONFIG,
};

use super::ReadonlyContext;

pub fn query_cost(
    ctx: ReadonlyContext,
    args: CostQueryArgs,
) -> Result<CostResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let CostQueryArgs { is_update, node } = args;
    let config = CONFIG.load(deps.storage)?;

    let (total, subtotals) = compute_node_cost(
        &config.fees,
        is_update,
        node.body.unwrap_or_default().len(),
        node.tags.unwrap_or_default().len(),
        node.links.unwrap_or_default().len(),
    )?;

    Ok(CostResponse { total, subtotals })
}
