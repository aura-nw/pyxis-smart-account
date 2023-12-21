use crate::state::Plugin;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_ownable::cw_ownable_execute;
pub use pyxis_sm::plugin_manager_msg::QueryMsg;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

/// Message type for `execute` entry_point
#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    AllowPlugin {
        plugin_info: Plugin,
    },
    DisallowPlugin {
        plugin_address: Addr,
    },
    UpdatePlugin {
        plugin_info: Plugin,
    },
    MigratePlugin {
        plugin_address: String,
        new_code_id: u64,
        msg: String,
    },
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}
