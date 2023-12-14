use std::vec;

use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use pyxis_sm::plugin_manager_msg::PluginType;

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
fn cannot_register_plugin_without_plugin_manager() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
            config: "config".to_string(),
        },
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_err());
}

#[test]
fn register_plugin() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    allow_plugin(
        &mut app,
        &contracts,
        &code_ids,
        "plugin_1",
        PluginType::Other,
    );

    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
            config: "config".to_string(),
        },
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_ok());
}

#[test]
fn cannot_register_same_plugin() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    allow_plugin(
        &mut app,
        &contracts,
        &code_ids,
        "plugin_1",
        PluginType::Other,
    );

    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
            config: "config".to_string(),
        },
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_ok());

    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
            config: "config".to_string(),
        },
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_err());
}

#[test]
fn can_register_two_plugins() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    allow_plugin(
        &mut app,
        &contracts,
        &code_ids,
        "plugin_1",
        PluginType::Other,
    );
    allow_plugin(
        &mut app,
        &contracts,
        &code_ids,
        "plugin_2",
        PluginType::Other,
    );

    app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
            config: "config".to_string(),
        },
        &vec![],
    )
    .unwrap();

    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_2").unwrap().clone(),
            config: "config".to_string(),
        },
        &vec![],
    );
    assert!(response.is_ok());
}
