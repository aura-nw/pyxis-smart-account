use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin};

#[cw_serde]
pub struct CallInfo {
    pub fee: Vec<Coin>,
    pub gas: u64,
    pub fee_payer: String,
    pub fee_granter: String,
}

impl Default for CallInfo {
    fn default() -> Self {
        CallInfo {
            fee: vec![],
            gas: 0,
            fee_payer: "".to_string(),
            fee_granter: "".to_string(),
        }
    }
}

#[cw_serde]
pub struct SdkMsg {
    pub type_url: String,
    pub value: Binary,
}

#[cw_serde]
pub struct AuthzInfo {
    pub grantee: String,
}

impl AuthzInfo {
    pub fn is_authz(&self) -> bool {
        self.grantee != String::default()
    }
}

#[cw_serde]
pub enum PyxisSudoMsg {
    // pre_execute is a base message which is called before any other message
    PreExecute {
        msgs: Vec<SdkMsg>,
        call_info: CallInfo,
        authz_info: AuthzInfo,
    },
    // after_execute is a base message which is called after any other message
    AfterExecute {
        msgs: Vec<SdkMsg>,
        call_info: CallInfo,
        authz_info: AuthzInfo,
    },
    // recover is a base message which is called when a smart account is recovered (change owner)
    Recover {
        caller: String,
        pub_key: Binary,
        credentials: Binary,
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
        authz_info: AuthzInfo,
    },
    /// AfterExecute is called at the end of a transaction
    AfterExecute {
        msgs: Vec<SdkMsg>,
        call_info: CallInfo,
        authz_info: AuthzInfo,
    },
}

#[cw_serde]
pub enum PyxisRecoveryPluginExecuteMsg {
    Register {
        config: String,
    },
    Unregister {},
    Recover {
        caller: String,
        pub_key: Binary,
        credentials: Binary,
    },
}
