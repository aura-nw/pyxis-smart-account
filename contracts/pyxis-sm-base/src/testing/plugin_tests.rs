use std::collections::HashMap;
use std::fmt::format;
use std::vec;

use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::Addr;
use cw_multi_test::{App, Executor};

use crate::contract::instantiate;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::testing::test_setup::mock_app;
use pyxis_sm::msg::{PyxisExecuteMsg, PyxisPluginExecuteMsg};
use sample_plugin::msg::InstantiateMsg as PluginInstantiateMsg;
use sample_plugin_manager::msg::{
    ExecuteMsg as PluginManagerExecuteMsg, InstantiateMsg as PluginManagerInstantiateMsg,
};

// since we haven't been able to use instantiate2 with cw_multi_test, we need to use a hardcoded address
const SM_ADDRESS: &str = "contract1";

fn setup_contracts<'a>(app: &mut App, code_ids: &HashMap<&str, u64>) -> HashMap<String, Addr> {
    let mut contracts: HashMap<String, Addr> = HashMap::new();

    let plugin_manager_addr = app.instantiate_contract(
        *code_ids.get("sample_plugin_manager").unwrap(),
        Addr::unchecked(SM_ADDRESS),
        &PluginManagerInstantiateMsg {},
        &vec![],
        "sample plugin manager 1",
        Some(SM_ADDRESS.to_string()),
    );
    println!("sample plugin manager: {:?}", plugin_manager_addr);
    assert!(plugin_manager_addr.is_ok());

    let plugin_manager_addr = plugin_manager_addr.unwrap();
    contracts.insert("plugin_manager".to_string(), plugin_manager_addr.clone());

    let smart_account_addr = app.instantiate_contract(
        *code_ids.get("smart_account").unwrap(),
        Addr::unchecked(SM_ADDRESS),
        &InstantiateMsg {
            plugin_manager_addr,
        },
        &vec![],
        "smart account 1",
        Some(SM_ADDRESS.to_string()),
    );
    println!("smart_account_addr: {:?}", smart_account_addr);
    assert!(smart_account_addr.is_ok());
    contracts.insert("smart_account".to_string(), smart_account_addr.unwrap());

    // loop to create 5 plugins
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

        let plugin_addr = plugin_addr.unwrap();
        let key = format!("plugin_{}", i);
        contracts.insert(key, plugin_addr);
    }

    contracts
}

fn allow_plugin(app: &mut App, contracts: &HashMap<String, Addr>, plugin_name: &str) {
    println!("allowing plugin: {}", contracts.get(plugin_name).unwrap());
    // allow a plugin to be used by smart account by calling the plugin manager
    app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("plugin_manager").unwrap().clone(),
        &PluginManagerExecuteMsg::AllowPlugin {
            plugin_address: contracts.get(plugin_name).unwrap().clone(),
        },
        &vec![],
    )
    .unwrap();
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
fn cannot_register_plugin_without_plugin_manager() {
    let (mut app, code_ids) = mock_app();

    let contracts = setup_contracts(&mut app, &code_ids);

    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
            config: "config".to_string(),
            checksum: "checksum".to_string(),
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

    allow_plugin(&mut app, &contracts, "plugin_1");

    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
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

    let contracts = setup_contracts(&mut app, &code_ids);

    allow_plugin(&mut app, &contracts, "plugin_1");

    let response = app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
            config: "config".to_string(),
            checksum: "checksum".to_string(),
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
            checksum: "checksum".to_string(),
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

    allow_plugin(&mut app, &contracts, "plugin_1");
    allow_plugin(&mut app, &contracts, "plugin_2");

    app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("smart_account").unwrap().clone(),
        &ExecuteMsg::RegisterPlugin {
            plugin_address: contracts.get("plugin_1").unwrap().clone(),
            checksum: "checksum".to_string(),
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
            checksum: "checksum".to_string(),
            config: "config".to_string(),
        },
        &vec![],
    );
    assert!(response.is_ok());
}
