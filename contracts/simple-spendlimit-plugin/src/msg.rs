use crate::state::Limit;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use pyxis_sm_derive::{base_plugin_execute, base_plugin_query};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[base_plugin_execute]
#[cw_serde]
pub enum ExecuteMsg {
    AddLimit { limit: Limit },
    UpdateLimit { index: u32, limit: Limit },
    DeleteLimit { index: u32 },
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[base_plugin_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Option<Vec<Limit>>)]
    GetLimits { address: Addr },
}

// We define a custom struct for each query response
// #[cw_serde]
// pub struct YourQueryResponse {}
