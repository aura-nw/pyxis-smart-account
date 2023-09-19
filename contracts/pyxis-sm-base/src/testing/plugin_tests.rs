use std::collections::HashMap;
use std::vec;

use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::Addr;
use cw_multi_test::{App, Executor};

use crate::contract::instantiate;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::testing::test_setup::mock_app;
use pyxis_sm::msg::{PyxisExecuteMsg, PyxisPluginExecuteMsg};
use sample_plugin::msg::InstantiateMsg as PluginInstantiateMsg;

// since we haven't been able to use instantiate2 with cw_multi_test, we need to use a hardcoded address
const SM_ADDRESS: &str = "contract0";

fn setup_contracts(app: &mut App, code_ids: &HashMap<&str, u64>) -> (Addr, Vec<Addr>) {
    let smart_account_addr = app.instantiate_contract(
        *code_ids.get("smart_account").unwrap(),
        Addr::unchecked(SM_ADDRESS),
        &InstantiateMsg {
            plugin_manager_addr: Addr::unchecked("plugin_manager_addr"),
        },
        &vec![],
        "smart account 1",
        Some(SM_ADDRESS.to_string()),
    );
    println!("smart_account_addr: {:?}", smart_account_addr);
    assert!(smart_account_addr.is_ok());

    // loop to create 5 plugins
    let mut plugin_addresses: Vec<Addr> = vec![];
    for i in 1..5 {
        let plugin_addr = app.instantiate_contract(
            *code_ids.get("sample_plugin").unwrap(),
            Addr::unchecked(SM_ADDRESS),
            &PluginInstantiateMsg {},
            &vec![],
            "sample plugin 1",
            Some(SM_ADDRESS.to_string()),
        );
        println!("plugin_addr: {:?}", plugin_addr);
        assert!(plugin_addr.is_ok());

        plugin_addresses.push(plugin_addr.unwrap());
    }

    (smart_account_addr.unwrap(), plugin_addresses)
}

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
fn register_plugin() {
    let (mut app, code_ids) = mock_app();

    let (smart_account_addr, plugin_addrs) = setup_contracts(&mut app, &code_ids);

    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        smart_account_addr,
        &ExecuteMsg::RegisterPlugin {
            plugin_address: plugin_addrs[0].clone(),
            config: "config".to_string(),
            checksum: "checksum".to_string(),
        },
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_ok());
}

#[test]
fn cannot_register_same_plugin() {
    let (mut app, code_ids) = mock_app();

    let (smart_account_addr, plugin_addrs) = setup_contracts(&mut app, &code_ids);

    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        smart_account_addr.clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: plugin_addrs[0].clone(),
            config: "config".to_string(),
            checksum: "checksum".to_string(),
        },
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_ok());

    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        smart_account_addr,
        &ExecuteMsg::RegisterPlugin {
            plugin_address: plugin_addrs[0].clone(),
            config: "config".to_string(),
            checksum: "checksum".to_string(),
        },
        &vec![],
    );
    println!("response: {:?}", response);
    assert!(response.is_err());
}

fn can_register_two_plugins() {
    let (mut app, code_ids) = mock_app();

    let (smart_account_addr, plugin_addrs) = setup_contracts(&mut app, &code_ids);

    app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        smart_account_addr.clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: plugin_addrs[0].clone(),
            checksum: "checksum".to_string(),
            config: "config".to_string(),
        },
        &vec![],
    )
    .unwrap();

    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        smart_account_addr.clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: plugin_addrs[1].clone(),
            checksum: "checksum".to_string(),
            config: "config".to_string(),
        },
        &vec![],
    );
    assert!(response.is_ok());
}
