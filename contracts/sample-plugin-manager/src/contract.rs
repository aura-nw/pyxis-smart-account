#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, Addr,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg};
use crate::state::{Plugin, PLUGINS, CONFIG, Config};
use pyxis_sm::plugin_manager_msg::{PluginResponse, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:sample-plugin-manager";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        admin: Addr::unchecked(msg.admin)
    };
    CONFIG.save(deps.storage, &config)?;

    // With `Response` type, it is possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract migration
/// To make a contract migratable, you need
/// - this entry_point implemented
/// - only contract admin can migrate, so admin has to be set at contract initiation time
/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {
        // Find matched incoming message variant and execute them with your custom logic.
        //
        // With `Response` type, it is possible to dispatch message to invoke external logic.
        // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    }
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    match msg {
        ExecuteMsg::AllowPlugin {
            plugin_address,
            plugin_type,
        } => {
            // just save it
            let plugin = Plugin {
                name: "sample plugin".to_string(),
                plugin_type,
                version: "0.1.0".to_string(),
                code_id: 1,
                address: plugin_address.clone(),
                checksum: "checksum".to_string(),
                enabled: true,
            };
            PLUGINS.save(deps.storage, &plugin_address.to_string(), &plugin)?;
            Ok(Response::new().add_attributes(vec![
                ("action", "allow_plugin"),
                ("plugin_address", plugin_address.to_string().as_str()),
            ]))
        }
        ExecuteMsg::DisallowPlugin { plugin_address } => {
            PLUGINS.remove(deps.storage, &plugin_address.to_string());
            Ok(Response::new().add_attributes(vec![
                ("action", "disallow_plugin"),
                ("plugin_address", plugin_address.to_string().as_str()),
            ]))
        }
        ExecuteMsg::UpdatePlugin { plugin_address, enabled } => {
            let mut plugin = PLUGINS.load(deps.storage, &plugin_address.to_string())?;
            plugin.enabled = enabled;
            PLUGINS.save(deps.storage, &plugin_address.to_string(), &plugin)?;

            Ok(Response::new().add_attributes(vec![
                ("action", "update_plugin"),
                ("plugin_address", &plugin_address.to_string()),
                ("enabled", &enabled.to_string())
            ]))
        }
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::PluginInfo { address } => {
            println!("address: {}", address);
            let plugin = PLUGINS.load(deps.storage, &address)?;
            to_json_binary(&PluginResponse {
                name: plugin.name,
                plugin_type: plugin.plugin_type,
                version: plugin.version,
                address: plugin.address.to_string(),
                enabled: plugin.enabled
            })
        }
    }
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    // With `Response` type, it is still possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages

    todo!()
}
