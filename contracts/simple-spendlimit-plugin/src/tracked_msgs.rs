use cosmos_sdk_proto::traits::{TypeUrl, Message};
use cosmwasm_std::{StdResult, Coin, Uint128,};
use cosmos_sdk_proto::ibc::applications::transfer::v1::MsgTransfer;
use cosmos_sdk_proto::cosmos::bank::v1beta1::{MsgSend, MsgMultiSend};
use cosmos_sdk_proto::cosmos::distribution::v1beta1::MsgFundCommunityPool;
use cosmos_sdk_proto::cosmos::gov::v1beta1::{MsgSubmitProposal,MsgDeposit};
use cosmos_sdk_proto::cosmos::staking::v1beta1::MsgDelegate;
use cosmos_sdk_proto::cosmwasm::wasm::v1::{
    MsgExecuteContract, MsgInstantiateContract, MsgInstantiateContract2
};
use pyxis_sm::msg::SdkMsg;

const WASM_INSTANTIATE2_TYPE_URL: &str = "/cosmwasm.wasm.v1.MsgInstantiateContract2";
const GOV_SUBMIT_PROPOSAL: &str = "/cosmos.gov.v1beta1.MsgSubmitProposal";
const GOV_DEPOSIT_PROPOSAL: &str = "/cosmos.gov.v1beta1.MsgDeposit";

/// get transfer balances of transaction 
/// 
/// tracking messages supported
/// 
/// **bank**  MsgSend | MsgMultiSend
/// 
/// **wasm**  MsgExecuteContract | MsgInstantiateContract | MsgInstantiateContract2
/// 
/// **gov**   MsgSubmitProposal | MsgDeposit
/// 
/// **dist**  MsgFundCommunityPool
/// 
/// **stake** MsgDelegate
/// 
/// **ibc**   MsgTransfer
pub fn get_transfer_balances(msgs: Vec<SdkMsg>)-> StdResult<Vec<Coin>> {

    let mut transfer_balances: Vec<Coin> = Vec::new();
    for msg in msgs {
        match msg.type_url.as_str() {
            MsgSend::TYPE_URL => {
                let msg_send = MsgSend::decode(msg.value.as_slice()).unwrap();
                let amount: Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin> = msg_send.amount;
            
                add_balances(transfer_balances.as_mut(), amount)?;
            },
            MsgMultiSend::TYPE_URL => {
                let msg_multi_send = MsgMultiSend::decode(msg.value.as_slice()).unwrap();
                
                for input in msg_multi_send.inputs {
                    add_balances(transfer_balances.as_mut(), input.coins)?;
                }
            },
            MsgExecuteContract::TYPE_URL => {
                let msg_wasm_exec = MsgExecuteContract::decode(msg.value.as_slice()).unwrap();
                let funds = msg_wasm_exec.funds;

                add_balances(transfer_balances.as_mut(), funds)?;
            },
            MsgInstantiateContract::TYPE_URL => {
                let msg_wasm_instantiate = MsgInstantiateContract::decode(msg.value.as_slice()).unwrap();
                let funds = msg_wasm_instantiate.funds;

                add_balances(transfer_balances.as_mut(), funds)?;
            },
            WASM_INSTANTIATE2_TYPE_URL => {
                let msg_wasm_instantiate2 = MsgInstantiateContract2::decode(msg.value.as_slice()).unwrap();
                let funds = msg_wasm_instantiate2.funds;

                add_balances(transfer_balances.as_mut(), funds)?;
            },
            MsgTransfer::TYPE_URL => {
                let msg_ibc_transfer = MsgTransfer::decode(msg.value.as_slice()).unwrap();
                let token = msg_ibc_transfer.token;

                if token.is_some() {
                    add_balances(transfer_balances.as_mut(), vec![token.unwrap()])?;
                }
            },
            MsgFundCommunityPool::TYPE_URL => {
                let msg_fund_com_pool = MsgFundCommunityPool::decode(msg.value.as_slice()).unwrap();
                let amount = msg_fund_com_pool.amount;

                add_balances(transfer_balances.as_mut(), amount)?;
            },
            GOV_SUBMIT_PROPOSAL => {
                let msg_submit_proposal = MsgSubmitProposal::decode(msg.value.as_slice()).unwrap();
                let initital_deposit = msg_submit_proposal.initial_deposit;

                add_balances(transfer_balances.as_mut(), initital_deposit)?;
            },
            GOV_DEPOSIT_PROPOSAL => {
                let msg_deposit_proposal = MsgDeposit::decode(msg.value.as_slice()).unwrap();
                let amount = msg_deposit_proposal.amount;

                add_balances(transfer_balances.as_mut(), amount)?;

            },
            MsgDelegate::TYPE_URL => {
                let msg_delegate = MsgDelegate::decode(msg.value.as_slice()).unwrap();
                let amount = msg_delegate.amount;

                if amount.is_some() {
                    add_balances(transfer_balances.as_mut(), vec![amount.unwrap()])?;
                }
            },
            _ => {}
        }
    }

    Ok(transfer_balances)
}

fn add_balances(
    balances: &mut Vec<Coin>, 
    coins: Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>
) -> StdResult<()> {
    for coin in coins {
        let amount = Uint128::from(u128::from_str_radix(&coin.amount, 10).unwrap());

        if let Some(idx) = balances.iter().position(|b| b.denom.eq(&coin.denom)){
            balances[idx].amount = balances[idx].amount.checked_add(amount).unwrap();
        } else{
            balances.push(Coin { 
                denom: coin.denom, 
                amount, 
            });
        }
    }
    
    Ok(())
}