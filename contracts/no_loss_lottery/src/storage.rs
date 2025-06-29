use soroban_sdk::{contracttype, Address, Env};

use crate::{contract::LottoState, error::ContractError};

// TTL constants (in ledgers, ~5 seconds per ledger)
pub const TTL_1_5_DAYS: u32 = 25_920; // 1.5 days * 24 hours * 60 minutes * 12 ledgers/minute
pub const TTL_7_DAYS: u32 = 120_960; // 7 days * 24 hours * 60 minutes * 12 ledgers/minute

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    State,
    Ticket(Address),
    Currency,
}

pub fn check_state(e: Env, state: LottoState) -> Result<bool, ContractError> {
    let current_state: LottoState = e.storage().persistent().get(&DataKey::State).unwrap();
    if current_state != state {
        return Err(ContractError::WrongState);
    };
    Ok(true)
}

pub fn set_state(e: Env, state: LottoState) -> Result<(), ContractError> {
    e.storage().persistent().set(&DataKey::State, &state);
    Ok(())
}

pub fn set_currency(e: Env, currency: Address) -> Result<(), ContractError> {
    e.storage().persistent().set(&DataKey::Currency, &currency);
    Ok(())
}

pub fn get_currency(e: Env) -> Result<Address, ContractError> {
    let currency = e.storage().persistent().get(&DataKey::Currency).unwrap();
    Ok(currency)
}

pub fn get_state(e: Env, state: LottoState) -> Result<LottoState, ContractError> {
    Ok(e.storage()
        .persistent()
        .get(&DataKey::State)
        .ok_or(ContractError::NoStateFound)?)
}

pub fn extend_state_ttl(e: Env, ledgers: u32) {
    let key = DataKey::State;
    e.storage().persistent().extend_ttl(&key, ledgers, ledgers);
}

pub fn init(e: &Env, admin: &Address) {
    e.storage().persistent().set(&DataKey::Admin, &admin);
    e.storage()
        .persistent()
        .set(&DataKey::State, &LottoState::Ended);
}

pub fn admin_exists(e: &Env) -> bool {
    e.storage().persistent().has(&DataKey::Admin)
}
