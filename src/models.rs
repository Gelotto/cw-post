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
    pub token: Token,
    pub fee_recipient: Option<Addr>,
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
    pub parent_id: String,
    pub n_replies: u16,
    pub n_reactions: u16,
    pub royalties: Uint128,
    pub created_by: Addr,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub title: String,
    pub body: Option<String>,
    pub links: Vec<Link>,
    pub tags: Vec<String>,
    pub status: NodeStatus,
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
