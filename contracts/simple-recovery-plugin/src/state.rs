// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Map;

#[cw_serde]
pub struct RecoveryConfig {
    pub smart_account_address: Addr,
    pub recover_address: Addr,
}

impl From<String> for RecoveryConfig {
    fn from(config: String) -> Self {
        serde_json_wasm::from_str::<RecoveryConfig>(&config).unwrap()
    }
}

pub const CONFIG_MAP: Map<&Addr, RecoveryConfig> = Map::new("config");
