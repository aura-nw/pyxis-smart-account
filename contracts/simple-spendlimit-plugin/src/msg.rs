use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use pyxis_sm::msg::{CallInfo, SdkMsg};
use crate::state::Limit;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    Register { 
        config: String 
    },

    Unregister {},

    PreExecute { 
        msgs: Vec<SdkMsg>, 
        call_info: CallInfo, 
        is_authz: bool 
    },

    AfterExecute { 
        msgs: Vec<SdkMsg>, 
        call_info: CallInfo, 
        is_authz: bool 
    },

    AddLimit {
        limit: Limit
    },
    UpdateLimit {
        index: u32,
        limit: Limit,
    },
    DeleteLimit {
        index: u32
    }
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Option<Vec<Limit>>)]
    GetLimits {
        address: Addr,
    },
}

// We define a custom struct for each query response
// #[cw_serde]
// pub struct YourQueryResponse {}
