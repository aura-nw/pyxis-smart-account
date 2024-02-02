/// Messages and related struct definitions for the plugin manager.
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PluginResponse)]
    PluginInfo { address: String },

    #[returns(AllPluginsResponse)]
    AllPlugins {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    #[returns(QueryPluginsStatusResponse)]
    PluginsStatus { addresses: Vec<String> },
}

#[cw_serde]
pub enum PluginType {
    Recovery,
    Other,
}

#[cw_serde]
pub enum PluginStatus {
    Active,
    Inactive,
}

#[cw_serde]
pub enum UnregisterRequirement {
    Required,
    NotRequired,
}

#[cw_serde]
pub struct PluginResponse {
    pub name: String,
    pub plugin_type: PluginType,
    pub version: String,
    pub address: String,
    pub code_id: u64,
    pub status: PluginStatus,
    pub unregister_req: UnregisterRequirement,
}

#[cw_serde]
pub struct AllPluginsResponse {
    pub plugins: Vec<PluginResponse>,
}

#[cw_serde]
pub struct QueryPluginStatus {
    pub address: String,
    pub status: PluginStatus,
}

#[cw_serde]
pub struct QueryPluginsStatusResponse {
    pub plugins_status: Vec<QueryPluginStatus>,
}
