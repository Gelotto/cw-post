use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};

use crate::tokens::Token;

#[cw_serde]
pub enum NodeStatus {
    Normal,
    Deleted,
}

#[cw_serde]
pub struct Config {
    /// The token type used for payments & fees
    pub token: Token,
    /// Address to send fees to
    pub fee_recipient: Option<Addr>,
    /// Prices and fee rates for various actions
    pub fees: FeeParams,
}

#[cw_serde]
pub struct FeeParams {
    /// Base cost of main post/thread creation
    pub creation: Uint128,
    /// Unit cost for each reaction
    pub reaction: Uint128,
    /// Unit cost for each link attached to a post
    pub link: Uint128,
    /// Unit cost per 280-char block of text in post body
    pub text: Uint128,
    /// Unit cost per tag
    pub tag: Uint128,
    /// Fee rate applied to tips
    pub tip_pct: Uint128,
}

#[cw_serde]
pub struct Node {
    pub id: String,
    /// Biz logic status of node
    pub status: NodeStatus,
    /// Parent node ID
    pub parent_id: String,
    /// Number of child nodes
    pub n_replies: u16,
    /// Number of reactions to the node
    pub n_reactions: u16,
    /// Total tip amount received by node creator
    pub royalties: Uint128,
    /// Account that created the node
    pub created_by: Addr,
    /// Block time on creation
    pub created_at: Timestamp,
    /// Block time when last edited by creator
    pub updated_at: Timestamp,
    /// HTML title of the post
    pub title: String,
    /// HTML body of the post
    pub body: Option<String>,
    /// URL links associated with the post
    pub links: Vec<Link>,
    /// Tags associated with the post
    pub tags: Vec<String>,
    /// Not Safe For Work flag
    pub nsfw: bool,
}

#[cw_serde]
pub enum Link {
    Generic {
        url: String,
        label: Option<String>,
    },
    Image {
        url: String,
        label: Option<String>,
    },
    Video {
        url: String,
        label: Option<String>,
        provider: String,
    },
    Audio {
        url: String,
        label: Option<String>,
        provider: String,
    },
}
