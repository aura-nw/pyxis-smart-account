use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use pyxis_sm::plugin_manager_msg::PluginType;

#[cw_serde]
pub struct Config {
    pub plugin_manager_addr: Addr,
    pub recoverable: bool,
}

#[cw_serde]
pub enum PluginStatus {
    Active,
    Inactive,
}

#[cw_serde]
pub struct Plugin {
    pub name: String,
    pub plugin_type: PluginType,
    pub contract_address: Addr,
    pub checksum: String,
    pub status: PluginStatus,
    pub config: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
// PLUGINS is a map of plugin contract address to Plugin
pub const PLUGINS: Map<&Addr, Plugin> = Map::new("plugins");

pub const ERROR_LOG: Item<Option<String>> = Item::new("error log");
