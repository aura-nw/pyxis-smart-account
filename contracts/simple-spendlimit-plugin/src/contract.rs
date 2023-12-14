use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::msg::{InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{Limit, LIMITS};
use crate::tracked_msgs::get_transfer_balances;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
    Storage, Uint128, Uint64,
};
use cw2::set_contract_version;
use pyxis_sm::msg::{CallInfo, SdkMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:simple-spendlimit-plugin";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const MAX_SEC_PERIODIC: u64 = 315400000u64; // 10 years

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
        ExecuteMsg::PreExecute {
            msgs,
            call_info,
            is_authz,
        } => handle_pre_execute(deps, env, info, msgs, call_info, is_authz),
        ExecuteMsg::AfterExecute {
            msgs,
            call_info,
            is_authz,
        } => handle_after_execute(deps, env, info, msgs, call_info, is_authz),
        ExecuteMsg::Register { config } => handle_register(deps, env, info, config),
        ExecuteMsg::Unregister {} => handle_unregister(deps, env, info),

        ExecuteMsg::AddLimit { limit } => handle_add_limit(deps, env, info, limit),
        ExecuteMsg::UpdateLimit { index, limit } => {
            handle_update_limit(deps, env, info, index, limit)
        }
        ExecuteMsg::DeleteLimit { index } => handle_delete_limit(deps, env, info, index),
    }
}

fn handle_pre_execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msgs: Vec<SdkMsg>,
    _call_info: CallInfo,
    _is_authz: bool,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "pre_execute"))
}

fn handle_after_execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msgs: Vec<SdkMsg>,
    _call_info: CallInfo,
    _is_authz: bool,
) -> Result<Response, ContractError> {
    // load sender's limits
    let mut limits = load_limits(deps.storage, &info.sender)?;

    // get transfer balances of transaction
    let transfer_balances = get_transfer_balances(msgs)?;

    if transfer_balances.len() == 0 {
        return Ok(Response::new());
    }

    for limit in limits.iter_mut() {
        match limit {
            Limit::PerTransaction(l) => {
                if let Some(item) = transfer_balances
                    .iter()
                    .find(|coin| coin.denom.eq(&l.limit.denom))
                {
                    // transfer amount bigger than limit
                    if item.amount > l.limit.amount {
                        return Err(ContractError::ReachTransactionSpendLimit {
                            denom: l.limit.denom.clone(),
                            limit: l.limit.amount,
                            spent_amount: item.amount,
                        });
                    }
                }
            }
            Limit::Periodic(ref mut l) => {
                let block_time = Uint64::from(env.block.time.seconds());

                if block_time > l.end_period() {
                    // if new period, reset used coins
                    l.used = Uint128::zero();

                    // set to new begin period
                    l.begin_period = block_time
                        - (block_time - l.begin_period)
                            .checked_rem(l.periodic)
                            .unwrap()
                }

                if let Some(item) = transfer_balances
                    .iter()
                    .find(|coin| coin.denom.eq(&l.limit.denom))
                {
                    let spent_amount = item.amount.checked_add(l.used).unwrap();

                    // transfer amount bigger than period limit
                    if spent_amount > l.limit.amount {
                        return Err(ContractError::ReachPeriodicSpendLimit {
                            denom: l.limit.denom.clone(),
                            limit: l.limit.amount,
                            begin_period: l.begin_period,
                            periodic: l.periodic,
                            spent_amount,
                        });
                    }

                    l.used = spent_amount;
                }
            }
        }
    }

    // update limits
    LIMITS.save(deps.storage, &info.sender, &limits)?;

    Ok(Response::new().add_attribute("action", "after_execute"))
}

fn handle_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _config: String,
) -> Result<Response, ContractError> {
    if let Some(_limits) = LIMITS.may_load(deps.storage, &info.sender)? {
        return Err(ContractError::AccountAlreadyRegister {});
    } else {
        LIMITS.save(deps.storage, &info.sender, &Vec::new())?;
    }

    Ok(Response::new()
        .add_attribute("action", "register")
        .add_attribute("account", info.sender.to_string()))
}

fn handle_unregister(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    LIMITS.remove(deps.storage, &info.sender);

    Ok(Response::new()
        .add_attribute("action", "unregister")
        .add_attribute("account", info.sender))
}

fn handle_add_limit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    limit: Limit,
) -> Result<Response, ContractError> {
    validate_limit(&limit, env)?;

    let mut limits = load_limits(deps.storage, &info.sender)?;
    limits.push(limit);
    LIMITS.save(deps.storage, &info.sender, &limits)?;

    Ok(Response::new().add_attribute("action", "add_limit"))
}

fn handle_update_limit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    index: u32,
    limit: Limit,
) -> Result<Response, ContractError> {
    validate_limit(&limit, env)?;

    let mut limits = load_limits(deps.storage, &info.sender)?;

    if index as usize >= limits.len() {
        return Err(ContractError::OutOfRange {});
    }

    limits[index as usize] = limit;

    LIMITS.save(deps.storage, &info.sender, &limits)?;

    Ok(Response::new()
        .add_attribute("action", "edit_limit")
        .add_attribute("index", index.to_string()))
}

fn handle_delete_limit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    index: u32,
) -> Result<Response, ContractError> {
    let mut limits = load_limits(deps.storage, &info.sender)?;

    if index as usize >= limits.len() {
        return Err(ContractError::OutOfRange {});
    }

    limits.remove(index as usize);
    LIMITS.save(deps.storage, &info.sender, &limits)?;

    Ok(Response::new()
        .add_attribute("action", "delete_limit")
        .add_attribute("index", index.to_string()))
}

fn load_limits(storage: &dyn Storage, sender: &Addr) -> Result<Vec<Limit>, ContractError> {
    let limits = LIMITS.may_load(storage, sender)?;
    if limits.is_none() {
        return Err(ContractError::AccountNotRegistered {});
    }

    Ok(limits.unwrap())
}

fn validate_limit(limit: &Limit, env: Env) -> Result<(), ContractError> {
    match limit {
        Limit::PerTransaction(_) => {}
        Limit::Periodic(l) => {
            if l.periodic.u64() == 0u64 {
                return Err(ContractError::CustomError {
                    val: String::from("zero time period"),
                });
            }

            if l.periodic.u64() > MAX_SEC_PERIODIC {
                return Err(ContractError::CustomError {
                    val: String::from("period too large"),
                });
            }

            if l.begin_period.u64() > env.block.time.seconds() + MAX_SEC_PERIODIC {
                return Err(ContractError::CustomError {
                    val: String::from("begin period is too far"),
                });
            }
        }
    }

    Ok(())
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetLimits { address } => to_json_binary(&get_limits(deps, address)?),
    }
}

fn get_limits(deps: Deps, address: Addr) -> StdResult<Option<Vec<Limit>>> {
    Ok(LIMITS.may_load(deps.storage, &address)?)
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    // With `Response` type, it is still possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages

    todo!()
}
