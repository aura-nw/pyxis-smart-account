use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct CallInfo {
    pub fee: u128,
    pub gas_price: u128,
    pub gas_limit: u128,
    pub fee_payer: String,
    pub fee_granter: String,
}

impl Default for CallInfo {
    fn default() -> Self {
        CallInfo {
            fee: 0,
            gas_price: 0,
            gas_limit: 0,
            fee_payer: "".to_string(),
            fee_granter: "".to_string(),
        }
    }
}

#[cw_serde]
pub struct SdkMsg {
    pub type_url: String,
    pub value: String,
}

#[cw_serde]
pub enum PyxisExecuteMsg {
    // pre_execute is a base message which is called before any other message
    PreExecute {
        msgs: Vec<SdkMsg>,
        call_info: CallInfo,
    },
    // after_execute is a base message which is called after any other message
    AfterExecute {
        msgs: Vec<SdkMsg>,
        call_info: CallInfo,
    },
}

#[cw_serde]
pub enum PyxisPluginExecuteMsg {
    /// Register a plugin to this smart account, the caller must be the smart account itself
    Register { config: String },
    /// Unregister a plugin from this smart account, the caller must be the smart account itself
    Unregister {},
    /// PreExecute is called before a transaction is executed
    PreExecute {
        msgs: Vec<SdkMsg>,
        call_info: CallInfo,
    },
    /// AfterExecute is called at the end of a transaction
    AfterExecute {
        msgs: Vec<SdkMsg>,
        call_info: CallInfo,
    },
}
