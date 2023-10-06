// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use pyxis_sm::plugin_manager_msg::PluginType;

#[cw_serde]
pub struct Plugin {
    pub name: String,
    pub plugin_type: PluginType,
    pub code_id: u64,
    pub version: String,
    pub address: Addr,
    pub checksum: String,
}

pub const PLUGINS: Map<&str, Plugin> = Map::new("plugins");
