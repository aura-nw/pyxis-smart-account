// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Uint128, Uint64};
use cw_storage_plus::Map;

#[cw_serde]
pub struct LimitPerTransaction {
    pub limit: Coin,
}

#[cw_serde]
pub struct LimitPeriodic {
    pub limit: Coin,
    pub used: Uint128,
    pub begin_period: Uint64,
    pub periodic: Uint64,
}

impl LimitPeriodic {
    pub fn end_period(&self) -> Uint64 {
        return self.begin_period + self.periodic;
    }
}

#[cw_serde]
pub enum Limit {
    PerTransaction(LimitPerTransaction),
    Periodic(LimitPeriodic),
}

pub const LIMITS: Map<&Addr, Vec<Limit>> = Map::new("limits");
