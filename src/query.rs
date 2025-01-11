use cosmwasm_std::{Addr, Deps, Env, StdResult, Storage};

use crate::state::{ADMIN, WINNER, LOTTERY};

pub fn admin(store: &dyn Storage) -> StdResult<Addr> {
    ADMIN.load(store)
}

pub fn winner(store: &dyn Storage) -> StdResult<Addr> {
    WINNER.load(store)
}

pub fn lottery_balance(deps: Deps, env: Env) -> StdResult<u128> {
    let balance = deps.querier.query_balance(env.contract.address, "uxion")?.amount;
    Ok(balance.u128())
}

pub fn total_participants(store: &dyn Storage) -> StdResult<Vec<Addr>> {
    LOTTERY.load(store)
}