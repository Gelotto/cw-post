use crate::error::ContractError;
use crate::execute::delete::exec_delete_node;
use crate::execute::like::exec_toggle_like;
use crate::execute::react::exec_toggle_reaction;
use crate::execute::reply::exec_reply;
use crate::execute::tip::exec_tip;
use crate::execute::{configure::exec_configure, Context};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, NodesQueryMsg, QueryMsg};
use crate::query::cost::query_cost;
use crate::query::nodes::{query_nodes_by_id, query_nodes_by_parent_id, query_nodes_by_tag};
use crate::query::{config::query_config, ReadonlyContext};
use crate::state;
use cosmwasm_std::{entry_point, to_json_binary};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-post";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(state::init(Context { deps, env, info }, msg)?)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let ctx = Context { deps, env, info };
    match msg {
        ExecuteMsg::Configure(config) => exec_configure(ctx, config),
        ExecuteMsg::Reply(msg) => exec_reply(ctx, msg),
        ExecuteMsg::Like(msg) => exec_toggle_like(ctx, msg),
        ExecuteMsg::React(msg) => exec_toggle_reaction(ctx, msg),
        ExecuteMsg::Tip(msg) => exec_tip(ctx, msg),
        ExecuteMsg::Delete(msg) => exec_delete_node(ctx, msg),
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    let ctx = ReadonlyContext { deps, env };
    let result = match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(ctx)?),
        QueryMsg::Cost(args) => to_json_binary(&query_cost(ctx, args)?),
        QueryMsg::Nodes(msg) => match msg {
            NodesQueryMsg::ByParentId(params) => to_json_binary(&query_nodes_by_id(ctx, params)?),
            NodesQueryMsg::ById(params) => to_json_binary(&query_nodes_by_parent_id(ctx, params)?),
            NodesQueryMsg::ByTag(params) => to_json_binary(&query_nodes_by_tag(ctx, params)?),
        },
    }?;
    Ok(result)
}

#[entry_point]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
