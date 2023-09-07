use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Map;

#[cw_serde]
pub struct Plugin {
    pub contract_address: Addr,
    pub checksum: String,
    pub status: bool,
    pub config: String,
}

// PLUGINS is a map of plugin contract address to Plugin
pub const PLUGINS: Map<&Addr, Plugin> = Map::new("plugins");
