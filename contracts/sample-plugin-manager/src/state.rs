// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use pyxis_sm::plugin_manager_msg::{
    PluginResponse, PluginStatus, PluginType, UnregisterRequirement,
};

#[cw_serde]
pub struct Plugin {
    pub name: String,
    pub plugin_type: PluginType,
    pub code_id: u64,
    pub version: String,
    pub address: Addr,
    pub status: PluginStatus,
    pub unregister_req: UnregisterRequirement,
}

impl Into<PluginResponse> for Plugin {
    fn into(self) -> PluginResponse {
        PluginResponse {
            name: self.name,
            plugin_type: self.plugin_type,
            version: self.version,
            address: self.address.to_string(),
            code_id: self.code_id,
            status: self.status,
            unregister_req: self.unregister_req,
        }
    }
}

pub const PLUGINS: Map<&str, Plugin> = Map::new("plugins");
