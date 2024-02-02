use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use pyxis_sm::plugin_manager_msg::{PluginType, QueryPluginStatus};

#[cw_serde]
pub struct Config {
    pub plugin_manager_addr: Addr,
    pub recoverable: bool,
}

#[cw_serde]
pub struct Plugin {
    pub name: String,
    pub plugin_type: PluginType,
    pub contract_address: Addr,
    pub config: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
// PLUGINS is a map of plugin contract address to Plugin
pub const PLUGINS: Map<&Addr, Plugin> = Map::new("plugins");

// PLUGINS STATUS
pub const QUERY_PLUGINS_STATUS: Item<Vec<QueryPluginStatus>> = Item::new("plugins_status");
