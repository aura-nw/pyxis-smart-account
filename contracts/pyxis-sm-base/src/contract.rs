use std::vec;

use cosmos_sdk_proto::traits::{Message, Name};
use cosmwasm_std::{to_json_binary, CosmosMsg, Order, StdError};

use cosmos_sdk_proto::cosmwasm::wasm::v1::MsgExecuteContract;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    wasm_execute, Addr, Binary, ContractInfoResponse, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, Reply, Response, StdResult, WasmQuery,
};
use cw2::set_contract_version;
use cw_storage_plus::Bound;
use serde_json_wasm::de::Error;

use crate::error::ContractError;
use crate::msg::{AllPluginsResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{Config, Plugin, PluginStatus, CONFIG, PLUGINS};

use pyxis_sm::msg::{
    CallInfo, PyxisPluginExecuteMsg, PyxisRecoveryPluginExecuteMsg, PyxisSudoMsg, SdkMsg,
};
use pyxis_sm::plugin_manager_msg::{PluginResponse, PluginType, QueryMsg as PMQueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pyxis-sm-base";
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

    CONFIG.save(
        deps.storage,
        &Config {
            plugin_manager_addr: msg.plugin_manager_addr,
            recoverable: false,
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

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
    // check if this is called by this contract itself
    if info.sender != env.contract.address {
        return Err(ContractError::Std(StdError::generic_err("Unauthorized")));
    }

    match msg {
        ExecuteMsg::RegisterPlugin {
            plugin_address,
            config,
        } => register_plugin(deps, env, info, plugin_address, config),
        ExecuteMsg::UnregisterPlugin { plugin_address } => {
            unregister_plugin(deps, env, info, plugin_address)
        }
        ExecuteMsg::UpdatePlugin {
            plugin_address,
            status,
        } => update_plugin(deps, env, info, plugin_address, status),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: PyxisSudoMsg) -> Result<Response, ContractError> {
    match msg {
        PyxisSudoMsg::PreExecute {
            msgs,
            call_info,
            is_authz,
        } => pre_execute(deps, env, msgs, call_info, is_authz),
        PyxisSudoMsg::AfterExecute {
            msgs,
            call_info,
            is_authz,
        } => after_execute(deps, env, msgs, call_info, is_authz),
        PyxisSudoMsg::Recover {
            caller,
            pub_key,
            credentials,
        } => handle_recover(deps, env, caller, pub_key, credentials),
    }
}

/// pre_execute is called for every message before it is executed
/// it will call the pre_execute message of all the plugins except the recovery plugin
/// if any of the plugin returns an error, the whole transaction will be rejected
pub fn pre_execute(
    deps: DepsMut,
    env: Env,
    msgs: Vec<SdkMsg>,
    call_info: CallInfo,
    is_authz: bool,
) -> Result<Response, ContractError> {
    // if tx contains UnregisterPlugin or UpdatePlugin messages
    // make sure those plugins are not called at this time
    let mut disable_plugins: Vec<Addr> = Vec::new();
    for msg in &msgs {
        if msg.type_url != format!("/{}", MsgExecuteContract::full_name()) {
            continue;
        }

        let msg_exec = MsgExecuteContract::decode(msg.value.as_slice()).unwrap();
        if msg_exec.contract == env.contract.address.to_string() {
            // execute call to this smart-account contract must be
            // UnregisterPlugin, RegisterPlugin or UpdatePlugin
            // only smart-account owner can execute those msgs
            // error will be thrown at `after_execute` handler
            // if is_authz {
            //    return Err(ContractError::Std(StdError::generic_err("Unauthorization")));
            // }

            let msg_raw: Result<ExecuteMsg, Error> =
                serde_json_wasm::from_slice(msg_exec.msg.as_slice());
            if msg_raw.is_err() {
                // should never return err in `pre_execute`
                // if not a message of type `ExecuteMsg` return
                // in this situation, there will be no need to log error here as it will eventually fail when executing tx
                return Ok(Response::new().add_attribute("action", "pre_execute"));
            }
            let msg = msg_raw.unwrap();
            match msg {
                ExecuteMsg::UnregisterPlugin { plugin_address } => {
                    disable_plugins.push(plugin_address);
                }
                ExecuteMsg::UpdatePlugin {
                    plugin_address,
                    status: _,
                } => {
                    disable_plugins.push(plugin_address);
                }
                _ => {}
            }
        }
    }

    // call the pre_execute message of all the plugins
    let pre_execute_msgs = PLUGINS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|data| data.unwrap())
        .filter(|(_, plugin)| {
            plugin.status == PluginStatus::Active
                && plugin.plugin_type != PluginType::Recovery
                && !disable_plugins
                    .iter()
                    .any(|addr| addr == &plugin.contract_address)
        })
        .map(|(_, plugin)| {
            CosmosMsg::Wasm(
                wasm_execute(
                    &plugin.contract_address,
                    &PyxisPluginExecuteMsg::PreExecute {
                        msgs: msgs.clone(),
                        call_info: call_info.clone(),
                        is_authz,
                    },
                    vec![],
                )
                .unwrap(),
            )
        })
        .collect::<Vec<CosmosMsg>>();

    Ok(Response::new()
        .add_attribute("action", "pre_execute")
        .add_messages(pre_execute_msgs))
}

/// after_execute is called for every message after it is executed
/// it will call the after_execute message of all the plugins except the recovery plugin
/// if any of the plugin returns an error, the whole transaction will be rejected
pub fn after_execute(
    deps: DepsMut,
    env: Env,
    msgs: Vec<SdkMsg>,
    call_info: CallInfo,
    is_authz: bool,
) -> Result<Response, ContractError> {
    // if tx contains RegisterPlugin, UnregisterPlugin or UpdatePlugin messages
    // make sure those plugins are not called at this time
    let mut disable_plugins: Vec<Addr> = Vec::new();
    for msg in &msgs {
        if msg.type_url != format!("/{}", MsgExecuteContract::full_name()) {
            continue;
        }

        let msg_exec = MsgExecuteContract::decode(msg.value.as_slice()).unwrap();
        let msg_contract_addr = Addr::unchecked(msg_exec.contract);
        if let Some(plugin) = PLUGINS.may_load(deps.storage, &msg_contract_addr)? {
            // do not allow smart account to directly execute `PyxisPlugin Execute` messages
            // those messages are only called on behalf of the account through the smart contract
            match plugin.plugin_type {
                PluginType::Recovery => {
                    let msg: Result<PyxisRecoveryPluginExecuteMsg, _> =
                        serde_json_wasm::from_slice(msg_exec.msg.as_slice());
                    if msg.is_ok() {
                        return Err(ContractError::Std(StdError::generic_err(
                            "Not allowed action",
                        )));
                    }
                }
                PluginType::Other => {
                    let msg: Result<PyxisPluginExecuteMsg, _> =
                        serde_json_wasm::from_slice(msg_exec.msg.as_slice());
                    if msg.is_ok() {
                        return Err(ContractError::Std(StdError::generic_err(
                            "Not allowed action",
                        )));
                    }
                }
            }
        }

        if msg_contract_addr == env.contract.address {
            // execute call to this smart-account contract must be
            // UnregisterPlugin, RegisterPlugin or UpdatePlugin
            // only smart-account owner can execute those msgs
            if is_authz {
                return Err(ContractError::Std(StdError::generic_err("Unauthorization")));
            }

            let msg: ExecuteMsg = serde_json_wasm::from_slice(msg_exec.msg.as_slice()).unwrap();
            match msg {
                ExecuteMsg::RegisterPlugin {
                    plugin_address,
                    config: _,
                } => {
                    disable_plugins.push(plugin_address);
                }
                ExecuteMsg::UnregisterPlugin { plugin_address } => {
                    disable_plugins.push(plugin_address);
                }
                ExecuteMsg::UpdatePlugin {
                    plugin_address,
                    status: _,
                } => {
                    disable_plugins.push(plugin_address);
                }
            }
        }
    }

    // call the pre_execute message of all the plugins
    let after_execute_msgs = PLUGINS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|data| data.unwrap())
        .filter(|(_, plugin)| {
            plugin.status == PluginStatus::Active
                && plugin.plugin_type != PluginType::Recovery
                && !disable_plugins
                    .iter()
                    .any(|addr| addr == &plugin.contract_address)
        })
        .map(|(_, plugin)| {
            CosmosMsg::Wasm(
                wasm_execute(
                    &plugin.contract_address,
                    &PyxisPluginExecuteMsg::AfterExecute {
                        msgs: msgs.clone(),
                        call_info: call_info.clone(),
                        is_authz,
                    },
                    vec![],
                )
                .unwrap(),
            )
        })
        .collect::<Vec<CosmosMsg>>();

    Ok(Response::new()
        .add_attribute("action", "after_execute")
        .add_messages(after_execute_msgs))
}

/// handle_recover is called when a smart account is recovered (change owner)
/// it will call the recover message of the recovery plugin
/// if the recovery plugin returns an error, the whole transaction will be rejected
/// if there is no recovery plugin, the transaction will be rejected
pub fn handle_recover(
    deps: DepsMut,
    _env: Env,
    caller: String,
    pub_key: Binary,
    credentials: Binary,
) -> Result<Response, ContractError> {
    // recover is only enabled after a recovery plugin is registered
    // we also limit the recovery plugin to only one
    // load config to check if recoverable is enabled
    let config = CONFIG.load(deps.storage)?;
    if !config.recoverable {
        return Err(ContractError::Std(StdError::generic_err(
            "Recovery is not enabled",
        )));
    }

    // we will loop through every plugin to find the recovery plugin
    // and pass the recover message to it
    let recover_msgs = PLUGINS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|data| data.unwrap())
        .filter(|(_, plugin)| {
            plugin.status == PluginStatus::Active && plugin.plugin_type == PluginType::Recovery
        })
        .map(|(_, plugin)| {
            CosmosMsg::Wasm(
                wasm_execute(
                    &plugin.contract_address,
                    &PyxisRecoveryPluginExecuteMsg::Recover {
                        caller: caller.clone(),
                        pub_key: pub_key.clone(),
                        credentials: credentials.clone(),
                    },
                    vec![],
                )
                .unwrap(),
            )
        })
        .collect::<Vec<CosmosMsg>>();

    assert!(
        recover_msgs.len() == 1,
        "There should be only one recovery plugin"
    );

    Ok(Response::new()
        .add_attribute("action", "recover")
        .add_messages(recover_msgs))
}

/// Register a plugin to this smart account
/// Only this smart account can register a plugin for itself
pub fn register_plugin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    plugin_address: Addr,
    config: String,
) -> Result<Response, ContractError> {
    // check if this plugin has already been registered
    // for now we will throw error
    if PLUGINS.has(deps.storage, &plugin_address) {
        return Err(ContractError::Std(StdError::generic_err(
            "Plugin is already registered",
        )));
    }

    // call plugin manager to check if this plugin is valid
    // if the request is successful, it means the plugin is valid
    let plugin_manager_addr = CONFIG.load(deps.storage)?.plugin_manager_addr;
    let query_plugin_msg = PMQueryMsg::PluginInfo {
        address: plugin_address.to_string(),
    };
    let plugin_info: PluginResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: plugin_manager_addr.to_string(),
            msg: to_json_binary(&query_plugin_msg)?,
        }))?;

    // check if plugin is enable
    if !plugin_info.enabled {
        return Err(ContractError::Std(StdError::generic_err(
            "Plugin is disabled",
        )));
    }

    // query plugin contract infor
    let contract_info: ContractInfoResponse =
        deps.querier
            .query(&QueryRequest::Wasm(WasmQuery::ContractInfo {
                contract_addr: plugin_info.address.to_string(),
            }))?;
    // check if plugin contract not been migrated
    if contract_info.code_id != plugin_info.code_id {
        return Err(ContractError::Std(StdError::generic_err(
            "Invalid plugin code_id",
        )));
    }

    // add this plugin and its config to the storage
    PLUGINS.save(
        deps.storage,
        &plugin_address.clone(),
        &Plugin {
            name: plugin_info.name,
            plugin_type: plugin_info.plugin_type.clone(),
            contract_address: plugin_address.clone(),
            status: PluginStatus::Active,
            config: config.clone(),
        },
    )?;

    // TODO: may allow multiple recovery plugins in the future
    // if plugin type is recovery and recoverable is false, enable recoverable
    // otherwise, throw an error
    if plugin_info.plugin_type == PluginType::Recovery {
        let mut config = CONFIG.load(deps.storage)?;
        if !config.recoverable {
            config.recoverable = true;
            CONFIG.save(deps.storage, &config)?;
        } else {
            return Err(ContractError::Std(StdError::generic_err(
                "Recovery plugin is already registered",
            )));
        }
    }

    let register_msg = CosmosMsg::Wasm(wasm_execute(
        plugin_address.as_str(),
        &PyxisPluginExecuteMsg::Register { config },
        info.funds,
    )?);

    Ok(Response::new()
        .add_attribute("action", "register")
        .add_message(register_msg))
}

/// Unregister a plugin from this smart account
/// Only this smart account can unregister a plugin of itself
pub fn unregister_plugin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    plugin_address: Addr,
) -> Result<Response, ContractError> {
    let plugin = PLUGINS.load(deps.storage, &plugin_address)?;

    // if the plugin is a recovery plugin, disable recoverable
    if plugin.plugin_type == PluginType::Recovery {
        let mut config = CONFIG.load(deps.storage)?;
        if config.recoverable {
            config.recoverable = false;
            CONFIG.save(deps.storage, &config)?;
        }
    }

    PLUGINS.remove(deps.storage, &plugin_address);

    // call plugin manager to check if this plugin is enabled
    let plugin_manager_addr = CONFIG.load(deps.storage)?.plugin_manager_addr;
    let query_plugin_msg = PMQueryMsg::PluginInfo {
        address: plugin_address.to_string(),
    };
    let plugin_info: Result<PluginResponse, StdError> =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: plugin_manager_addr.to_string(),
            msg: to_json_binary(&query_plugin_msg)?,
        }));

    // if query error or plugin is diabled, just return
    // else call unregister message
    if plugin_info.is_err() || !plugin_info.unwrap().enabled {
        return Ok(Response::new().add_attribute("action", "unregister_plugin"));
    } else {
        // call unregister in the plugin contract
        let unregister_msg = CosmosMsg::Wasm(wasm_execute(
            plugin_address.as_str(),
            &PyxisPluginExecuteMsg::Unregister {},
            info.funds,
        )?);

        return Ok(Response::new()
            .add_attribute("action", "unregister_plugin")
            .add_message(unregister_msg));
    }
}

fn update_plugin(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    plugin_address: Addr,
    status: PluginStatus,
) -> Result<Response, ContractError> {
    let mut plugin = PLUGINS.load(deps.storage, &plugin_address)?;

    assert!(plugin.status != status, "Plugin status not change");

    match status {
        PluginStatus::Inactive => {
            // call plugin manager to check if this plugin is enabled
            let plugin_manager_addr = CONFIG.load(deps.storage)?.plugin_manager_addr;
            let query_plugin_msg = PMQueryMsg::PluginInfo {
                address: plugin_address.to_string(),
            };
            let plugin_info: Result<PluginResponse, StdError> =
                deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr: plugin_manager_addr.to_string(),
                    msg: to_json_binary(&query_plugin_msg)?,
                }));

            if plugin_info.is_ok() && plugin_info.unwrap().enabled {
                return Err(ContractError::Std(StdError::generic_err(
                    "Plugin is enabled, cannot deactivate",
                )));
            }
        }
        _ => {}
    }

    plugin.status = status;
    PLUGINS.save(deps.storage, &plugin_address, &plugin)?;

    Ok(Response::new().add_attribute("action", "update_plugin"))
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllPlugins { start_after, limit } => {
            let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
            let start = start_after.map(|s| Bound::ExclusiveRaw(s.into_bytes()));

            let plugins = PLUGINS
                .range(deps.storage, start, None, Order::Ascending)
                .take(limit)
                .map(|item| item.map(|(_, plugin)| plugin))
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
    Ok(Response::new())
}
