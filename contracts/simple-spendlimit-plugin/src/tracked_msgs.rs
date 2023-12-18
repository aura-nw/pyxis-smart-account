use cosmos_sdk_proto::cosmos::bank::v1beta1::{MsgMultiSend, MsgSend};
use cosmos_sdk_proto::cosmos::distribution::v1beta1::MsgFundCommunityPool;
use cosmos_sdk_proto::cosmos::gov::v1::{
    MsgDeposit as MsgDepositV1, MsgSubmitProposal as MsgSubmitProposalV1,
};
use cosmos_sdk_proto::cosmos::gov::v1beta1::{MsgDeposit, MsgSubmitProposal};
use cosmos_sdk_proto::cosmos::staking::v1beta1::MsgDelegate;
use cosmos_sdk_proto::cosmos::vesting::v1beta1::{
    MsgCreatePeriodicVestingAccount, MsgCreatePermanentLockedAccount, MsgCreateVestingAccount,
};
use cosmos_sdk_proto::cosmwasm::wasm::v1::{
    MsgExecuteContract, MsgInstantiateContract, MsgInstantiateContract2,
};
use cosmos_sdk_proto::ibc::applications::transfer::v1::MsgTransfer;
use cosmos_sdk_proto::traits::Message;
use cosmwasm_std::{Coin, StdResult, Uint128};
use pyxis_sm::msg::SdkMsg;

// bank
const BANK_SEND: &'static str = "/cosmos.bank.v1beta1.MsgSend";
const BANK_MULTI_SEND: &'static str = "/cosmos.bank.v1beta1.MsgMultiSend";
// wasm
const WASM_EXECUTE: &'static str = "/cosmwasm.wasm.v1.MsgExecuteContract";
const WASM_INSTANTIATE: &'static str = "/cosmwasm.wasm.v1.MsgInstantiateContract";
const WASM_INSTANTIATE2: &'static str = "/cosmwasm.wasm.v1.MsgInstantiateContract2";
// gov
const GOV_SUBMIT_PROPOSAL: &'static str = "/cosmos.gov.v1beta1.MsgSubmitProposal";
const GOV_DEPOSIT_PROPOSAL: &'static str = "/cosmos.gov.v1beta1.MsgDeposit";
const GOV_SUBMIT_PROPOSAL_V1: &'static str = "/cosmos.gov.v1.MsgSubmitProposal";
const GOV_DEPOSIT_PROPOSAL_V1: &'static str = "/cosmos.gov.v1.MsgDeposit";
// dist
const DIST_FUND_COMMUNITY_POOL: &'static str = "/cosmos.distribution.v1beta1.MsgFundCommunityPool";
// stake
const STAKE_DELEGATE: &'static str = "/cosmos.staking.v1beta1.MsgDelegate";
// vesting
const VESTING_CREATE_VESTING_ACCOUNT: &str = "/cosmos.vesting.v1beta1.MsgCreateVestingAccount";
const VESTING_CREATE_PERMANENT_LOCKED_ACCOUNT: &str =
    "/cosmos.vesting.v1beta1.MsgCreatePermanentLockedAccount";
const VESTING_CREATE_PERIODIC_VESTING_ACCOUNT: &str =
    "/cosmos.vesting.v1beta1.MsgCreatePeriodicVestingAccount";
// ibc
const IBC_TRANSFER: &'static str = "/ibc.applications.transfer.v1.MsgTransfer";

/// get transfer balances of transaction
///
/// tracking messages supported
///
/// **bank**    MsgSend | MsgMultiSend (v1beta1)
///
/// **wasm**    MsgExecuteContract | MsgInstantiateContract | MsgInstantiateContract2 (v1)
///
/// **gov**     MsgSubmitProposal | MsgDeposit (v1beta1, v1)
///
/// **dist**    MsgFundCommunityPool (v1beta1)
///
/// **stake**   MsgDelegate (v1beta1)
///
/// **vesting** MsgCreateVestingAccount | MsgCreatePermanentLockedAccount | MsgCreatePeriodicVestingAccount (v1beta1)
///
/// **ibc**     MsgTransfer (v1)
pub fn get_transfer_balances(msgs: Vec<SdkMsg>) -> StdResult<Vec<Coin>> {
    let mut transfer_balances: Vec<Coin> = Vec::new();
    for msg in msgs {
        match msg.type_url.as_str() {
            BANK_SEND => {
                let msg_send = MsgSend::decode(msg.value.as_slice()).unwrap();
                let amount: Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin> = msg_send.amount;

                add_balances(transfer_balances.as_mut(), amount)?;
            }
            BANK_MULTI_SEND => {
                let msg_multi_send = MsgMultiSend::decode(msg.value.as_slice()).unwrap();

                for input in msg_multi_send.inputs {
                    add_balances(transfer_balances.as_mut(), input.coins)?;
                }
            }
            WASM_EXECUTE => {
                let msg_wasm_exec = MsgExecuteContract::decode(msg.value.as_slice()).unwrap();
                let funds = msg_wasm_exec.funds;

                add_balances(transfer_balances.as_mut(), funds)?;
            }
            WASM_INSTANTIATE => {
                let msg_wasm_instantiate =
                    MsgInstantiateContract::decode(msg.value.as_slice()).unwrap();
                let funds = msg_wasm_instantiate.funds;

                add_balances(transfer_balances.as_mut(), funds)?;
            }
            WASM_INSTANTIATE2 => {
                let msg_wasm_instantiate2 =
                    MsgInstantiateContract2::decode(msg.value.as_slice()).unwrap();
                let funds = msg_wasm_instantiate2.funds;

                add_balances(transfer_balances.as_mut(), funds)?;
            }
            GOV_SUBMIT_PROPOSAL => {
                let msg_submit_proposal = MsgSubmitProposal::decode(msg.value.as_slice()).unwrap();
                let initital_deposit = msg_submit_proposal.initial_deposit;

                add_balances(transfer_balances.as_mut(), initital_deposit)?;
            }
            GOV_DEPOSIT_PROPOSAL => {
                let msg_deposit_proposal = MsgDeposit::decode(msg.value.as_slice()).unwrap();
                let amount = msg_deposit_proposal.amount;

                add_balances(transfer_balances.as_mut(), amount)?;
            }
            GOV_SUBMIT_PROPOSAL_V1 => {
                let msg_submit_proposal =
                    MsgSubmitProposalV1::decode(msg.value.as_slice()).unwrap();
                let initital_deposit = msg_submit_proposal.initial_deposit;

                add_balances(transfer_balances.as_mut(), initital_deposit)?;
            }
            GOV_DEPOSIT_PROPOSAL_V1 => {
                let msg_deposit_proposal = MsgDepositV1::decode(msg.value.as_slice()).unwrap();
                let amount = msg_deposit_proposal.amount;

                add_balances(transfer_balances.as_mut(), amount)?;
            }
            DIST_FUND_COMMUNITY_POOL => {
                let msg_fund_com_pool = MsgFundCommunityPool::decode(msg.value.as_slice()).unwrap();
                let amount = msg_fund_com_pool.amount;

                add_balances(transfer_balances.as_mut(), amount)?;
            }
            STAKE_DELEGATE => {
                let msg_delegate = MsgDelegate::decode(msg.value.as_slice()).unwrap();
                let amount = msg_delegate.amount;

                if amount.is_some() {
                    add_balances(transfer_balances.as_mut(), vec![amount.unwrap()])?;
                }
            }
            VESTING_CREATE_VESTING_ACCOUNT => {
                let msg_create_vesting_acc =
                    MsgCreateVestingAccount::decode(msg.value.as_slice()).unwrap();
                let amount = msg_create_vesting_acc.amount;

                add_balances(transfer_balances.as_mut(), amount)?;
            }
            VESTING_CREATE_PERMANENT_LOCKED_ACCOUNT => {
                let msg_create_per_locked_acc =
                    MsgCreatePermanentLockedAccount::decode(msg.value.as_slice()).unwrap();
                let amount = msg_create_per_locked_acc.amount;

                add_balances(transfer_balances.as_mut(), amount)?;
            }
            VESTING_CREATE_PERIODIC_VESTING_ACCOUNT => {
                let msg_create_per_vesting_acc =
                    MsgCreatePeriodicVestingAccount::decode(msg.value.as_slice()).unwrap();

                for period in msg_create_per_vesting_acc.vesting_periods {
                    add_balances(transfer_balances.as_mut(), period.amount)?;
                }
            }
            IBC_TRANSFER => {
                let msg_ibc_transfer = MsgTransfer::decode(msg.value.as_slice()).unwrap();
                let token = msg_ibc_transfer.token;

                if token.is_some() {
                    add_balances(transfer_balances.as_mut(), vec![token.unwrap()])?;
                }
            }
            _ => {}
        }
    }

    Ok(transfer_balances)
}

fn add_balances(
    balances: &mut Vec<Coin>,
    coins: Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
) -> StdResult<()> {
    for coin in coins {
        let amount = Uint128::from(u128::from_str_radix(&coin.amount, 10).unwrap());

        if let Some(idx) = balances.iter().position(|b| b.denom.eq(&coin.denom)) {
            balances[idx].amount = balances[idx].amount.checked_add(amount).unwrap();
        } else {
            balances.push(Coin {
                denom: coin.denom,
                amount,
            });
        }
    }

    Ok(())
}
