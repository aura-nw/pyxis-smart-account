use crate::contract::{execute, instantiate, query};
use cosmwasm_std::Empty;
use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper};
use sample_plugin::contract::{
    execute as plugin_execute, instantiate as plugin_instantiate, query as plugin_query,
};
use std::collections::HashMap;

pub fn smart_account_code() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

pub fn sample_plugin_code() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(plugin_execute, plugin_instantiate, plugin_query);
    Box::new(contract)
}

pub fn mock_app<'a>() -> (App, HashMap<&'a str, u64>) {
    let mut app = AppBuilder::new().build(|_router, _api, _storage| {});
    let mut code_ids: HashMap<&'a str, u64> = HashMap::new();

    let smart_account_code = app.store_code(smart_account_code());
    code_ids.insert("smart_account", smart_account_code);

    let sample_plugin_code = app.store_code(sample_plugin_code());
    code_ids.insert("sample_plugin", sample_plugin_code);

    (app, code_ids)
}
