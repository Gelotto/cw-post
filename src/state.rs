use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Response, Storage, Timestamp, Uint128, Uint64};

use crate::{
    error::ContractError,
    execute::Context,
    math::add_u64,
    models::{Link, NodeStatus},
    msg::{InstantiateMsg, NodeInitArgs},
};
use cw_storage_plus::{Item, Map};

use super::models::Config;

pub const NODE_ID_COUNTER_KEY: &str = "node_id";
pub const NUM_NODES_COUNTER_KEY: &str = "num_nodes";

pub const OPERATOR: Item<Addr> = Item::new("op");
pub const CONFIG: Item<Config> = Item::new("config");
pub const CREATED_BY: Item<Addr> = Item::new("created_by");
pub const CREATED_AT: Item<Timestamp> = Item::new("created_at");
pub const COUNTERS: Map<&str, Uint64> = Map::new("counters");
pub const TOTAL_TIP_AMOUNT: Item<Uint128> = Item::new("total_tip_amount");
pub const NODE_STATUS: Map<&String, NodeStatus> = Map::new("node_status");
pub const NODE_HEADER: Map<&String, NodeHeader> = Map::new("nh");
pub const NODE_ATTRS: Map<&String, NodeAttributes> = Map::new("node_attrs");
pub const NODE_TAGS: Map<&String, Vec<String>> = Map::new("node_tags");
pub const NODE_UPDATED_AT: Map<&String, Timestamp> = Map::new("node_t_updated");
pub const NODE_NUM_REPLIES: Map<&String, u16> = Map::new("n_replies");
pub const NODE_NUM_REACTIONS: Map<&String, u16> = Map::new("n_reacts");
pub const NODE_NUM_LIKES: Map<&String, u32> = Map::new("n_likes");
pub const NODE_TOTAL_TIP_AMOUNT: Map<&String, Uint128> = Map::new("tip_totals");
pub const IX_PARENT_CHILD_ID: Map<(&String, &String), u8> = Map::new("npc");
pub const IX_RANKED_PARENT_CHILD_ID: Map<(&String, u32, &String), u8> = Map::new("nrpc");
pub const IX_ADDR_LIKED_ID: Map<(&Addr, &String), u8> = Map::new("ali");
pub const IX_LIKED_ID_ADDR: Map<(&String, &Addr), u8> = Map::new("lai");
pub const IX_TAG_NODE_ID: Map<(&String, u32, &String), u8> = Map::new("tni");

#[cw_serde]
pub struct NodeHeader {
    pub id: String,
    pub parent_id: String,
    pub created_by: Addr,
}

#[cw_serde]
pub struct NodeAttributes {
    pub created_at: Timestamp,
    pub title: String,
    pub body: Option<String>,
    pub links: Vec<Link>,
}

/// Top-level initialization of contract state
pub fn init(
    ctx: Context,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let Context { deps, env, info } = ctx;
    let InstantiateMsg {
        config,
        operator,
        mut root,
    } = msg;

    CREATED_AT.save(deps.storage, &env.block.time)?;
    CREATED_BY.save(deps.storage, &info.sender)?;
    TOTAL_TIP_AMOUNT.save(deps.storage, &Uint128::zero())?;
    CONFIG.save(deps.storage, &config)?;
    OPERATOR.save(
        deps.storage,
        &deps
            .api
            .addr_validate(operator.unwrap_or_else(|| info.sender.clone()).as_str())?,
    )?;

    // Ensure root node's parent ID is empty string
    root.parent_id = String::from("");

    // Create post's root node in the reply tree
    init_node(deps.storage, &info.sender, env.block.time, root)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

pub fn init_node(
    store: &mut dyn Storage,
    sender: &Addr,
    time: Timestamp,
    args: NodeInitArgs,
) -> Result<(), ContractError> {
    // Tick up total count of nodes in tree
    increment_counter(store, NUM_NODES_COUNTER_KEY, 1u64)?.to_string();

    // Get next node ID for inserting in reply tree
    let node_id = increment_counter(store, NODE_ID_COUNTER_KEY, 1u64)?.to_string();

    // Save entry in table for looking up child ID's given parent ID
    IX_PARENT_CHILD_ID.save(store, (&"".to_owned(), &node_id), &0)?;
    IX_RANKED_PARENT_CHILD_ID.save(store, (&"".to_owned(), 0, &node_id), &0)?;

    // Save node data that's frequently loaded by biz logic
    NODE_HEADER.save(
        store,
        &node_id,
        &NodeHeader {
            id: node_id.to_owned(),
            parent_id: args.parent_id.to_owned(),
            created_by: sender.to_owned(),
        },
    )?;

    // Save data that changes on specific executions
    NODE_UPDATED_AT.save(store, &node_id, &time)?;
    NODE_NUM_REPLIES.save(store, &args.parent_id, &0)?;
    NODE_STATUS.save(store, &node_id, &NodeStatus::Normal)?;

    // Save node data that only changes on user edits
    let node_data = NodeAttributes {
        links: args.links.unwrap_or_default(),
        created_at: time,
        title: args.title,
        body: args.body,
    };

    NODE_ATTRS.save(store, &node_id, &node_data)?;

    let tags = args.tags.unwrap_or_default();

    NODE_TAGS.save(store, &node_id, &tags)?;

    // Insert entries in lookup table used for finding nodes by tag
    for tag in tags.iter() {
        let tag = tag.to_lowercase();
        IX_TAG_NODE_ID.save(store, (&tag, 0, &node_id), &0)?;
    }

    Ok(())
}

pub fn increment_counter<T: Into<Uint64>>(
    store: &mut dyn Storage,
    key: &str,
    delta: T,
) -> Result<Uint64, ContractError> {
    COUNTERS.update(store, key, |n| -> Result<_, ContractError> {
        add_u64(n.unwrap_or_default(), delta)
    })
}
