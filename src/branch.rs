use crate::charity_account::{AccountLock, InternationalCharityAccount};
use crate::events::{BalanceRefreshEvent, SyncingEventType};
use std::{
    sync::{mpsc::Sender, Arc, RwLock},
    thread,
    time::Duration,
};

///
#[derive(Clone)]
pub struct Branch {
    pub branch_name: String,
    pub share_read_account_lock: AccountLock,
}

///
impl Branch {
    ///
    pub fn new(branch_name: &str, share_read_account_lock: AccountLock) -> Self {
        Branch {
            branch_name: String::from(branch_name),
            share_read_account_lock,
        }
    }

    /// Simulate the branch business system to keep syncing the account balance.
    ///
    /// But we don't implement any branch big screen UI rendering here, we just
    /// simulate it will send the event back to the `HeadquarterOffice` system,
    /// as all the branch balances will display on the headquarter office big
    /// screen.
    pub fn sync_balance(
        &self,
        refresh_frequency: Duration,
        event_sender: Sender<SyncingEventType>,
    ) {
        loop {
            thread::sleep(refresh_frequency);

            let latest_balance =
                InternationalCharityAccount::get_current_balance(&self.share_read_account_lock);
            let display_content = if latest_balance == -1 {
                "BAL: donation is happening ...".to_string()
            } else {
                format!("BAL: {}", latest_balance)
            };

            // Send event
            event_sender.send(SyncingEventType::BranchBalanceUpdate(BalanceRefreshEvent {
                branch_name: self.branch_name.clone(),
                display_content,
            }));
        }
    }
}
