#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, Address, BytesN, Env, Map, String, panic_with_error};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    NameToOwner,
    AddressToIdentity,
}

#[contracttype]
#[derive(Clone)]
pub struct Identity {
    pub name: String,
    pub hash: BytesN<32>,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyRegistered = 1,
    NotRegistered = 2,
    NameTaken = 3,
}

#[contract]
pub struct IdentityRegistry;

#[contractimpl]
impl IdentityRegistry {
    fn get_name_to_owner(e: &Env) -> Map<String, Address> {
        e.storage()
            .persistent()
            .get(&DataKey::NameToOwner)
            .unwrap_or_else(|| Map::new(e))
    }

    fn set_name_to_owner(e: &Env, map: &Map<String, Address>) {
        e.storage().persistent().set(&DataKey::NameToOwner, map);
    }

    fn get_address_to_identity(e: &Env) -> Map<Address, Identity> {
        e.storage()
            .persistent()
            .get(&DataKey::AddressToIdentity)
            .unwrap_or_else(|| Map::new(e))
    }

    fn set_address_to_identity(e: &Env, map: &Map<Address, Identity>) {
        e.storage().persistent().set(&DataKey::AddressToIdentity, map);
    }

    pub fn register(e: Env, user: Address, name: String, hash: BytesN<32>) {
        user.require_auth();

        let mut name_to_owner = Self::get_name_to_owner(&e);
        if name_to_owner.contains_key(name.clone()) {
            panic_with_error!(&e, Error::NameTaken);
        }

        let mut address_to_identity = Self::get_address_to_identity(&e);
        if address_to_identity.contains_key(user.clone()) {
            panic_with_error!(&e, Error::AlreadyRegistered);
        }

        name_to_owner.set(name.clone(), user.clone());
        let identity = Identity { name, hash };
        address_to_identity.set(user.clone(), identity);

        Self::set_name_to_owner(&e, &name_to_owner);
        Self::set_address_to_identity(&e, &address_to_identity);
    }

    pub fn update_hash(e: Env, user: Address, hash: BytesN<32>) {
        user.require_auth();

        let mut address_to_identity = Self::get_address_to_identity(&e);
        if !address_to_identity.contains_key(user.clone()) {
            panic_with_error!(&e, Error::NotRegistered);
        }

        let mut identity = address_to_identity.get(user.clone()).unwrap();
        identity.hash = hash;
        address_to_identity.set(user, identity);

        Self::set_address_to_identity(&e, &address_to_identity);
    }

    pub fn get_identity(e: Env, address: Address) -> Identity {
        let address_to_identity = Self::get_address_to_identity(&e);
        if !address_to_identity.contains_key(address.clone()) {
            panic_with_error!(&e, Error::NotRegistered);
        }
        address_to_identity.get(address).unwrap()
    }
}