#[cfg(not(feature = "library"))]
pub mod contract;
pub mod error;
#[cfg(not(feature = "library"))]
pub mod execute;
pub mod fees;
mod math;
pub mod models;
pub mod msg;
#[cfg(not(feature = "library"))]
pub mod query;
pub mod state;
pub mod tokens;

#[cfg(not(target_arch = "wasm32"))]
mod interface;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::interface::CwPostContract;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::msg::{ExecuteMsgFns as CwPostExecuteMsgFns, QueryMsgFns as CwPostQueryMsgFns};
