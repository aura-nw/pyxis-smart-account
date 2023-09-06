use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use pyxis_sm::msg::PyxisExecuteMsg;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    RegisterPlugin {
        plugin_address: Addr,
        checksum: String,
        config: String,
    },
    UnregisterPlugin {
        plugin_address: Addr,
    },
    EnablePlugin {
        plugin_address: Addr,
    },
    DisablePlugin {
        plugin_address: Addr,
    },
    PyxisExecuteMsg(PyxisExecuteMsg),
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // This example query variant indicates that any client can query the contract
    // using `YourQuery` and it will return `YourQueryResponse`
    // This `returns` information will be included in contract's schema
    // which is used for client code generation.
    //
    // #[returns(YourQueryResponse)]
    // YourQuery {},
}

// We define a custom struct for each query response
// #[cw_serde]
// pub struct YourQueryResponse {}
