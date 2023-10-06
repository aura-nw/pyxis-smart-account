use std::vec;

use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use pyxis_sm::msg::CallInfo;
use pyxis_sm::plugin_manager_msg::PluginType;

use crate::msg::ExecuteMsg;
use crate::testing::test_setup::{allow_plugin, mock_app, setup_contracts, SM_ADDRESS};

#[test]
fn pre_execute_without_plugin() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    // execute smart account without a plugin
    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::PyxisExecuteMsg(pyxis_sm::msg::PyxisExecuteMsg::PreExecute {
            msgs: vec![],
            call_info: CallInfo::default(),
        }),
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_ok());
}

#[test]
fn pre_execute_with_a_plugin_always_reject() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    allow_plugin(&mut app, &contracts, "plugin_1", PluginType::Other);

    // register plugin with smart account
    app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
            config: "reject".to_string(),
            checksum: "checksum".to_string(),
        },
        &vec![],
    )
    .unwrap();

    // execute smart account with a plugin
    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::PyxisExecuteMsg(pyxis_sm::msg::PyxisExecuteMsg::PreExecute {
            msgs: vec![],
            call_info: CallInfo::default(),
        }),
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_err());
}

#[test]
fn pre_execute_and_plugin_approve() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    allow_plugin(&mut app, &contracts, "plugin_1", PluginType::Other);

    // register plugin with smart account
    app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
            config: "approve".to_string(),
            checksum: "checksum".to_string(),
        },
        &vec![],
    )
    .unwrap();

    // execute smart account with a plugin
    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::PyxisExecuteMsg(pyxis_sm::msg::PyxisExecuteMsg::PreExecute {
            msgs: vec![],
            call_info: CallInfo::default(),
        }),
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_ok());
}

#[test]
fn pre_execute_and_one_plugin_reject() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    allow_plugin(&mut app, &contracts, "plugin_1", PluginType::Other);
    allow_plugin(&mut app, &contracts, "plugin_2", PluginType::Other);

    // register plugin 1 to approve
    app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
            config: "approve".to_string(),
            checksum: "checksum".to_string(),
        },
        &vec![],
    )
    .unwrap();

    // register plugin 2 to reject
    app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_2").unwrap().clone(),
            config: "reject".to_string(),
            checksum: "checksum".to_string(),
        },
        &vec![],
    )
    .unwrap();

    // execute smart account with a plugin
    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::PyxisExecuteMsg(pyxis_sm::msg::PyxisExecuteMsg::PreExecute {
            msgs: vec![],
            call_info: CallInfo::default(),
        }),
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_err());
}

#[test]
fn after_execute_without_plugin_success() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    // execute smart account without a plugin
    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::PyxisExecuteMsg(pyxis_sm::msg::PyxisExecuteMsg::AfterExecute {
            msgs: vec![],
            call_info: CallInfo::default(),
        }),
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_ok());
}

#[test]
fn after_execute_and_plugin_reject() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    allow_plugin(&mut app, &contracts, "plugin_1", PluginType::Other);

    // register plugin with smart account
    app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
            config: "reject".to_string(),
            checksum: "checksum".to_string(),
        },
        &vec![],
    )
    .unwrap();

    // execute smart account with a plugin
    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::PyxisExecuteMsg(pyxis_sm::msg::PyxisExecuteMsg::AfterExecute {
            msgs: vec![],
            call_info: CallInfo::default(),
        }),
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_err());
}

#[test]
fn after_execute_and_plugin_approve() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    allow_plugin(&mut app, &contracts, "plugin_1", PluginType::Other);

    // register plugin with smart account
    app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
            config: "approve".to_string(),
            checksum: "checksum".to_string(),
        },
        &vec![],
    )
    .unwrap();

    // execute smart account with a plugin
    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::PyxisExecuteMsg(pyxis_sm::msg::PyxisExecuteMsg::AfterExecute {
            msgs: vec![],
            call_info: CallInfo::default(),
        }),
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_ok());
}
