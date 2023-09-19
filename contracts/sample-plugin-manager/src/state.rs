// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus

use cosmwasm_schema::cw_serde;
use cw_storage_plus::Map;

#[cw_serde]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub address: String,
}

pub const PLUGINS: Map<&str, Plugin> = Map::new("plugins");
