use crate::state::{Plugin, PluginStatus};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub plugin_manager_addr: Addr,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    RegisterPlugin {
        plugin_address: Addr,
        config: String,
    },
    UnregisterPlugin {
        plugin_address: Addr,
    },
    UpdatePlugin {
        plugin_address: Addr,
        status: PluginStatus,
    },
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AllPluginsResponse)]
    AllPlugins {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct AllPluginsResponse {
    pub plugins: Vec<Plugin>,
}
