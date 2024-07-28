use crate::{
    error::ContractError,
    msg::LikeMsg,
    state::{
        NodeHeader, CONFIG, IX_ADDR_LIKED_ID, IX_LIKED_ID_ADDR, IX_RANKED_PARENT_CHILD_ID, IX_TAG_NODE_ID, NODE_HEADER,
        NODE_NUM_LIKES, NODE_TAGS,
    },
};
use cosmwasm_std::Response;

use super::{tip::apply_tip_if_exists, Context};

pub fn exec_toggle_like(
    ctx: Context,
    msg: LikeMsg,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;
    let LikeMsg { node_id, tip_amount } = msg;
    let config = CONFIG.load(deps.storage)?;
    let NodeHeader {
        created_by: tip_recipient,
        parent_id,
        ..
    } = NODE_HEADER.load(deps.storage, &node_id)?;

    // Currently number of likes received by the node
    let n_likes = NODE_NUM_LIKES.may_load(deps.storage, &node_id)?.unwrap_or_default();

    if IX_ADDR_LIKED_ID.has(deps.storage, (&info.sender, &node_id)) {
        // Sender already liked, so we unlike. Decrement or remove state data
        IX_LIKED_ID_ADDR.remove(deps.storage, (&node_id, &info.sender));
        if n_likes > u32::MIN {
            let next_n_likes = n_likes - 1;
            IX_RANKED_PARENT_CHILD_ID.remove(deps.storage, (&parent_id, n_likes, &node_id));
            if next_n_likes > 0 {
                IX_RANKED_PARENT_CHILD_ID.save(deps.storage, (&parent_id, next_n_likes, &node_id), &0)?;
                NODE_NUM_LIKES.save(deps.storage, &node_id, &next_n_likes)?;
            } else {
                NODE_NUM_LIKES.remove(deps.storage, &node_id);
            }
        }
    } else {
        // Sender is liking
        IX_LIKED_ID_ADDR.save(deps.storage, (&node_id, &info.sender), &0)?;
        if n_likes < u32::MAX {
            let next_n_likes = n_likes + 1;
            IX_RANKED_PARENT_CHILD_ID.remove(deps.storage, (&parent_id, n_likes, &node_id));
            IX_RANKED_PARENT_CHILD_ID.save(deps.storage, (&parent_id, next_n_likes, &node_id), &0)?;

            for tag in NODE_TAGS.load(deps.storage, &node_id)?.iter() {
                IX_TAG_NODE_ID.remove(deps.storage, (tag, n_likes, &node_id));
                IX_TAG_NODE_ID.save(deps.storage, (tag, next_n_likes, &node_id), &0)?;
            }

            NODE_NUM_LIKES.save(deps.storage, &node_id, &next_n_likes)?;
        }
    }

    // TODO: add submsg to factory to:
    //  - create relation <(post)--{like: nodeId}--(sender)>

    // Apply any included tip (for the "like" case)
    apply_tip_if_exists(
        deps.storage,
        Response::new().add_attribute("action", "like"),
        &config,
        tip_recipient,
        tip_amount,
        &node_id,
    )
}
