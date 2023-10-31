use crate::contract::{execute, instantiate, query, sudo as sudo_fn};
use crate::msg::InstantiateMsg;
use aura_test_tube::{AuraTestApp, Wasm};
use cosmwasm_std::{coins, Addr, Empty};
use pyxis_sm::plugin_manager_msg::PluginType;
use sample_plugin::{
    contract::{
        execute as plugin_execute, instantiate as plugin_instantiate, query as plugin_query,
    },
    msg::InstantiateMsg as PluginInstantiateMsg,
};
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
use test_tube::{Module, SigningAccount};

// since we haven't been able to use instantiate2 with cw_multi_test, we need to use a hardcoded address
pub const SM_ADDRESS: &str = "contract1";
pub const ROOT_PATH: &str = "artifacts";

pub fn smart_account_code() -> Vec<u8> {
    std::fs::read(format!("{}/pyxis_sm_base.wasm", ROOT_PATH)).unwrap()
}

pub fn sample_plugin_code() -> Vec<u8> {
    std::fs::read(format!("{}/sample_plugin.wasm", ROOT_PATH)).unwrap()
}

pub fn recovery_plugin_code() -> Vec<u8> {
    std::fs::read(format!("{}/simple_recovery_plugin.wasm", ROOT_PATH)).unwrap()
}

pub fn sample_plugin_manager_code() -> Vec<u8> {
    std::fs::read(format!("{}/sample_plugin_manager.wasm", ROOT_PATH)).unwrap()
}

pub fn mock_app<'a>() -> (AuraTestApp, SigningAccount, HashMap<&'a str, u64>) {
    let app = AuraTestApp::default();
    let wasm = Wasm::new(&app);

    println!("current directory: {:?}", std::env::current_dir().unwrap());

    let deployer = app
        .init_base_account(&coins(100_000_000_000, "uaura"))
        .unwrap();

    let mut code_ids: HashMap<&'a str, u64> = HashMap::new();

    let smart_account_code_id = wasm
        .store_code(&smart_account_code(), None, &deployer)
        .unwrap()
        .data
        .code_id;
    code_ids.insert("smart_account", smart_account_code_id);

    let sample_plugin_code_id = wasm
        .store_code(&sample_plugin_code(), None, &deployer)
        .unwrap()
        .data
        .code_id;
    code_ids.insert("sample_plugin", sample_plugin_code_id);

    let recovery_plugin_code_id = wasm
        .store_code(&recovery_plugin_code(), None, &deployer)
        .unwrap()
        .data
        .code_id;
    code_ids.insert("recovery_plugin", recovery_plugin_code_id);

    let sample_plugin_manager_code_id = wasm
        .store_code(&sample_plugin_manager_code(), None, &deployer)
        .unwrap()
        .data
        .code_id;
    code_ids.insert("sample_plugin_manager", sample_plugin_manager_code_id);

    println!("code_ids: {:?}", code_ids);

    (app, deployer, code_ids)
}

pub fn setup_contracts<'a>(
    app: &mut AuraTestApp,
    deployer: &SigningAccount,
    code_ids: &HashMap<&str, u64>,
) -> HashMap<String, Addr> {
    let mut contracts: HashMap<String, Addr> = HashMap::new();
    let wasm = Wasm::new(app);

    let instantiate_plugin_manager_res = wasm
        .instantiate(
            *code_ids.get("sample_plugin_manager").unwrap(),
            &PluginManagerInstantiateMsg {},
            None,
            Some("sample_plugin_manager"),
            &[],
            deployer,
        )
        .unwrap();

    println!(
        "sample plugin manager: {:?}",
        instantiate_plugin_manager_res
    );

    let plugin_manager_addr = Addr::unchecked(instantiate_plugin_manager_res.data.address);
    contracts.insert("plugin_manager".to_string(), plugin_manager_addr.clone());

    let instantiate_sm_res = wasm
        .instantiate(
            *code_ids.get("smart_account").unwrap(),
            &InstantiateMsg {
                plugin_manager_addr,
            },
            None,
            Some("smart account 1"),
            &vec![],
            deployer,
        )
        .unwrap();
    println!("smart_account_addr: {:?}", instantiate_sm_res);
    contracts.insert(
        "smart_account".to_string(),
        Addr::unchecked(instantiate_sm_res.data.address),
    );

    // loop to create 5 plugins
    for i in 1..5 {
        let response = wasm
            .instantiate(
                *code_ids.get("sample_plugin").unwrap(),
                &PluginInstantiateMsg {},
                None,
                Some("sample plugin 1"),
                &vec![],
                deployer,
            )
            .unwrap();
        println!("plugin_addr: {:?}", response.data.address);

        let key = format!("plugin_{}", i);
        contracts.insert(key, Addr::unchecked(response.data.address));
    }

    // create a recovery plugin
    let recovery_plugin_res = wasm
        .instantiate(
            *code_ids.get("recovery_plugin").unwrap(),
            &PluginInstantiateMsg {},
            None,
            Some("recovery plugin"),
            &vec![],
            deployer,
        )
        .unwrap();
    contracts.insert(
        "recovery_plugin".to_string(),
        Addr::unchecked(recovery_plugin_res.data.address),
    );

    contracts
}

pub fn allow_plugin(
    app: &mut AuraTestApp,
    deployer: &SigningAccount,
    contracts: &HashMap<String, Addr>,
    plugin_name: &str,
    plugin_type: PluginType,
) {
    let wasm = Wasm::new(app);
    println!("allowing plugin: {}", contracts.get(plugin_name).unwrap());
    // allow a plugin to be used by smart account by calling the plugin manager
    wasm.execute(
        &contracts.get("plugin_manager").unwrap().to_string(),
        &PluginManagerExecuteMsg::AllowPlugin {
            plugin_address: contracts.get(plugin_name).unwrap().clone(),
            plugin_type,
        },
        &vec![],
        deployer,
    )
    .unwrap();
}
