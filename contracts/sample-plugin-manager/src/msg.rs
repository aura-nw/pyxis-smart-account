use crate::state::Plugin;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
pub use pyxis_sm::plugin_manager_msg::QueryMsg;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    AllowPlugin { plugin_info: Plugin },
    DisallowPlugin { plugin_address: Addr },
    UpdatePlugin { plugin_info: Plugin },
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}
