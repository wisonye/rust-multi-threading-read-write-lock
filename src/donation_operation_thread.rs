use crate::charity_account::{AccountLock, InternationalCharityAccount};
use crate::events::{DonationEvent, SyncingEventType};
use names::{Generator, Name};
use rand::Rng;
use std::{sync::mpsc::Sender, thread, time::Duration};

///
pub struct DonationOperationThread {}

///
impl DonationOperationThread {
    ///
    pub fn start(
        simulate_donation_frequency: Duration,
        account_exclusive_look: AccountLock,
        event_sender: Sender<SyncingEventType>,
    ) {
        thread::spawn(move || loop {
            thread::sleep(simulate_donation_frequency);

            // Random name and donation amount
            let mut rng = rand::thread_rng();
            let mut generator = Generator::default();

            let donor = generator.next().unwrap();
            let donation_amount = rng.gen_range(1, 10) * 100;

            event_sender.send(SyncingEventType::DonationUpdate(DonationEvent {
                donor,
                donation_amount,
            }));

            InternationalCharityAccount::deposit(
                &account_exclusive_look,
                "Wison Ye",
                donation_amount,
            );

            event_sender.send(SyncingEventType::DonationDone);
        });
    }
}
