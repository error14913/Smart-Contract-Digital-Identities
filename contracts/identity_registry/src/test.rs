#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, BytesN, Env, String};

#[test]
fn test_register_and_get() {
    let env = Env::default();
    let contract_id = env.register(IdentityRegistry, ());
    let client = IdentityRegistryClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let name = String::from_str(&env, "unique_user");
    let hash = BytesN::from_array(&env, &[1; 32]);

    env.mock_all_auths();
    client.register(&user, &name, &hash);

    let identity = client.get_identity(&user);
    assert_eq!(identity.name, name);
    assert_eq!(identity.hash, hash);
}

#[test]
#[should_panic(expected = "Status(ContractError(3))")]  // Panics on NameTaken
fn test_name_taken() {
    let env = Env::default();
    let contract_id = env.register(IdentityRegistry, ());
    let client = IdentityRegistryClient::new(&env, &contract_id);

    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let name = String::from_str(&env, "taken");
    let hash = BytesN::from_array(&env, &[1; 32]);

    env.mock_all_auths();
    client.register(&user1, &name, &hash);
    client.register(&user2, &name, &hash);  // Should panic
}

#[test]
fn test_update_hash() {
    let env = Env::default();
    let contract_id = env.register(IdentityRegistry, ());
    let client = IdentityRegistryClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let name = String::from_str(&env, "updater");
    let old_hash = BytesN::from_array(&env, &[1; 32]);
    let new_hash = BytesN::from_array(&env, &[2; 32]);

    env.mock_all_auths();
    client.register(&user, &name, &old_hash);
    client.update_hash(&user, &new_hash);

    let identity = client.get_identity(&user);
    assert_eq!(identity.hash, new_hash);
}