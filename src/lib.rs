extern crate core;

pub mod contract;
pub mod error;
pub mod execute;
pub mod msg;

mod state;
mod query;

pub const CONTRACT_NAME: &str = "xion-contract";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");