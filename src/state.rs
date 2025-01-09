use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// use crate::grant::HelloWorldConfig;

// pub const HELLO_WORLD_CONFIG: Map<String, HelloWorldConfig> = Map::new("hello_world_config");

pub const ADMIN: Item<Addr> = Item::new("admin");