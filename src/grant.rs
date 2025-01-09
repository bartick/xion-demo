use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct HelloWorldConfig {
    description: String,
    pub owner: String,
    pub msg: String,
}