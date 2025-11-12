#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, Env, Symbol, BytesN, Address, Vec, Map, symbol_short
};

#[contracttype]
#[derive(Clone)]
pub struct Identity {
    pub owner: Address,
    pub identity_type: Symbol,
    pub metadata: Symbol,
    pub credentials: Vec<BytesN<32>>,
}

#[contracttype]
#[derive(Clone)]
pub struct Credential {
    pub issuer: Address,
    pub credential_type: Symbol,
    pub issue_date: i64,
    pub expiry_date: i64,
    pub revoked: bool,
}

#[contract]
pub struct DigitalIdentityContract;

#[contractimpl]
impl DigitalIdentityContract {
    pub fn register_identity(
        env: Env,
        identity_id: BytesN<32>,
        owner: Address,
        identity_type: Symbol,
        metadata: Symbol,
    ) {
        owner.require_auth();
        let ids_key = symbol_short!("ids");
        let mut identities: Map<BytesN<32>, Identity> = 
            env.storage().persistent().get(&ids_key).unwrap_or(Map::new(&env));

        if identities.contains_key(identity_id.clone()) {
            panic!("Identity exists");
        }

        let identity = Identity {
            owner: owner.clone(),
            identity_type,
            metadata,
            credentials: Vec::new(&env),
        };

        identities.set(identity_id.clone(), identity);
        env.storage().persistent().set(&ids_key, &identities);
        env.events().publish((symbol_short!("id"), symbol_short!("reg")), identity_id);
    }

    pub fn get_identity(env: Env, identity_id: BytesN<32>) -> Option<(Address, Symbol, Symbol, Vec<BytesN<32>>)> {
        let identities: Map<BytesN<32>, Identity> = 
            env.storage().persistent().get(&symbol_short!("ids")).unwrap_or(Map::new(&env));
        
        identities.get(identity_id).map(|i| {
            (i.owner.clone(), i.identity_type.clone(), i.metadata.clone(), i.credentials.clone())
        })
    }

    pub fn transfer_identity(env: Env, invoker: Address, identity_id: BytesN<32>, new_owner: Address) {
        invoker.require_auth();
        new_owner.require_auth();

        let ids_key = symbol_short!("ids");
        let mut identities: Map<BytesN<32>, Identity> = 
            env.storage().persistent().get(&ids_key).unwrap_or(Map::new(&env));
        let mut identity = identities.get(identity_id.clone()).expect("Not found");

        if identity.owner != invoker {
            panic!("Not owner");
        }

        identity.owner = new_owner.clone();
        identities.set(identity_id, identity);
        env.storage().persistent().set(&ids_key, &identities);
    }

    pub fn issue_credential(
        env: Env,
        credential_id: BytesN<32>,
        identity_id: BytesN<32>,
        issuer: Address,
        credential_type: Symbol,
        issue_date: i64,
        expiry_date: i64,
    ) {
        issuer.require_auth();

        let creds_key = symbol_short!("creds");
        let mut credentials: Map<BytesN<32>, Credential> = 
            env.storage().persistent().get(&creds_key).unwrap_or(Map::new(&env));

        if credentials.contains_key(credential_id.clone()) {
            panic!("Cred exists");
        }

        let ids_key = symbol_short!("ids");
        let mut identities: Map<BytesN<32>, Identity> = 
            env.storage().persistent().get(&ids_key).unwrap_or(Map::new(&env)); // ĐÃ SỬA DÒNG NÀY
        let mut identity = identities.get(identity_id.clone()).expect("Not found");

        let credential = Credential {
            issuer: issuer.clone(),
            credential_type,
            issue_date,
            expiry_date,
            revoked: false,
        };

        credentials.set(credential_id.clone(), credential);
        identity.credentials.push_back(credential_id.clone());

        identities.set(identity_id, identity);
        env.storage().persistent().set(&creds_key, &credentials);
        env.storage().persistent().set(&ids_key, &identities);
    }

    pub fn verify_credential(env: Env, credential_id: BytesN<32>) -> Option<(Address, Symbol, i64, i64, bool)> {
        let credentials: Map<BytesN<32>, Credential> = 
            env.storage().persistent().get(&symbol_short!("creds")).unwrap_or(Map::new(&env));
        
        credentials.get(credential_id).map(|c| {
            (c.issuer.clone(), c.credential_type.clone(), c.issue_date, c.expiry_date, c.revoked)
        })
    }

    pub fn revoke_credential(env: Env, invoker: Address, credential_id: BytesN<32>) {
        invoker.require_auth();

        let creds_key = symbol_short!("creds");
        let mut credentials: Map<BytesN<32>, Credential> = 
            env.storage().persistent().get(&creds_key).unwrap_or(Map::new(&env));
        let mut credential = credentials.get(credential_id.clone()).expect("Not found");

        if credential.issuer != invoker {
            panic!("Not issuer");
        }

        credential.revoked = true;
        credentials.set(credential_id, credential);
        env.storage().persistent().set(&creds_key, &credentials);
    }
}
