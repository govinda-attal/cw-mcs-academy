use common::state::membership::MEMBERS;
use cosmwasm_std::{
    ensure, to_binary, DepsMut, Empty, Env, MessageInfo, Order, Response, SubMsg, WasmMsg,
};

use crate::{
    contract::PROXY_INSTANTIATION_REPLY_ID,
    error::ContractError,
    state::{CONFIG, PROPOSALS, VOTES},
};

use proxy::msg::InstantiateMsg as ProxyInstantiateMsg;

pub fn propose_member(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    addr: String,
) -> Result<Response, ContractError> {
    println!("Sender to propose ?? {}", info.sender);
    ensure!(
        MEMBERS.has(deps.storage, &info.sender),
        ContractError::Unauthorized
    );

    let addr = deps.api.addr_validate(&addr)?;

    for member in MEMBERS.range(deps.storage, None, None, Order::Ascending) {
        let (member, _) = member?;
        ensure!(
            proxy::state::OWNER.query(&deps.querier, member)? != addr,
            ContractError::AlreadyAMember
        );
    }

    ensure!(
        !VOTES.has(deps.storage, (&info.sender, &addr)),
        ContractError::AlreadyVoted
    );

    let cnt = PROPOSALS.may_load(deps.storage, &addr)?.unwrap_or(0) + 1;
    VOTES.save(deps.storage, (&info.sender, &addr), &Empty {})?;

    let config = CONFIG.load(deps.storage)?;
    if cnt < config.minimal_acceptances {
        PROPOSALS.save(deps.storage, &addr, &cnt)?;
        let resp = Response::new()
            .add_attribute("action", "propose_member")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("addr", addr.as_str())
            .add_attribute("acceptances", cnt.to_string());
        return Ok(resp);
    }

    PROPOSALS.remove(deps.storage, &addr);

    let inst_msg = ProxyInstantiateMsg {
        owner: addr.to_string(),
        weight: config.starting_weight,
        denom: config.denom,
        direct_part: config.direct_part,
        distribution_contract: config.distribution_contract.into_string(),
        membership_contract: env.contract.address.to_string(),
        half_time: config.half_time,
    };
    let inst_msg = WasmMsg::Instantiate {
        admin: Some(env.contract.address.into_string()),
        code_id: config.proxy_code_id,
        msg: to_binary(&inst_msg)?,
        funds: vec![],
        label: format!("{} Proxy", addr),
    };
    let inst_msg = SubMsg::reply_on_success(inst_msg, PROXY_INSTANTIATION_REPLY_ID);

    let resp = Response::new()
        .add_submessage(inst_msg)
        .add_attribute("action", "propose_member")
        .add_attribute("sender", info.sender.as_str())
        .add_attribute("addr", addr.as_str());
    Ok(resp)
}
