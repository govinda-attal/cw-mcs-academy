use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{
    error::ContractError,
    msg::{ExecMsg, InstantiateMsg},
    state::{MEMBERSHIP, TOTAL_WEIGHT},
};

mod exec;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    MEMBERSHIP.save(deps.storage, &info.sender)?;
    TOTAL_WEIGHT.save(deps.storage, &msg.total_weight)?;
    Ok(Response::new().set_data(msg.data))
}

pub fn execute(
    deps: DepsMut,

    env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> Result<Response, ContractError> {
    use ExecMsg::*;
    match msg {
        Distribute {} => exec::distribute(deps, env, info),
        NewMember { addr, weight } => exec::new_member(deps, env, info, addr, weight),
        Withdraw { weight, diff } => exec::withdraw(deps, env, info, weight, diff),
    }
}
