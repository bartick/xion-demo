use cosmwasm_std::{Addr, DepsMut, Event, MessageInfo, Response};
use rand::seq::SliceRandom;

use crate::{error::{ContractError::{AlreadyRegistered, Unauthorized, LotteryNotStarted, LotteryAlreadyStarted}, ContractResult}, state::{ADMIN, LOTTERY, START, WINNER}};


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

pub fn pick_winner(
    deps: DepsMut,
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

    let winner = total_registered.choose(&mut rand::thread_rng()).unwrap();

    WINNER.save(deps.storage, winner)?;

    START.save(deps.storage, &false)?;

    Ok(Response::new().add_event(
        Event::new("pick_winner").add_attributes(vec![
            ("winner", winner.clone().into_string()),
        ])
    ))
}