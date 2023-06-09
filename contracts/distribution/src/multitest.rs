use anyhow::Result as AnyResult;

use cosmwasm_std::{Addr, Binary, Coin, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::msg::{ExecMsg, InstantiateMsg};
pub struct CodeId(u64);

impl From<CodeId> for u64 {
    fn from(value: CodeId) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub struct Contract(Addr);

impl CodeId {
    pub fn store_code(app: &mut App) -> Self {
        let contract = ContractWrapper::new(super::execute, super::instantiate, super::query);
        CodeId(app.store_code(Box::new(contract)))
    }
}

impl Contract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: CodeId,
        sender: &str,
        total_weight: Uint128,
        data: Binary,
        label: &str,
    ) -> AnyResult<Self> {
        let msg = InstantiateMsg { total_weight, data };

        app.instantiate_contract(code_id.0, Addr::unchecked(sender), &msg, &[], label, None)
            .map(Self)
            .map_err(Into::into)
    }

    #[track_caller]
    pub fn distribute(&self, app: &mut App, sender: Addr, funds: &[Coin]) -> AnyResult<()> {
        app.execute_contract(sender, self.0.clone(), &ExecMsg::Distribute {}, funds)
            .map(|_| ())
            .map_err(Into::into)
    }

    #[track_caller]
    pub fn new_member(
        &self,
        app: &mut App,
        sender: Addr,
        member: &str,
        weight: u64,
    ) -> AnyResult<()> {
        let msg = ExecMsg::NewMember {
            addr: member.to_owned(),
            weight,
        };

        app.execute_contract(sender, self.0.clone(), &msg, &[])
            .map(|_| ())
            .map_err(Into::into)
    }

    #[track_caller]
    pub fn withdraw(&self, app: &mut App, sender: Addr, weight: u64, diff: i64) -> AnyResult<()> {
        let msg = ExecMsg::Withdraw { weight, diff };

        app.execute_contract(sender, self.0.clone(), &msg, &[])
            .map(|_| ())
            .map_err(Into::into)
    }
}

impl From<Addr> for Contract {
    fn from(value: Addr) -> Self {
        Self(value)
    }
}
