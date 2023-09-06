use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub struct Plugin {
    contract_address: Addr,
    checksum: String,
    status: bool,
    config: String,
}

// PLUGINS is a list of registered plugins
pub const PLUGINS: Item<Vec<Plugin>> = Item::new("plugins");
