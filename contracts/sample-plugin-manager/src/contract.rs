#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, ContractInfoResponse, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, Reply, Response, StdError, StdResult, WasmMsg, WasmQuery, Order,
};
use cw2::set_contract_version;
use cw_ownable::{assert_owner, update_ownership};
use cw_storage_plus::Bound;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg};
use crate::state::{Plugin, PLUGINS};
use pyxis_sm::plugin_manager_msg::{AllPluginsResponse, PluginResponse, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:sample-plugin-manager";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// settings for query pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    cw_ownable::initialize_owner(deps.storage, deps.api, Some(&msg.owner))?;

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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AllowPlugin { plugin_info } => {
            // check onwership
            assert_owner(deps.storage, &info.sender).map_err(|_| ContractError::Unauthorized {})?;

            // check if this plugin has already been allowed
            // for now we will throw error
            if PLUGINS.has(deps.storage, &plugin_info.address.to_string()) {
                return Err(ContractError::Std(StdError::generic_err(
                    "Plugin is already allowed",
                )));
            }

            validate_plugin(deps.as_ref(), env, &plugin_info)?;

            // just save it
            PLUGINS.save(deps.storage, &plugin_info.address.to_string(), &plugin_info)?;
            Ok(Response::new().add_attributes(vec![
                ("action", "allow_plugin"),
                ("plugin_address", plugin_info.address.to_string().as_str()),
            ]))
        }
        ExecuteMsg::DisallowPlugin { plugin_address } => {
            assert_owner(deps.storage, &info.sender).map_err(|_| ContractError::Unauthorized {})?;

            PLUGINS.remove(deps.storage, &plugin_address.to_string());
            Ok(Response::new().add_attributes(vec![
                ("action", "disallow_plugin"),
                ("plugin_address", plugin_address.to_string().as_str()),
            ]))
        }
        ExecuteMsg::UpdatePlugin { plugin_info } => {
            assert_owner(deps.storage, &info.sender).map_err(|_| ContractError::Unauthorized {})?;

            if !PLUGINS.has(deps.storage, &plugin_info.address.to_string()) {
                return Err(ContractError::Std(StdError::generic_err(
                    "Plugin not found",
                )));
            }

            validate_plugin(deps.as_ref(), env, &plugin_info)?;

            // just save it
            PLUGINS.save(deps.storage, &plugin_info.address.to_string(), &plugin_info)?;
            Ok(Response::new().add_attribute("action", "update_plugin"))
        }
        ExecuteMsg::MigratePlugin {
            plugin_address,
            new_code_id,
            msg,
        } => {
            assert_owner(deps.storage, &info.sender).map_err(|_| ContractError::Unauthorized {})?;

            let mut plugin = PLUGINS
                .load(deps.storage, &plugin_address)
                .map_err(|_| ContractError::Std(StdError::generic_err("Plugin not found")))?;

            // set new code_id
            plugin.code_id = new_code_id;

            PLUGINS.save(deps.storage, &plugin_address, &plugin)?;
            Ok(Response::new()
                .add_attribute("action", "migrate_plugin")
                .add_message(CosmosMsg::Wasm(WasmMsg::Migrate {
                    contract_addr: plugin_address.clone(),
                    new_code_id,
                    msg: Binary::from(msg.as_bytes()),
                })))
        }
        ExecuteMsg::UpdateOwnership(action) => {
            update_ownership(deps, &env.block, &info.sender, action)
                .map_err(|_| ContractError::Std(StdError::generic_err("Update ownership fail")))?;

            Ok(Response::new().add_attribute("action", "update_ownership"))
        }
    }
}

// validate plugin info
// prevent front-run attack
fn validate_plugin(deps: Deps, env: Env, plugin_info: &Plugin) -> StdResult<()> {
    // query plugin contract infor
    let contract_info: ContractInfoResponse =
        deps.querier
            .query(&QueryRequest::Wasm(WasmQuery::ContractInfo {
                contract_addr: plugin_info.address.to_string(),
            }))?;

    if contract_info.code_id != plugin_info.code_id {
        return Err(StdError::generic_err("Invalid plugin code_id"));
    }

    // require plugin-manager as plugin admin
    if contract_info.admin.unwrap_or(String::default()) != env.contract.address {
        return Err(StdError::generic_err("Invalid plugin admin"));
    }

    Ok(())
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::PluginInfo { address } => {
            println!("address: {}", address);
            let plugin = PLUGINS.load(deps.storage, &address)?;
            to_json_binary(&PluginResponse::from(plugin.into()))
        }
        QueryMsg::AllPlugins { start_after, limit } => {
            let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
            let start = start_after.map(|s| Bound::ExclusiveRaw(s.into_bytes()));

            let plugins = PLUGINS
                .range(deps.storage, start, None, Order::Ascending)
                .take(limit)
                .map(|item| {
                    item.map(|(_, plugin)| plugin.into())
                })
                .collect::<StdResult<_>>()?;

            to_json_binary(&AllPluginsResponse { plugins })
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
