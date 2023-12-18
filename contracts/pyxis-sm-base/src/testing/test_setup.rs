use crate::contract::{execute, instantiate, query, sudo as sudo_fn};
use crate::msg::InstantiateMsg;
use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};
use pyxis_sm::plugin_manager_msg::PluginType;
use sample_plugin::{
    contract::{
        execute as plugin_execute, instantiate as plugin_instantiate, query as plugin_query,
    },
    msg::InstantiateMsg as PluginInstantiateMsg,
};
use sample_plugin_manager::state::Plugin;
use sample_plugin_manager::{
    contract::{
        execute as plugin_manager_execute, instantiate as plugin_manager_instantiate,
        query as plugin_manager_query,
    },
    msg::{ExecuteMsg as PluginManagerExecuteMsg, InstantiateMsg as PluginManagerInstantiateMsg},
};
use simple_recovery_plugin::contract::{
    execute as recovery_plugin_execute, instantiate as recovery_plugin_instantiate,
    query as recovery_plugin_query,
};
use std::collections::HashMap;

// since we haven't been able to use instantiate2 with cw_multi_test, we need to use a hardcoded address
pub const SM_ADDRESS: &str = "contract1";

pub fn smart_account_code() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query).with_sudo(sudo_fn);
    Box::new(contract)
}

pub fn sample_plugin_code() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(plugin_execute, plugin_instantiate, plugin_query);
    Box::new(contract)
}

pub fn recovery_plugin_code() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        recovery_plugin_execute,
        recovery_plugin_instantiate,
        recovery_plugin_query,
    );
    Box::new(contract)
}

pub fn sample_plugin_manager_code() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        plugin_manager_execute,
        plugin_manager_instantiate,
        plugin_manager_query,
    );
    Box::new(contract)
}

pub fn mock_app<'a>() -> (App, HashMap<&'a str, u64>) {
    let mut app = AppBuilder::new().build(|_router, _api, _storage| {});
    let mut code_ids: HashMap<&'a str, u64> = HashMap::new();

    let smart_account_code = app.store_code(smart_account_code());
    code_ids.insert("smart_account", smart_account_code);

    let sample_plugin_code = app.store_code(sample_plugin_code());
    code_ids.insert("sample_plugin", sample_plugin_code);

    let recovery_plugin_code = app.store_code(recovery_plugin_code());
    code_ids.insert("recovery_plugin", recovery_plugin_code);

    let sample_plugin_manager_code = app.store_code(sample_plugin_manager_code());
    code_ids.insert("sample_plugin_manager", sample_plugin_manager_code);

    (app, code_ids)
}

pub fn setup_contracts<'a>(app: &mut App, code_ids: &HashMap<&str, u64>) -> HashMap<String, Addr> {
    let mut contracts: HashMap<String, Addr> = HashMap::new();

    let plugin_manager_addr = app.instantiate_contract(
        *code_ids.get("sample_plugin_manager").unwrap(),
        Addr::unchecked(SM_ADDRESS),
        &PluginManagerInstantiateMsg {
            admin: SM_ADDRESS.to_string(),
        },
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

    // create a recovery plugin
    let recovery_plugin_addr = app.instantiate_contract(
        *code_ids.get("recovery_plugin").unwrap(),
        Addr::unchecked(SM_ADDRESS),
        &PluginInstantiateMsg {},
        &vec![],
        "recovery plugin",
        Some(SM_ADDRESS.to_string()),
    );
    contracts.insert("recovery_plugin".to_string(), recovery_plugin_addr.unwrap());

    contracts
}

pub fn allow_plugin(
    app: &mut App,
    contracts: &HashMap<String, Addr>,
    code_ids: &HashMap<&str, u64>,
    plugin_name: &str,
    plugin_type: PluginType,
) {
    println!("allowing plugin: {}", contracts.get(plugin_name).unwrap());
    // allow a plugin to be used by smart account by calling the plugin manager
    app.execute_contract(
        Addr::unchecked(SM_ADDRESS),
        contracts.get("plugin_manager").unwrap().clone(),
        &PluginManagerExecuteMsg::AllowPlugin {
            plugin_info: Plugin {
                name: plugin_name.to_string(),
                plugin_type,
                address: contracts.get(plugin_name).unwrap().clone(),
                code_id: *code_ids.get(plugin_name).unwrap(),
                version: "v0.1.0".to_string(),
                enabled: true,
            },
        },
        &vec![],
    )
    .unwrap();
}
