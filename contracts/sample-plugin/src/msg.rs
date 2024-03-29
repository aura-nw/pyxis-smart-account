use cosmwasm_schema::{cw_serde, QueryResponses};
use pyxis_sm_derive::base_plugin_query;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

/// Message type for `query` entry_point
#[base_plugin_query]
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
