use cosmwasm_std::{Addr, DepsMut, Event, MessageInfo, Response};

use crate::{error::{ContractError::Unauthorized, ContractResult}, state::ADMIN};


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