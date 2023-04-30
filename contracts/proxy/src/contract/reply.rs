use cosmwasm_std::{coins, BankMsg, DepsMut, Env, Response, StdError, SubMsgResponse};

use crate::{
    error::ContractError,
    state::{CONFIG, PENDING_WITHDRAWAL},
};

pub fn withdraw(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let withdrawl_info = PENDING_WITHDRAWAL.load(deps.storage)?;

    let config = CONFIG.load(deps.storage)?;

    let total_amount = deps
        .querier
        .query_balance(env.contract.address, &config.denom)?;

    let amount = withdrawl_info.amount.unwrap_or(total_amount.amount);

    let send_msg = BankMsg::Send {
        to_address: withdrawl_info.receiver.into_string(),
        amount: coins(amount.u128(), &config.denom),
    };

    Ok(Response::new()
        .add_message(send_msg)
        .add_attribute("action", amount.to_string()))
}

pub fn propose_member(reply: Result<SubMsgResponse, String>) -> Result<Response, ContractError> {
    let response = reply.map_err(StdError::generic_err)?;
    if let Some(data) = response.data {
        let resp = Response::new().set_data(data);
        Ok(resp)
    } else {
        Ok(Response::new())
    }
}
