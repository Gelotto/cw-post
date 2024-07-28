pub mod configure;
pub mod delete;
pub mod like;
pub mod react;
pub mod reply;
pub mod tip;

use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub struct Context<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}
