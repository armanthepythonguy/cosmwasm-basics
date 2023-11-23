use contract::execute;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Empty, StdResult, Response, entry_point, Deps, Binary, to_binary, to_json_binary, StdError};
use thiserror::Error;
mod contract;
mod state;
pub mod msg;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError{
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized - only {owner} can call it")]
    Unauthorized {owner: String}
}

#[entry_point]
pub fn instantiate(_deps : DepsMut, _env: Env, _info: MessageInfo, _msg: msg::InstantiateMsg) -> StdResult<Response> {
    contract::instantiate(_deps, _info, _msg.counter, _msg.minimal_donation);
    Ok(Response::new())
}

#[entry_point]
pub fn query(
    _deps : Deps,
    _env : Env,
    _msg : msg::QueryMsg
) -> StdResult<Binary>{
    use msg::QueryMsg::*;
    match _msg{
        Value {} =>to_json_binary(&contract::query::value(_deps)?),
    }
}
#[entry_point]
pub fn execute(_deps : DepsMut, _env: Env, _info: MessageInfo, _msg: msg::ExecMsg) -> Result<Response, ContractError> {
    use msg::ExecMsg::*;
    match _msg{
        Poke {} => execute::donate(_deps, _info).map_err(ContractError::Std),
        Withdraw {} => execute::withdraw(_deps, _env, _info),
    }
}


#[cfg(test)]
pub mod multitest;
// mod test {
//     use super::*;
//     use cosmwasm_std::{Empty, Addr, coin, coins};
//     use cw_multi_test::{Contract, ContractWrapper, App, Executor};

//     use crate::{execute, instantiate, query, msg::{QueryMsg, ValueResp, ExecMsg, InstantiateMsg}};

//     fn counting_contract() -> Box<dyn Contract<Empty>> {
//         let contract = ContractWrapper::new(execute, instantiate, query);
//         Box::new(contract)
//     }

//     #[test]
//     fn query_value(){
//         let sender = Addr::unchecked("sender");
//         let mut app = App::new(|router, _api, storage|{
//             router
//                 .bank
//                 .init_balance(storage, &sender, coins(10, "atom"))
//                 .unwrap()
//         });
//         let contract_id = app.store_code(counting_contract());
//         let contract_addr = app.instantiate_contract(contract_id, Addr::unchecked("sender"), &InstantiateMsg{counter:0, minimal_donation: coin(10, "atom")}, &[], "Counting Contract", None).unwrap();
//         let resp : ValueResp = app.wrap().query_wasm_smart(contract_addr.clone(), &QueryMsg::Value {  }).unwrap();
//         assert_eq!(resp, ValueResp{value:0});
//         app.execute_contract(
//             Addr::unchecked("sender"),
//             contract_addr.clone(), 
//             &ExecMsg::Poke {  },
//             &coins(10, "atom")
//         ).unwrap();
//         let resp : ValueResp = app.wrap().query_wasm_smart(contract_addr.clone(), &QueryMsg::Value {  }).unwrap();
//         assert_eq!(resp, ValueResp{value:1});
//     }

//     #[test]
//     fn withdraw_test(){
//         let sender = Addr::unchecked("sender");
//         let owner = Addr::unchecked("owner");
//         let mut app = App::new(|router, _api, storage|{
//             router
//                 .bank
//                 .init_balance(storage, &sender, coins(10, "atom"))
//                 .unwrap()
//         });
//         let contract_id = app.store_code(counting_contract());
//         let contract_addr = app.instantiate_contract(contract_id, owner.clone(), &InstantiateMsg{counter:0, minimal_donation: coin(10, "atom")}, &[], "Counting Contract", None).unwrap();
//         let resp : ValueResp = app.wrap().query_wasm_smart(contract_addr.clone(), &QueryMsg::Value {  }).unwrap();
//         assert_eq!(resp, ValueResp{value:0});
//         app.execute_contract(
//             sender.clone(),
//             contract_addr.clone(), 
//             &ExecMsg::Poke {  },
//             &coins(10, "atom")
//         ).unwrap();
//         let resp : ValueResp = app.wrap().query_wasm_smart(contract_addr.clone(), &QueryMsg::Value {  }).unwrap();
//         assert_eq!(resp, ValueResp{value:1});
//         app.execute_contract(
//             owner.clone(),
//             contract_addr.clone(), 
//             &ExecMsg::Withdraw {  },
//             &[]
//         ).unwrap();
//         assert_eq!(
//             app.wrap().query_all_balances(owner).unwrap(),
//             coins(10, "atom")
//         );
//         assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
//         assert_eq!(
//             app.wrap().query_all_balances(contract_addr).unwrap(),
//             vec![]
//         );

//     }

//     #[test]
//     fn unauthorized_withdraw(){
//         let owner = Addr::unchecked("owner");
//         let member = Addr::unchecked("member");
    
//         let mut app = App::default();
    
//         let contract_id = app.store_code(counting_contract());
    
//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 owner.clone(),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(10, "atom"),
//                 },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();
    
//         let err = app
//             .execute_contract(member, contract_addr, &ExecMsg::Withdraw {}, &[])
//             .unwrap_err();
    
//         assert_eq!(
//             ContractError::Unauthorized {
//                 owner: owner.into()
//             },
//             err.downcast().unwrap()
//         );
//     }
// }