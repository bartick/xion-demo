use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<Addr>
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateAdmin {
        new_admin: Addr,
    },
    StartLottery {},
    JoinLottery {},
    PickWinner {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(cosmwasm_std::Binary)]
    Admin {},
    #[returns(cosmwasm_std::Binary)]
    Winner {},
    #[returns(cosmwasm_std::Binary)]
    LotteryBalance {},
    #[returns(cosmwasm_std::Binary)]
    TotalParticipants {},
}