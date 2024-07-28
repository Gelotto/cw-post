use std::marker::PhantomData;

use cosmwasm_std::{Order, StdResult, Storage};
use cw_storage_plus::Bound;

use crate::{
    error::ContractError,
    models::Node,
    msg::{NodesByIdQueryArgs, NodesByParentIdQueryArgs, NodesByTagQueryArgs, NodesPaginationResponse, OrderBy},
    state::{
        NodeAttributes, NodeHeader, IX_PARENT_CHILD_ID, IX_RANKED_PARENT_CHILD_ID, IX_TAG_NODE_ID, NODE_ATTRS,
        NODE_HEADER, NODE_NUM_REACTIONS, NODE_NUM_REPLIES, NODE_STATUS, NODE_TAGS, NODE_TOTAL_TIP_AMOUNT,
        NODE_UPDATED_AT,
    },
};

use super::ReadonlyContext;

pub const MAX_LIMIT: u8 = 50;

/// Query nodes by ID or by parent ID
pub fn query_nodes_by_id(
    ctx: ReadonlyContext,
    params: NodesByIdQueryArgs,
) -> Result<NodesPaginationResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let limit = params.limit.min(MAX_LIMIT) as usize;
    let ids = params.ids;
    let mut nodes: Vec<Node> = Vec::with_capacity(limit);
    let mut cursor_values: Box<Vec<String>> = Box::new(vec![]);

    let next_cursor: Option<Vec<String>> = {
        // Prepare args for Map range
        let (min_bound, max_bound, order) = if params.desc {
            let order = Order::Descending;
            let max_bound: Option<Bound<&String>> = None;
            let min_bound = params.cursor.and_then(|vals| {
                *cursor_values = vals;
                Some(Bound::Exclusive((&(*cursor_values)[0], PhantomData)))
            });
            (min_bound, max_bound, order)
        } else {
            let order = Order::Ascending;
            let min_bound: Option<Bound<&String>> = None;
            let max_bound = params.cursor.and_then(|vals| {
                *cursor_values = vals;
                Some(Bound::Exclusive((&(*cursor_values)[0], PhantomData)))
            });
            (min_bound, max_bound, order)
        };

        // Build return Nodes vec
        for result in NODE_HEADER
            .range(deps.storage, min_bound, max_bound, order)
            .collect::<Vec<StdResult<_>>>()
        {
            if let Some(node) = build_node(deps.storage, result?.1)? {
                nodes.push(node);
            }
            if nodes.len() == limit {
                break;
            }
        }

        // Get next cursor to return. This corresponds to a position in the
        // provided ID's vec.
        if let Some(u) = nodes.last() {
            if *ids.last().unwrap() == u.id {
                None
            } else {
                Some(vec![u.id.clone()])
            }
        } else {
            None
        }
    };

    Ok(NodesPaginationResponse {
        cursor: next_cursor,
        nodes,
    })
}

pub fn query_nodes_by_parent_id(
    ctx: ReadonlyContext,
    params: NodesByParentIdQueryArgs,
) -> Result<NodesPaginationResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let limit = params.limit.min(MAX_LIMIT) as usize;
    let parent_id = params.parent_id;
    let mut nodes: Vec<Node> = Vec::with_capacity(limit);
    let mut cursor_values: Box<Vec<String>> = Box::new(vec![]);

    // Build return Nodes vec
    // Return child nodes of the given parent ID
    let next_cursor: Option<Vec<String>> = {
        match params.order_by {
            // Return child nodes in order of creation time
            OrderBy::Time => {
                // Prepare args for Map range
                let (min_bound, max_bound, order) = if params.desc {
                    let order = Order::Descending;
                    let max_bound: Option<Bound<&String>> = None;
                    let min_bound = params.cursor.and_then(|vals| {
                        *cursor_values = vals;
                        Some(Bound::Exclusive((&(*cursor_values)[0], PhantomData)))
                    });
                    (min_bound, max_bound, order)
                } else {
                    let order = Order::Ascending;
                    let min_bound: Option<Bound<&String>> = None;
                    let max_bound = params.cursor.and_then(|vals| {
                        *cursor_values = vals;
                        Some(Bound::Exclusive((&(*cursor_values)[0], PhantomData)))
                    });
                    (min_bound, max_bound, order)
                };

                for result in IX_PARENT_CHILD_ID
                    .prefix(&parent_id)
                    .keys(deps.storage, min_bound, max_bound, order)
                    .collect::<Vec<StdResult<_>>>()
                {
                    let child_id = result?;
                    let header = NODE_HEADER.load(deps.storage, &child_id)?;
                    if let Some(node) = build_node(deps.storage, header)? {
                        nodes.push(node);
                    }
                    if nodes.len() == limit {
                        break;
                    }
                }

                // Get next cursor to return
                if let Some(u) = nodes.last() {
                    let ((_, final_child_id), _) = if params.desc {
                        IX_PARENT_CHILD_ID.last(deps.storage)
                    } else {
                        IX_PARENT_CHILD_ID.first(deps.storage)
                    }?
                    .unwrap();

                    if final_child_id == u.id {
                        None
                    } else {
                        Some(vec![u.id.clone()])
                    }
                } else {
                    None
                }
            },
            // Return child nodes in order of like count
            OrderBy::Likes => {
                // Prepare args for Map range
                let (min_bound, max_bound, order) = if params.desc {
                    let order = Order::Descending;
                    let max_bound: Option<Bound<(&String, u32, &String)>> = None;
                    let min_bound: Option<Bound<(&String, u32, &String)>> = params.cursor.and_then(|vals| {
                        *cursor_values = vals;
                        Some(Bound::Exclusive((
                            (
                                &parent_id,
                                (*cursor_values)[0].parse::<u32>().unwrap(),
                                &(*cursor_values)[1],
                            ),
                            PhantomData,
                        )))
                    });
                    (min_bound, max_bound, order)
                } else {
                    let order = Order::Ascending;
                    let min_bound: Option<Bound<(&String, u32, &String)>> = None;
                    let max_bound: Option<Bound<(&String, u32, &String)>> = params.cursor.and_then(|vals| {
                        *cursor_values = vals;
                        Some(Bound::Exclusive((
                            (
                                &parent_id,
                                (*cursor_values)[0].parse::<u32>().unwrap(),
                                &(*cursor_values)[1],
                            ),
                            PhantomData,
                        )))
                    });
                    (min_bound, max_bound, order)
                };

                let mut tail_child_rank = 0u32;

                for result in IX_RANKED_PARENT_CHILD_ID
                    .keys(deps.storage, min_bound, max_bound, order)
                    .collect::<Vec<StdResult<_>>>()
                {
                    let (_, child_rank, child_id) = result?;
                    let header = NODE_HEADER.load(deps.storage, &child_id)?;

                    if let Some(node) = build_node(deps.storage, header)? {
                        nodes.push(node);
                    }

                    tail_child_rank = child_rank;

                    if nodes.len() == limit {
                        break;
                    }
                }

                // Get next cursor to return
                if let Some(u) = nodes.last() {
                    let ((_, _, final_child_id), _) = if params.desc {
                        IX_RANKED_PARENT_CHILD_ID.last(deps.storage)
                    } else {
                        IX_RANKED_PARENT_CHILD_ID.first(deps.storage)
                    }?
                    .unwrap();

                    if final_child_id == u.id {
                        None
                    } else {
                        Some(vec![tail_child_rank.to_string(), u.id.clone()])
                    }
                } else {
                    None
                }
            },
        }
    };

    Ok(NodesPaginationResponse {
        cursor: next_cursor,
        nodes,
    })
}

pub fn query_nodes_by_tag(
    ctx: ReadonlyContext,
    params: NodesByTagQueryArgs,
) -> Result<NodesPaginationResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let limit = params.limit.min(MAX_LIMIT) as usize;

    // Prepare args for Map range. The cursor value is a node ID string
    let mut cursor_values: Box<Vec<String>> = Box::new(vec![]);
    let (min_bound, max_bound, order) = if params.desc {
        let order = Order::Descending;
        let max_bound: Option<Bound<(&String, u32, &String)>> = None;
        let min_bound = params.cursor.and_then(|vals| {
            *cursor_values = vals;
            Some(Bound::Exclusive((
                (&params.tag, cursor_values[0].parse::<u32>().unwrap(), &cursor_values[1]),
                PhantomData,
            )))
        });
        (min_bound, max_bound, order)
    } else {
        let order = Order::Ascending;
        let min_bound: Option<Bound<(&String, u32, &String)>> = None;
        let max_bound = params.cursor.and_then(|vals| {
            *cursor_values = vals;
            Some(Bound::Exclusive((
                (&params.tag, cursor_values[0].parse::<u32>().unwrap(), &cursor_values[1]),
                PhantomData,
            )))
        });
        (min_bound, max_bound, order)
    };

    // Build return nodes
    let mut nodes: Vec<Node> = Vec::with_capacity(limit);

    for result in IX_TAG_NODE_ID
        .keys(deps.storage, min_bound, max_bound, order)
        .collect::<Vec<StdResult<_>>>()
    {
        let (_, _, node_id) = result?;

        let header = NODE_HEADER.load(deps.storage, &node_id)?;
        if let Some(node) = build_node(deps.storage, header)? {
            nodes.push(node);
        }
        if nodes.len() == limit {
            break;
        }
    }

    // Get next cursor to return
    let next_cursor = if let Some(u) = nodes.last() {
        if nodes.len() == limit {
            Some(vec![u.id.clone()])
        } else {
            None
        }
    } else {
        None
    };

    Ok(NodesPaginationResponse {
        cursor: next_cursor,
        nodes,
    })
}

pub fn build_node(
    store: &dyn Storage,
    node_header: NodeHeader,
) -> Result<Option<Node>, ContractError> {
    let NodeHeader {
        id,
        parent_id,
        created_by,
    } = node_header;
    let status = NODE_STATUS.load(store, &id)?;
    let updated_at = NODE_UPDATED_AT.load(store, &id)?;
    let n_replies = NODE_NUM_REPLIES.load(store, &id)?;
    let n_reactions = NODE_NUM_REACTIONS.load(store, &id)?;
    let royalties = NODE_TOTAL_TIP_AMOUNT.load(store, &id)?;
    let tags = NODE_TAGS.load(store, &id)?;
    let NodeAttributes {
        created_at,
        title,
        body,
        links,
    } = NODE_ATTRS.load(store, &id)?;

    Ok(Some(Node {
        id,
        status,
        parent_id,
        created_by,
        created_at,
        updated_at,
        n_replies,
        n_reactions,
        royalties,
        title,
        body,
        links,
        tags,
    }))
}
