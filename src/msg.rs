use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Uint64};

use crate::models::{Config, Link, Node};

#[cw_serde]
pub struct InstantiateMsg {
    pub config: Config,
    pub operator: Option<Addr>,
    pub root: NodeInitArgs,
}

#[cw_serde]
pub enum ExecuteMsg {
    Configure(Config),
    Reply(ReplyMsg),
    React(ReactMsg),
    Like(LikeMsg),
    Tip(TipMsg),
    Delete(DeleteMsg),
}

#[cw_serde]
pub enum NodesQueryMsg {
    ByParentId(NodesByParentIdQueryArgs),
    ByIds(NodesByIdQueryArgs),
    ByTag(NodesByTagQueryArgs),
}

#[cw_serde]
pub enum QueryMsg {
    Info {},
    Cost(CostQueryArgs),
    Nodes(NodesQueryMsg),
    Chat(ChatQueryArgs),
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct DeleteMsg {
    pub node_id: String,
}

#[cw_serde]
pub struct TipMsg {
    pub node_id: String,
    pub tip_amount: Uint128,
}

#[cw_serde]
pub enum ReactMsg {
    Image(String),
    Emoji(String),
}

pub type ReplyMsg = NodeInitArgs;
pub type LikeMsg = TipMsg;

#[cw_serde]
pub struct ChatQueryArgs {
    pub limit: u8,
    pub desc: Option<bool>,
    pub cursor: Option<String>,
}

#[cw_serde]
pub struct NodesByIdQueryArgs {
    pub ids: Vec<String>,
    pub order_by: OrderBy,
    pub limit: u8,
    pub desc: bool,
    pub cursor: Option<Vec<String>>,
}

#[cw_serde]
pub struct NodesByParentIdQueryArgs {
    pub parent_id: String,
    pub order_by: OrderBy,
    pub limit: u8,
    pub desc: bool,
    pub cursor: Option<Vec<String>>,
}

#[cw_serde]
pub struct NodesByTagQueryArgs {
    pub tag: String,
    pub limit: u8,
    pub desc: bool,
    pub cursor: Option<Vec<String>>,
}

#[cw_serde]
pub enum OrderBy {
    Time,
    Likes,
}

#[cw_serde]
pub struct CostQueryArgs {
    pub node: NodeInitArgs,
    pub is_update: bool,
}

#[cw_serde]
pub struct NodeInitArgs {
    pub title: String,
    pub body: Option<String>,
    pub links: Option<Vec<Link>>,
    pub tags: Option<Vec<String>>,
    pub parent_id: String,
}

#[cw_serde]
pub struct ConfigResponse(pub Config);

#[cw_serde]
pub struct NodesPaginationResponse {
    pub cursor: Option<Vec<String>>,
    pub nodes: Vec<Node>,
}

#[cw_serde]
pub struct ChatPaginationResponse {
    pub cursor: Option<String>,
    pub nodes: Vec<Node>,
}

#[cw_serde]
pub struct NodeCostSubtotals {
    pub creation: Uint128,
    pub body: Uint128,
    pub tags: Uint128,
    pub links: Uint128,
}

#[cw_serde]
pub struct InfoResponse {
    pub operator: Addr,
    pub config: Config,
    pub royalties: Uint128,
    pub n_nodes: Uint64,
}

#[cw_serde]
pub struct CostResponse {
    pub total: Uint128,
    pub subtotals: NodeCostSubtotals,
}
