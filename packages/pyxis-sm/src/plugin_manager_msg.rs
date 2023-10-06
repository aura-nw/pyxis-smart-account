/// Messages and related struct definitions for the plugin manager.
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PluginResponse)]
    PluginInfo { address: String },
}

#[cw_serde]
pub enum PluginType {
    Recovery,
    Other,
}

#[cw_serde]
pub struct PluginResponse {
    pub name: String,
    pub plugin_type: PluginType,
    pub version: String,
    pub address: String,
}
