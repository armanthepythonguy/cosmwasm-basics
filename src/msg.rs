use cosmwasm_std::Coin;
use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;

#[cw_serde]
pub struct  InstantiateMsg{
    #[serde(default)]
    pub counter : u64,
    pub minimal_donation : Coin
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResp)]
    Value {},
}

#[cw_serde]
pub struct ValueResp{
    pub value : u64,
}

#[cw_serde]
pub enum ExecMsg{
    Poke {},
    Withdraw {},
}
