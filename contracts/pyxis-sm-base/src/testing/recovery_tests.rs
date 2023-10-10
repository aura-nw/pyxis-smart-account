use std::vec;

use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::Addr;
use cw_multi_test::{App, Executor};
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
