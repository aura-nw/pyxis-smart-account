#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{UserConfig, USER_CONFIGS};

use pyxis_sm::msg::{CallInfo, PyxisPluginExecuteMsg, SdkMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:sample-plugin";
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
    msg: PyxisPluginExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        PyxisPluginExecuteMsg::Register { config } => handle_register(deps, env, info, config),
        PyxisPluginExecuteMsg::Unregister {} => handle_unregister(deps, env, info),
        PyxisPluginExecuteMsg::PreExecute { msgs, call_info } => {
            handle_pre_execute(deps, env, info, msgs, call_info)
        }
        PyxisPluginExecuteMsg::AfterExecute { .. } => Ok(Response::new()),
    }
}

// TODO: Implement your custom logic here
/// Handling register message
/// This is just a sample implementation.
pub fn handle_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    config: String,
) -> Result<Response, ContractError> {
    // we will check if the address is already registered
    // if it is, we will update the config
    // if not, we will register the address and config
    USER_CONFIGS.update(
        deps.storage,
        &info.sender,
        |user_config: Option<UserConfig>| -> StdResult<_> {
            match user_config {
                Some(mut user_config) => {
                    user_config.config = config;
                    Ok(user_config)
                }
                None => Ok(UserConfig {
                    address: info.sender.clone(),
                    config: config,
                }),
            }
        },
    )?;

    Ok(Response::default())
}

// TODO: Implement your custom logic here
/// Handling unregister message
/// This is just a sample implementation.
pub fn handle_unregister(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn handle_pre_execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msgs: Vec<SdkMsg>,
    _call_info: CallInfo,
) -> Result<Response, ContractError> {
    // load config of sender
    let user_config = USER_CONFIGS.load(deps.storage, &info.sender).unwrap();

    match user_config.config.as_str() {
        "approve" => Ok(Response::new()),
        "reject" => Err(ContractError::Rejected {
            reason: "reject".to_string(),
        }),
        _ => Err(ContractError::Rejected {
            reason: "config error".to_string(),
        }),
    }
}

pub fn handle_after_execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msgs: Vec<SdkMsg>,
    _call_info: CallInfo,
) -> Result<Response, ContractError> {
    // load config of sender
    let user_config = USER_CONFIGS.load(deps.storage, &info.sender).unwrap();

    match user_config.config.as_str() {
        "approve" => Ok(Response::new()),
        "reject" => Err(ContractError::Rejected {
            reason: "reject".to_string(),
        }),
        _ => Err(ContractError::Rejected {
            reason: "config error".to_string(),
        }),
    }
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
