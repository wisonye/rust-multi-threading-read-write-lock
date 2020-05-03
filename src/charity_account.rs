use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

pub type AccountLock = Arc<RwLock<InternationalCharityAccount>>;

///
#[derive(Debug)]
pub struct InternationalCharityAccount {
    balance: usize,
}

///
impl InternationalCharityAccount {
    ///
    pub fn new() -> Self {
        InternationalCharityAccount { balance: 0 }
    }

    ///
    pub fn get_current_balance(lock: &Arc<RwLock<Self>>) -> usize {
        let account = lock.read().unwrap();
        account.balance
    }

    ///
    pub fn deposit(lock: &Arc<RwLock<Self>>, donor: &str, how_much: usize) -> bool {
        let mut account = lock.write().unwrap();
        thread::sleep(Duration::from_secs(2));
        account.balance += how_much;
        true
    }

    pub fn withdraw_into_board_account(lock: &Arc<RwLock<Self>>, how_much: usize) -> bool {
        let mut account = lock.write().unwrap();
        thread::sleep(Duration::from_secs(2));

        if account.balance >= how_much {
            account.balance -= how_much;
            true
        } else {
            false
        }
    }
}
