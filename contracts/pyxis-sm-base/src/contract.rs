use cosmwasm_std::StdError;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{Plugin, PLUGINS};

use pyxis_sm::msg::PyxisExecuteMsg;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pyxis-sm-base";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

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
    match msg {
        ExecuteMsg::PyxisExecuteMsg(pyxis_msg) => match pyxis_msg {
            PyxisExecuteMsg::PreExecute { .. } => Ok(Response::default()),
            PyxisExecuteMsg::AfterExecute { .. } => Ok(Response::default()),
        },
        ExecuteMsg::RegisterPlugin {
            plugin_address,
            checksum,
            config,
        } => register_plugin(deps, plugin_address, checksum, config),
        ExecuteMsg::UnregisterPlugin { plugin_address } => unregister_plugin(deps, plugin_address),
    }
}

pub fn pre_execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: PyxisExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn after_execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: PyxisExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}
/// Register a plugin to this smart account
/// Only this smart account can register a plugin for itself
pub fn register_plugin(
    deps: DepsMut,
    plugin_address: Addr,
    checksum: String,
    config: String,
) -> Result<Response, ContractError> {
    // TODO: check if plugin_address is a valid plugin contract

    // check if this plugin has already been registered
    // for now we will throw error
    if PLUGINS.has(deps.storage, &plugin_address) {
        return Err(ContractError::Std(StdError::generic_err(
            "Plugin is already registered",
        )));
    }

    // add this plugin and its config to the storage
    PLUGINS.save(
        deps.storage,
        &plugin_address.clone(),
        &Plugin {
            contract_address: plugin_address,
            checksum,
            status: false,
            config,
        },
    )?;

    // TODO: call plugin's register hook

    Ok(Response::default())
}

/// Unregister a plugin from this smart account
/// Only this smart account can unregister a plugin of itself
pub fn unregister_plugin(deps: DepsMut, plugin_address: Addr) -> Result<Response, ContractError> {
    // just remove the plugin from the storage
    PLUGINS.remove(deps.storage, &plugin_address);

    // TODO: call plugin's unregister hook
    Ok(Response::default())
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
