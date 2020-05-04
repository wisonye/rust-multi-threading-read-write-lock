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

    /// If the `lock` is being held by any exclusive write operation,
    /// then all `lock.try_read()` will fail and return an error immediately.
    /// That's what `-1` return value means.
    pub fn get_current_balance(lock: &Arc<RwLock<Self>>) -> isize {
        match lock.try_read() {
            Ok(account) => {
                let account = lock.read().unwrap();
                account.balance as isize
            }
            Err(error) => {
                // `try_lock` failed because the operation would block
                -1
            }
        }
    }

    ///
    pub fn deposit(lock: &Arc<RwLock<Self>>, donor: &str, how_much: usize) -> bool {
        let mut account = lock.write().unwrap();
        thread::sleep(Duration::from_secs(2));
        account.balance += how_much;
        true
    }
}
