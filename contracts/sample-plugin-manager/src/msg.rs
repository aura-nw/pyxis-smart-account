use cosmwasm_schema::{cw_serde, QueryResponses};
pub use pyxis_sm::plugin_manager_msg::QueryMsg;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}
