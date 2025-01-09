use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{error::ContractResult, execute, msg::{ExecuteMsg, InstantiateMsg, QueryMsg}, query::admin, CONTRACT_NAME, CONTRACT_VERSION};


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
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> ContractResult<Response> {
    match msg {
        ExecuteMsg::UpdateAdmin { new_admin } => execute::update_admin(deps, info, new_admin)
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Admin {  } => to_json_binary(&admin(deps.storage)?)
    }
}