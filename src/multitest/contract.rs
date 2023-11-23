use cosmwasm_std::Addr;
use cosmwasm_std::{Coin, StdResult};
use cw_multi_test::{Executor, ContractWrapper, App};
use crate::{ContractError, query, instantiate, execute};
use crate::msg::{InstantiateMsg, ValueResp, QueryMsg, ExecMsg};
pub struct CountingContract(Addr);

impl CountingContract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }

    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: u64,
        sender: &Addr,
        label: &str,
        counter: impl Into<Option<u64>>,
        minimal_donation: Coin,
    ) -> StdResult<Self> {
        let counter = counter.into().unwrap_or_default();
 
        app.instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {
                counter,
                minimal_donation,
            },
            &[],
            label,
            None,
        )
        .map(CountingContract)
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn donate(
        &self,
        app: &mut App,
        sender: &Addr,
        funds: &[Coin],
    ) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Poke {}, funds)
            .map_err(|err| err.downcast().unwrap())
            .map(|_| ())
    }

    #[track_caller]
    pub fn withdraw(&self, app: &mut App, sender: &Addr) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Withdraw {}, &[])
            .map_err(|err| err.downcast().unwrap())
            .map(|_| ())
    }

    #[track_caller]
    pub fn query_value(&self, app: &App) -> StdResult<ValueResp> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Value {})
    }
}
 
impl From<CountingContract> for Addr {
    fn from(contract: CountingContract) -> Self {
        contract.0
    }
}