use crate::msg::InstantiateMsg;
use aura_proto::types::smartaccount::v1beta1::{CodeID, Params};
use aura_test_tube::init_local_smart_account;
use aura_test_tube::SmartAccount;
use aura_test_tube::{Account, Module, Runner, RunnerExecuteResult, SigningAccount};
use aura_test_tube::{AuraTestApp, Wasm};
use cosmos_sdk_proto::cosmos::bank::v1beta1::{MsgSend, MsgSendResponse};
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::traits::MessageExt;
use cosmwasm_std::{coins, Addr};
use pyxis_sm::plugin_manager_msg::PluginStatus;
use pyxis_sm::plugin_manager_msg::PluginType;
use pyxis_sm::plugin_manager_msg::UnregisterRequirement;
use sample_plugin::msg::InstantiateMsg as PluginInstantiateMsg;
use sample_plugin_manager::msg::{
    ExecuteMsg as PluginManagerExecuteMsg, InstantiateMsg as PluginManagerInstantiateMsg,
};
use sample_plugin_manager::state::Plugin;
use std::collections::HashMap;

// since we haven't been able to use instantiate2 with cw_multi_test, we need to use a hardcoded address
pub const SM_ADDRESS: &str = "contract1";
pub const ROOT_PATH: &str = "../../artifacts";

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

pub fn mock_app<'a>() -> (
    AuraTestApp,
    SigningAccount,
    SigningAccount,
    HashMap<&'a str, u64>,
) {
    let app = AuraTestApp::default();
    let wasm = Wasm::new(&app);

    println!("current directory: {:?}", std::env::current_dir().unwrap());

    let deployer = app
        .init_base_account(&coins(100_000_000_000, "uaura"))
        .unwrap();

    let user = app
        .init_base_account(&coins(100_000_000_000, "uaura"))
        .unwrap();

    let mut code_ids: HashMap<&'a str, u64> = HashMap::new();

    let smart_account_code_id = wasm
        .store_code(&smart_account_code(), None, &deployer)
        .unwrap()
        .data
        .code_id;
    code_ids.insert("smart_account", smart_account_code_id);
    // set whitelist for code id, don't need government
    let params = Params {
        whitelist_code_id: vec![CodeID {
            code_id: smart_account_code_id,
            status: true,
        }],
        disable_msgs_list: vec![],
        max_gas_execute: 2000000,
    };
    let param_set = aura_proto::shim::Any {
        type_url: String::from("/aura.smartaccount.v1beta1.Params"),
        value: params.to_bytes().unwrap(),
    };
    let _ = app.set_param_set("smartaccount", param_set.into()).unwrap();
    // query smartaccount module param set
    // let sa_params = smartaccount.query_params().unwrap();
    // assert_eq!(sa_params.params, Some(params));

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

    (app, deployer, user, code_ids)
}

pub fn setup_smart_account(
    app: &mut AuraTestApp,
    user: &SigningAccount,
    code_ids: &HashMap<&str, u64>,
    contracts: &HashMap<String, Addr>,
) -> Addr {
    let smartaccount = SmartAccount::new(app);

    let pub_key = aura_proto::shim::Any {
        type_url: String::from("/cosmos.crypto.secp256k1.PubKey"),
        value: cosmos_sdk_proto::cosmos::crypto::secp256k1::PubKey {
            key: user.public_key().to_bytes(),
        }
        .to_bytes()
        .unwrap(),
    };
    // or simple
    // let pub_key = acc.public_key().to_any().unwrap();
    let salt = "salt123".as_bytes().to_vec();
    let sm_code_id = *code_ids.get("smart_account").unwrap();

    let init_msg = serde_json_wasm::to_string(&InstantiateMsg {
        plugin_manager_addr: contracts.get("plugin_manager").unwrap().clone(),
    })
    .unwrap()
    .as_bytes()
    .to_vec();

    println!(
        "plugin manager addr: {:?}",
        contracts.get("plugin_manager").unwrap()
    );

    let sa_addr = smartaccount
        .query_generate_account(sm_code_id, salt.clone(), init_msg.clone(), pub_key.clone())
        .unwrap();

    // send some coins to the smart account
    let _: RunnerExecuteResult<MsgSendResponse> = app.execute(
        MsgSend {
            from_address: user.address(),
            to_address: sa_addr.clone(),
            amount: vec![Coin {
                denom: "uaura".to_string(),
                amount: "1000000".to_string(),
            }],
        },
        "/cosmos.bank.v1beta1.MsgSend",
        user,
    );

    let sa_acc = init_local_smart_account(sa_addr.clone(), user.private_key()).unwrap();

    let activate_res = smartaccount
        .activate_account(sm_code_id, salt, init_msg, pub_key, &sa_acc)
        .unwrap();

    println!("activate res: {:?}", activate_res);
    return Addr::unchecked(sa_addr);
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
            &PluginManagerInstantiateMsg {
                owner: deployer.address(),
            },
            None,
            Some("sample_plugin_manager"),
            &[],
            deployer,
        )
        .unwrap();

    println!(
        "sample plugin manager: {:?}",
        instantiate_plugin_manager_res.data.address
    );

    let plugin_manager_addr = Addr::unchecked(instantiate_plugin_manager_res.data.address);
    contracts.insert("plugin_manager".to_string(), plugin_manager_addr.clone());

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
    code_ids: &HashMap<&str, u64>,
    plugin_name: &str,
    plugin_type: PluginType,
) {
    let wasm = Wasm::new(app);
    println!("allowing plugin: {}", contracts.get(plugin_name).unwrap());
    // allow a plugin to be used by smart account by calling the plugin manager
    wasm.execute(
        &contracts.get("plugin_manager").unwrap().to_string(),
        &PluginManagerExecuteMsg::AllowPlugin {
            plugin_info: Plugin {
                name: plugin_name.to_string(),
                plugin_type,
                address: contracts.get(plugin_name).unwrap().clone(),
                code_id: *code_ids.get(plugin_name).unwrap(),
                version: "v0.1.0".to_string(),
                status: PluginStatus::Active,
                unregister_req: UnregisterRequirement::Required,
            },
        },
        &vec![],
        deployer,
    )
    .unwrap();
}
