use cosmwasm_std::{coins, Addr, BankMsg, CosmosMsg, DepsMut, Empty, Env, Event, MessageInfo, Response, Uint128};

use crate::{error::{ContractError::{AlreadyRegistered, LotteryAlreadyStarted, LotteryNotStarted, Unauthorized, NotEnoughFunds}, ContractResult}, state::{ADMIN, LOTTERY, START, WINNER}};


pub fn init(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<Addr>
) -> ContractResult<Response> {

    let hello_world_admin = match admin {
        Some(admin) => admin,
        None => info.sender,
    };
    ADMIN.save(deps.storage, &hello_world_admin)?;
    START.save(deps.storage, &false)?;
    LOTTERY.save(deps.storage, &Vec::new())?;
    WINNER.save(deps.storage, &Addr::unchecked("")).unwrap();


    Ok(Response::new().add_event(
        Event::new("create_hello_world_instance").add_attributes(vec![
            ("admin", hello_world_admin.into_string()),
        ])
    ))
}

pub fn update_admin(
    deps: DepsMut,
    info: MessageInfo,
    new_admin: Addr
) -> ContractResult<Response> {
    let admin = ADMIN.load(deps.storage)?;
    if admin != info.sender {
        return Err(Unauthorized);
    }

    ADMIN.save(deps.storage, &new_admin)?;

    Ok(Response::new().add_event(
        Event::new("update_admin").add_attributes(vec![
            ("old_admin", admin.into_string()),
            ("new_admin", new_admin.into_string()),
        ])
    ))
}

pub fn start_lottery(
    deps: DepsMut,
    info: MessageInfo,
) -> ContractResult<Response> {
    let admin = ADMIN.load(deps.storage)?;
    if admin != info.sender {
        return Err(Unauthorized);
    }

    let started = START.load(deps.storage)?;
    if started == true {
        return Err(LotteryAlreadyStarted);
    }

    START.save(deps.storage, &true)?;
    LOTTERY.save(deps.storage, &Vec::new())?;
    WINNER.save(deps.storage, &Addr::unchecked("")).unwrap();

    Ok(Response::new().add_event(
        Event::new("start_lottery").add_attributes(vec![
            ("admin", admin.into_string()),
        ])
    ))
}

pub fn add_person_to_lottery(
    deps: DepsMut,
    info: MessageInfo,
) -> ContractResult<Response> {

    if START.load(deps.storage)? == false {
        return Err(LotteryNotStarted);
    }

    let total_registered = LOTTERY.load(deps.storage)?;

    if total_registered.contains(&info.sender) {
        return Err(AlreadyRegistered);
    }

    let sent_funds = info
        .funds
        .iter()
        .find_map(|v| {
            if v.denom == "uxion" {
                Some(v.amount)
            } else {
                None
            }
        })
        .unwrap_or_else(Uint128::zero);

    if sent_funds == Uint128::zero() {
        return Err(NotEnoughFunds);
    }

    LOTTERY.update(deps.storage, |mut lottery| -> Result<_, cosmwasm_std::StdError> {
        lottery.push(info.sender.clone());
        Ok(lottery)
    })?;

    Ok(Response::new().add_event(
        Event::new("add_person_to_lottery").add_attributes(vec![
            ("new entry", info.sender.into_string()),
        ])
    ))
}

fn generate_random_number(env_time: u64, start: u32, end: u32) -> u32 {
    // Ensure the range is valid
    assert!(start < end, "Invalid range: start should be less than end.");

    // Use env_time as a seed-like input
    let a: u64 = 1664525;
    let c: u64 = 1013904223;
    let m: u64 = u32::MAX as u64;

    // Generate a pseudo-random number
    let random = (a.wrapping_mul(env_time).wrapping_add(c)) % m;

    // Map the random number to the desired range [start, end)
    start + (random as u32 % (end - start))
}

pub fn pick_winner(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> ContractResult<Response> {
    let admin = ADMIN.load(deps.storage)?;
    if admin != info.sender {
        return Err(Unauthorized);
    }

    let started = START.load(deps.storage)?;

    if started == false {
        return Err(LotteryNotStarted);
    }

    let total_registered = LOTTERY.load(deps.storage)?;

    let random_number = generate_random_number(env.block.height, 0, total_registered.len() as u32);

    let winner = total_registered[random_number as usize].clone();

    WINNER.save(deps.storage, &winner)?;

    START.save(deps.storage, &false)?;

    let contract_balance = deps.querier.query_balance(env.contract.address, "uxion")?.amount;

    let send_msg = CosmosMsg::<Empty>::Bank(BankMsg::Send { to_address: winner.to_string(), amount: coins(contract_balance.u128(), "uxion") });

    Ok(Response::new().add_event(
        Event::new("pick_winner").add_attributes(vec![
            ("winner", winner.clone().into_string()),
        ])
    ).add_message(send_msg))
}