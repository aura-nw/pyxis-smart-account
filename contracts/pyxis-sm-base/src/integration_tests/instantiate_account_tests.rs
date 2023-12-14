use aura_test_tube::init_local_smart_account;
use aura_test_tube::{Account, AuraTestApp, Runner, RunnerExecuteResult, SigningAccount};
use cosmos_sdk_proto::cosmos::bank::v1beta1::{MsgSend, MsgSendResponse};
use cosmos_sdk_proto::cosmos::bank::v1beta1::{QueryAllBalancesRequest, QueryAllBalancesResponse};
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;

use cosmwasm_schema::cw_serde;
use std::option::Option::None;

use crate::integration_tests::test_setup::{setup_contracts, setup_smart_account};

use super::test_setup::mock_app;

fn get_account_balances(app: &AuraTestApp, address: String, denom: &str) -> u128 {
    let acc_balance = app
        .query::<QueryAllBalancesRequest, QueryAllBalancesResponse>(
            "/cosmos.bank.v1beta1.Query/AllBalances",
            &QueryAllBalancesRequest {
                address,
                pagination: None,
            },
        )
        .unwrap()
        .balances
        .into_iter()
        .find(|c| c.denom == denom)
        .unwrap()
        .amount
        .parse::<u128>()
        .unwrap();
    return acc_balance;
}

fn send_coin(
    app: &AuraTestApp,
    from: &SigningAccount,
    to: String,
    amounts: Vec<Coin>,
) -> RunnerExecuteResult<MsgSendResponse> {
    app.execute(
        MsgSend {
            from_address: from.address(),
            to_address: to,
            amount: amounts,
        },
        "/cosmos.bank.v1beta1.MsgSend",
        from,
    )
}

#[cw_serde]
struct EmptyInit {}

#[cw_serde]
struct Listen {
    listen: EmptyInit,
}

#[test]
fn test_deploy() {
    let (mut app, deployer, user, code_ids) = mock_app();
    let contracts = setup_contracts(&mut app, &deployer, &code_ids);

    // let sm_address = contracts.get("smart_account").unwrap().clone();

    let sm_address = setup_smart_account(&mut app, &deployer, &code_ids, &contracts);

    let smartaccount =
        init_local_smart_account(sm_address.to_string(), deployer.private_key()).unwrap();

    // let send_res: RunnerExecuteResult<MsgSendResponse> = app.execute(
    //     MsgSend {
    //         from_address: sm_address.to_string(),
    //         to_address: deployer.address(),
    //         amount: vec![Coin {
    //             denom: "uaura".to_string(),
    //             amount: "10000".to_string(),
    //         }],
    //     },
    //     "/cosmos.bank.v1beta1.MsgSend",
    //     &user,
    // );
    // // this will fail because of insufficient fund
    // let err = send_res.unwrap_err();
    // assert!(err.to_string().contains("insufficient fund"));

    // send some funds to smart account
    let send_res: RunnerExecuteResult<MsgSendResponse> = app.execute(
        MsgSend {
            from_address: user.address(),
            to_address: sm_address.to_string(),
            amount: vec![Coin {
                denom: "uaura".to_string(),
                amount: "100000000".to_string(),
            }],
        },
        "/cosmos.bank.v1beta1.MsgSend",
        &user,
    );
    assert!(send_res.is_ok(), "can send to smart account");

    // send again from smart account, this time is a success
    let send_res: RunnerExecuteResult<MsgSendResponse> = app.execute(
        MsgSend {
            from_address: sm_address.to_string(),
            to_address: deployer.address(),
            amount: vec![Coin {
                denom: "uaura".to_string(),
                amount: "1000".to_string(),
            }],
        },
        "/cosmos.bank.v1beta1.MsgSend",
        &smartaccount,
    );
    println!("send res: {:?}", send_res);

    assert!(send_res.is_ok(), "can send from smart account");
}
