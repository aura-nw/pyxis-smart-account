#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

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
        } => register_plugin(plugin_address, checksum, config),
        ExecuteMsg::UnregisterPlugin { plugin_address } => unregister_plugin(plugin_address),
        ExecuteMsg::EnablePlugin { plugin_address } => enable_plugin(plugin_address),
        ExecuteMsg::DisablePlugin { plugin_address } => disable_plugin(plugin_address),
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
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: PyxisExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn register_plugin(
    plugin_address: Addr,
    checksum: String,
    config: String,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn unregister_plugin(plugin_address: Addr) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn enable_plugin(plugin_address: Addr) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn disable_plugin(plugin_address: Addr) -> Result<Response, ContractError> {
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
