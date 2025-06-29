use soroban_sdk::{contract, contractimpl, contracttype, token::Client, Address, Env};

use crate::error::ContractError;

use crate::storage::{
    admin_exists, check_state, extend_state_ttl, get_currency, init, set_currency, set_state,
    TTL_1_5_DAYS, TTL_7_DAYS,
};

#[contract]
pub struct Contract;

#[contracttype]
pub struct Ticket {
    user: Address,
    class: u32,
}

#[contracttype]
pub enum TicketSize {
    Small,
    Medium,
    Large,
}

impl TicketSize {
    const fn amount(&self) -> i128 {
        match self {
            TicketSize::Small => 10_000_000i128,
            TicketSize::Medium => 100_000_000i128,
            TicketSize::Large => 1_000_000_000i128,
        }
    }
}

#[contracttype]
#[derive(PartialEq, PartialOrd, Debug)]
pub enum LottoState {
    Sale,
    Yielding,
    Payback,
    Raffle,
    Ended,
}

#[contractimpl]
impl Contract {
    pub fn init(e: Env, admin: Address, currency: Address) -> Result<(), ContractError> {
        if admin_exists(&e) {
            return Err(ContractError::AlreadyInitialized);
        }
        init(&e, &admin);
        set_state(e.clone(), LottoState::Ended)?;
        extend_state_ttl(e.clone(), TTL_1_5_DAYS);
        set_currency(e, currency)?;
        Ok(())
    }

    pub fn start_sale(e: Env) -> Result<(), ContractError> {
        if check_state(e.clone(), LottoState::Ended)? {
            set_state(e.clone(), LottoState::Sale)?;
            extend_state_ttl(e, TTL_1_5_DAYS);
        }
        Ok(())
    }

    pub fn buy_ticket(e: Env, user: Address, size: TicketSize) -> Result<(), ContractError> {
        if check_state(e.clone(), LottoState::Sale)? {
            let token_address = get_currency(e)?;

            let client = Client::new(&e, &token_address);
            client.transfer(&user, &e.current_contract_address(), &amount);
        }
        Ok(())
    }

    pub fn deposit_to_blend(e: Env) -> Result<(), ContractError> {
        if check_state(e.clone(), LottoState::Sale)? {
            set_state(e.clone(), LottoState::Yielding)?;
            extend_state_ttl(e, TTL_7_DAYS);
        }
        Ok(())
    }

    pub fn withdraw_from_blend(e: Env) -> Result<(), ContractError> {
        if check_state(e.clone(), LottoState::Yielding)? {
            set_state(e.clone(), LottoState::Payback)?;
            extend_state_ttl(e, TTL_1_5_DAYS);
        }
        Ok(())
    }

    pub fn winner_chicken_dinner(e: Env) -> Result<(), ContractError> {
        if check_state(e.clone(), LottoState::Raffle)? {
            set_state(e, LottoState::Ended)?;
        }
        Ok(())
    }

    pub fn claim_ticket(e: Env, user: Address) -> Result<(), ContractError> {
        if check_state(e.clone(), LottoState::Payback)? {
            set_state(e, LottoState::Raffle)?;
        }
        Ok(())
    }
}
