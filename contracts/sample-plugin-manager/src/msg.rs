use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use pyxis_sm::plugin_manager_msg::PluginType;
pub use pyxis_sm::plugin_manager_msg::QueryMsg;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    AllowPlugin {
        plugin_address: Addr,
        plugin_type: PluginType,
    },
    DisallowPlugin {
        plugin_address: Addr,
    },
    UpdatePlugin {
        plugin_address: Addr,
        forced_unregister: bool,
    }
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}
