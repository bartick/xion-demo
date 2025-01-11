use cosmwasm_std::{Addr, StdResult, Storage};

use crate::state::ADMIN;

pub fn admin(store: &dyn Storage) -> StdResult<Addr> {
    ADMIN.load(store)
}