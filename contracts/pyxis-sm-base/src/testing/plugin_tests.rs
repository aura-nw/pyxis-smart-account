use std::vec;

use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::Addr;
use cw_multi_test::Executor;

use crate::testing::test_setup::mock_app;
use crate::{contract::instantiate, msg::InstantiateMsg};
use sample_plugin::msg::InstantiateMsg as PluginInstantiateMsg;

const SM_ADDRESS: &str = "smart_account_1";

#[test]
fn proper_instantiation() {
    let mut deps = mock_dependencies();
    let info = mock_info(SM_ADDRESS, &[]);
    let msg = InstantiateMsg {};
    let env = mock_env();
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn register_plugin() {
    let (mut app, code_ids) = mock_app();

    let smart_account_addr = app.instantiate_contract(
        *code_ids.get("smart_account").unwrap(),
        Addr::unchecked(SM_ADDRESS),
        &InstantiateMsg {},
        &vec![],
        "smart account 1",
        Some(SM_ADDRESS.to_string()),
    );
    assert!(smart_account_addr.is_ok());

    let plugin_addr = app.instantiate_contract(
        *code_ids.get("sample_plugin").unwrap(),
        Addr::unchecked(SM_ADDRESS),
        &PluginInstantiateMsg {},
        &vec![],
        "sample plugin 1",
        Some(SM_ADDRESS.to_string()),
    );
    assert!(plugin_addr.is_ok());
}
