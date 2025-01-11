use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{error::ContractResult, execute, msg::{ExecuteMsg, InstantiateMsg, QueryMsg}, query, CONTRACT_NAME, CONTRACT_VERSION};


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg
) -> ContractResult<Response> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    execute::init(deps, info, msg.admin)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> ContractResult<Response> {
    match msg {
        ExecuteMsg::UpdateAdmin { new_admin } => execute::update_admin(deps, info, new_admin),
        ExecuteMsg::StartLottery {  } => execute::start_lottery(deps, info),
        ExecuteMsg::JoinLottery {  } => execute::add_person_to_lottery(deps, info),
        ExecuteMsg::PickWinner {  } => execute::pick_winner(deps, env, info)
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Admin {  } => to_json_binary(&query::admin(deps.storage)?),
        QueryMsg::Winner {  } => to_json_binary(&query::winner(deps.storage)?),
        QueryMsg::LotteryBalance {  } => to_json_binary(&query::lottery_balance(deps, env)?),
        QueryMsg::TotalParticipants {  } => to_json_binary(&query::total_participants(deps.storage)?),
    }
}