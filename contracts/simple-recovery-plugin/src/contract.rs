#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{RecoveryConfig, CONFIG_MAP};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:simple-recovery-plugin";
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
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { config } => handle_register(deps, env, info, config),
        ExecuteMsg::Unregister {} => handle_unregister(deps, env, info),
        ExecuteMsg::Recover {
            caller,
            pub_key,
            credentials,
        } => handle_recover(deps, env, info, caller, pub_key, credentials),
    }
}

/// When registering a new address
/// - parse the config string
/// - store the parsed config in a map
fn handle_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    config: String,
) -> Result<Response, ContractError> {
    let parsed_config = RecoveryConfig::from(config);

    // sender should be the smart account address
    if info.sender != parsed_config.smart_account_address {
        return Err(ContractError::Unauthorized {});
    }

    CONFIG_MAP.save(deps.storage, &info.sender, &parsed_config)?;

    Ok(Response::new())
}

fn handle_unregister(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // we can just delete it but we will load it to make sure it is valid

    // load config of sender
    let _config = CONFIG_MAP.load(deps.storage, &info.sender)?;

    // if it can be load, it is valid
    // delete the config
    CONFIG_MAP.remove(deps.storage, &info.sender);

    Ok(Response::new())
}

fn handle_recover(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    caller: String,
    _pub_key: Binary,
    _credentials: Binary,
) -> Result<Response, ContractError> {
    // load config of sender
    let config = CONFIG_MAP.load(deps.storage, &info.sender)?;

    // check that the caller is the recover address
    if deps.api.addr_validate(&caller)? != config.recover_address {
        return Err(ContractError::Unauthorized {});
    }

    // we will ignore pubkey and credentials as we don't need them for this example
    Ok(Response::new())
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config { address } => to_json_binary(&query_config(deps, address)),
    }
}

fn query_config(deps: Deps, address: String) -> String {
    let config = CONFIG_MAP
        .may_load(deps.storage, &Addr::unchecked(address))
        .unwrap();
    config
        .and_then(|c| Some(serde_json_wasm::to_string(&c).unwrap()))
        .unwrap_or(String::default())
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    // With `Response` type, it is still possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages

    todo!()
}
