use cosmwasm_std::{Addr, StdResult, Storage};

use crate::state::{ADMIN, WINNER};

pub fn admin(store: &dyn Storage) -> StdResult<Addr> {
    ADMIN.load(store)
}

pub fn winner(store: &dyn Storage) -> StdResult<Addr> {
    WINNER.load(store)
}