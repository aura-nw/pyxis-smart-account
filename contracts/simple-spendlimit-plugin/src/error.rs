use cosmwasm_std::{StdError, Uint128, Uint64};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Account already register")]
    AccountAlreadyRegister {},

    #[error("Account not registered")]
    AccountNotRegistered {},

    #[error("Out of range")]
    OutOfRange {},

    #[error("Reach transaction spend limit")]
    ReachTransactionSpendLimit {
        denom: String,
        limit: Uint128,
        using: Uint128,
    },

    #[error("Reach periodic spend limit")]
    ReachPeriodicSpendLimit {
        denom: String,
        limit: Uint128,
        begin_period: Uint64,
        periodic: Uint64,
        using: Uint128,
    },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
