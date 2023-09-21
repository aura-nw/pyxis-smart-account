use std::collections::HashMap;
use std::vec;

use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::Addr;
use cw_multi_test::{App, Executor};
use pyxis_sm::msg::CallInfo;

use crate::contract::{instantiate, register_plugin};
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::testing::test_setup::{allow_plugin, mock_app, setup_contracts, SM_ADDRESS};
use sample_plugin_manager::msg::ExecuteMsg as PluginManagerExecuteMsg;

fn prepare_smart_account_with_plugin(app: &mut App, contracts: &HashMap<String, Addr>) {
    // allow a plugin to be used by smart account by calling the plugin manager
    app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("plugin_manager").unwrap().clone(),
        &PluginManagerExecuteMsg::AllowPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
        },
        &vec![],
    )
    .unwrap();

    // register a plugin with smart account
    app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
            config: "config".to_string(),
            checksum: "checksum".to_string(),
        },
        &vec![],
    )
    .unwrap();
}

#[test]
fn pre_execute_with_a_plugin_always_reject() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    allow_plugin(&mut app, &contracts, "plugin_1");

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
            funds: vec![],
            call_info: CallInfo {
                fee: 0,
                gas_price: 0,
                gas_limit: 0,
            },
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

    allow_plugin(&mut app, &contracts, "plugin_1");

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
            funds: vec![],
            call_info: CallInfo {
                fee: 0,
                gas_price: 0,
                gas_limit: 0,
            },
        }),
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_ok());
}
