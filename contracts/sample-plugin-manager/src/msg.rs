use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
pub use pyxis_sm::plugin_manager_msg::QueryMsg;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    AllowPlugin { plugin_address: Addr },
    DisallowPlugin { plugin_address: Addr },
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}
