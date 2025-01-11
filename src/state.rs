use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const LOTTERY: Item<Vec<Addr>> = Item::new("lottery");

pub const ADMIN: Item<Addr> = Item::new("admin");

pub const START: Item<bool> = Item::new("start");

pub const WINNER: Item<Addr> = Item::new("winner");