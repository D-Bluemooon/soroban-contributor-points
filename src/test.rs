#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env, Symbol};

#[test]
fn register_resolve_and_accumulate_points() {
    let env = Env::default();
    env.mock_all_auths();

    let maintainer = Address::generate(&env);
    let contributor = Address::generate(&env);

    let contract_id = env.register(ContributorPointsContract, ());
    let client = ContributorPointsContractClient::new(&env, &contract_id);

    client.init(&maintainer);

    let issue_a = Symbol::new(&env, "issue_101");
    let issue_b = Symbol::new(&env, "issue_102");

    client.add_issue(&issue_a, &Complexity::Trivial);
    client.add_issue(&issue_b, &Complexity::High);

    client.resolve_issue(&issue_a, &contributor);
    assert_eq!(client.points_of(&contributor), 100);

    client.resolve_issue(&issue_b, &contributor);
    assert_eq!(client.points_of(&contributor), 300);

    assert_eq!(client.resolved_by(&issue_a), Some(contributor.clone()));
}

#[test]
#[should_panic(expected = "issue already resolved")]
fn cannot_resolve_same_issue_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let maintainer = Address::generate(&env);
    let contributor_one = Address::generate(&env);
    let contributor_two = Address::generate(&env);

    let contract_id = env.register(ContributorPointsContract, ());
    let client = ContributorPointsContractClient::new(&env, &contract_id);

    client.init(&maintainer);
    let issue = Symbol::new(&env, "issue_201");
    client.add_issue(&issue, &Complexity::Medium);

    client.resolve_issue(&issue, &contributor_one);
    client.resolve_issue(&issue, &contributor_two); // should panic
}

#[test]
#[should_panic(expected = "issue not registered")]
fn cannot_resolve_unregistered_issue() {
    let env = Env::default();
    env.mock_all_auths();

    let maintainer = Address::generate(&env);
    let contributor = Address::generate(&env);

    let contract_id = env.register(ContributorPointsContract, ());
    let client = ContributorPointsContractClient::new(&env, &contract_id);

    client.init(&maintainer);
    let issue = Symbol::new(&env, "ghost_issue");
    client.resolve_issue(&issue, &contributor); // should panic
}
