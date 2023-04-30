use std::collections::HashMap;

use cosmwasm_std::{Addr, Decimal};
use cw_multi_test::App;

use super::CodeId as MembershipId;
use distribution::multitest::CodeId as DistributionId;
use proxy::multitest::{CodeId as ProxyId, Contract as ProxyContract};

#[test]
fn add_member() {
    let mut app = App::default();

    let denom = "STAR";
    let owner = "owner";
    let members = ["member1", "member2"];
    let candidate = "candidate";

    let proxy_id = ProxyId::store_code(&mut app);
    let distribution_id = DistributionId::store_code(&mut app);
    let membership_id = MembershipId::store_code(&mut app);

    let (membership, data) = membership_id
        .instantiate(
            &mut app,
            owner,
            10,
            denom,
            Decimal::percent(15),
            3600 * 24 * 30,
            2,
            proxy_id,
            distribution_id,
            &members,
            "Membership",
        )
        .unwrap();

    let proxies: HashMap<_, _> = data
        .members
        .into_iter()
        .map(|member| {
            (
                member.owner_addr,
                ProxyContract::from_addr(Addr::unchecked(member.proxy_addr)),
            )
        })
        .collect();

    assert_eq!(proxies.len(), 2);
    assert!(
        membership
            .is_member(&app, proxies[members[0]].addr().as_str())
            .unwrap()
            .is_member
    );
    assert!(
        membership
            .is_member(&app, proxies[members[0]].addr().as_str())
            .unwrap()
            .is_member
    );

    let data = proxies[members[0]]
        .propose_member(&mut app, members[0], candidate)
        .unwrap();

    assert!(data.is_none());

    let data = proxies[members[1]]
        .propose_member(&mut app, members[1], candidate)
        .unwrap();

    let data = data.unwrap();

    assert_eq!(data.owner_addr, candidate);

    assert!(
        membership
            .is_member(&app, data.proxy_addr.as_str())
            .unwrap()
            .is_member
    );
}
