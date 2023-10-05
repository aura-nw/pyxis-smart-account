use std::vec;

use cosmwasm_std::{to_binary, CosmosMsg, StdError};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    wasm_execute, Addr, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Reply, Response,
    StdResult, WasmQuery,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{Config, Plugin, PluginStatus, CONFIG, PLUGINS};

use pyxis_sm::msg::{CallInfo, PyxisExecuteMsg, PyxisPluginExecuteMsg, SdkMsg};
use pyxis_sm::plugin_manager_msg::{PluginResponse, QueryMsg as PMQueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pyxis-sm-base";
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

    CONFIG.save(
        deps.storage,
        &Config {
            plugin_manager_addr: msg.plugin_manager_addr,
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
        ExecuteMsg::PyxisExecuteMsg(pyxis_msg) => match pyxis_msg {
            PyxisExecuteMsg::PreExecute { msgs, call_info } => {
                pre_execute(deps, env, info, msgs, call_info)
            }
            PyxisExecuteMsg::AfterExecute { msgs, call_info } => {
                after_execute(deps, env, info, msgs, call_info)
            }
        },
        ExecuteMsg::RegisterPlugin {
            plugin_address,
            checksum,
            config,
        } => register_plugin(deps, env, info, plugin_address, checksum, config),
        ExecuteMsg::UnregisterPlugin { plugin_address } => {
            unregister_plugin(deps, env, info, plugin_address)
        }
    }
}

pub fn pre_execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: Vec<SdkMsg>,
    call_info: CallInfo,
) -> Result<Response, ContractError> {
    // call the pre_execute message of all the plugins
    let pre_execute_msgs = PLUGINS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|data| data.unwrap())
        .filter(|(_, plugin)| plugin.status == PluginStatus::Active)
        .map(|(_, plugin)| {
            CosmosMsg::Wasm(
                wasm_execute(
                    &plugin.contract_address,
                    &PyxisPluginExecuteMsg::PreExecute {
                        msgs: msg.clone(),
                        call_info: call_info.clone(),
                    },
                    vec![],
                )
                .unwrap(),
            )
        })
        .collect::<Vec<CosmosMsg>>();

    Ok(Response::new().add_messages(pre_execute_msgs))
}

pub fn after_execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: Vec<SdkMsg>,
    call_info: CallInfo,
) -> Result<Response, ContractError> {
    // call the pre_execute message of all the plugins
    let after_execute_msgs = PLUGINS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|data| data.unwrap())
        .filter(|(_, plugin)| plugin.status == PluginStatus::Active)
        .map(|(_, plugin)| {
            CosmosMsg::Wasm(
                wasm_execute(
                    &plugin.contract_address,
                    &PyxisPluginExecuteMsg::PreExecute {
                        msgs: msg.clone(),
                        call_info: call_info.clone(),
                    },
                    vec![],
                )
                .unwrap(),
            )
        })
        .collect::<Vec<CosmosMsg>>();

    Ok(Response::new().add_messages(after_execute_msgs))
}

/// Register a plugin to this smart account
/// Only this smart account can register a plugin for itself
pub fn register_plugin(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    plugin_address: Addr,
    checksum: String,
    config: String,
) -> Result<Response, ContractError> {
    // TODO: check if plugin_address is a valid plugin contract with the same checksum

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
            msg: to_binary(&query_plugin_msg)?,
        }))?;

    // TODO: query the contract info of the plugin_address and check if the checksum is the same

    // add this plugin and its config to the storage
    PLUGINS.save(
        deps.storage,
        &plugin_address.clone(),
        &Plugin {
            name: plugin_info.name,
            contract_address: plugin_address.clone(),
            checksum,
            status: PluginStatus::Active,
            config: config.clone(),
        },
    )?;

    let register_msg = CosmosMsg::Wasm(wasm_execute(
        plugin_address.as_str(),
        &PyxisPluginExecuteMsg::Register { config },
        vec![],
    )?);

    Ok(Response::new().add_message(register_msg))
}

/// Unregister a plugin from this smart account
/// Only this smart account can unregister a plugin of itself
pub fn unregister_plugin(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    plugin_address: Addr,
) -> Result<Response, ContractError> {
    // just remove the plugin from the storage
    PLUGINS.remove(deps.storage, &plugin_address);

    // call the unregister message
    let unregister_msg = CosmosMsg::Wasm(wasm_execute(
        plugin_address.as_str(),
        &PyxisPluginExecuteMsg::Unregister {},
        vec![],
    )?);

    Ok(Response::new().add_message(unregister_msg))
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Find matched incoming message variant and query them your custom logic
        // and then construct your query response with the type usually defined
        // `msg.rs` alongside with the query message itself.
        //
        // use `cosmwasm_std::to_binary` to serialize query response to json binary.
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
