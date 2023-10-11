use std::vec;

use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::Addr;
use cw_multi_test::{App, Executor};
use pyxis_sm::msg::PyxisExecuteMsg;
use pyxis_sm::plugin_manager_msg::PluginType;
use simple_recovery_plugin::state::RecoveryConfig;

use crate::contract::instantiate;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::testing::test_setup::{allow_plugin, mock_app, setup_contracts, SM_ADDRESS};

#[test]
fn proper_instantiation() {
    let mut deps = mock_dependencies();
    let info = mock_info(SM_ADDRESS, &[]);
    let msg = InstantiateMsg {
        plugin_manager_addr: Addr::unchecked("plugin_manager_addr"),
    };
    let env = mock_env();
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn cannot_recover_when_not_set_plugin() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::PyxisExecuteMsg(PyxisExecuteMsg::Recover {
            caller: "recoverer".to_string(),
            pubkey: vec![],
            credentials: vec![],
        }),
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_err());
}

#[test]
fn cannot_recover() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    allow_plugin(
        &mut app,
        &contracts,
        "recovery_plugin",
        PluginType::Recovery,
    );

    let recovery_config = RecoveryConfig {
        smart_account_address: contracts.get("smart_account").unwrap().clone(),
        recover_address: Addr::unchecked("recoverer"),
    };

    // register plugin with smart account
    app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("recovery_plugin").unwrap().clone(),
            config: serde_json_wasm::to_string(&recovery_config).unwrap(),
            checksum: "checksum".to_string(),
        },
        &vec![],
    )
    .unwrap();

    // call with incorrect caller will fail
    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::PyxisExecuteMsg(PyxisExecuteMsg::Recover {
            caller: "incorrect_caller".to_string(),
            pubkey: vec![],
            credentials: vec![],
        }),
        &vec![],
    );
    assert!(response.is_err());

    // call with correct caller should success
    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::PyxisExecuteMsg(PyxisExecuteMsg::Recover {
            caller: "recoverer".to_string(),
            pubkey: vec![],
            credentials: vec![],
        }),
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_ok());
}
