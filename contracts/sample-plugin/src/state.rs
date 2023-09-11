// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Map;

// we will just store the address of smart account and their config
#[cw_serde]
pub struct UserConfig {
    pub address: Addr,
    pub config: String,
}

pub const USER_CONFIGS: Map<&Addr, UserConfig> = Map::new("user_configs");
