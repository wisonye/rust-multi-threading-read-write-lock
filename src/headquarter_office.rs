use crate::branch::Branch;
use crate::charity_account::InternationalCharityAccount;
use crate::events::{BalanceRefreshEvent, DonationEvent, SyncingEventType};
use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, RwLock,
    },
    thread,
    time::Duration,
};

pub struct BranchDisplayContent {
    branch_name: String,
    display_content: String,
}

///
pub struct HeadquarterOffice {}

///
impl HeadquarterOffice {
    ///
    pub fn sync_balance_from_all_branches(
        all_branches: Vec<Branch>,
        sender: Sender<SyncingEventType>,
        event_bus: Receiver<SyncingEventType>,
    ) {
        // Branch content to render on big screen
        let mut branch_render_contents: Vec<BranchDisplayContent> = Vec::new();

        // Loop all the big screens and run them into separated thread
        for temp_branch in all_branches.iter() {
            let copied_branch = temp_branch.clone();
            let copied_event_sender = sender.clone();

            branch_render_contents.push(BranchDisplayContent {
                branch_name: copied_branch.branch_name.clone(),
                display_content: "BAL: syncing......".to_string(),
            });

            // Create new thread to simulate each branch system to sync the share account balance
            thread::spawn(move || {
                let refresh_frequency = Duration::from_millis(100);
                copied_branch.sync_balance(refresh_frequency, copied_event_sender);
            });
        }

        //
        // From now on, all branch syncing threads are running in background......
        //

        // Here is the main loop for `HeadquarterOffice` event bus:
        // Every time we got an event, re-render all branch contents on big screen.
        let mut donation_desc = "".to_string();
        while let Ok(event) = event_bus.recv() {
            // thread::sleep(Duration::from_secs(1));

            // Clean console content
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            // println!("event: {:?}", event);

            match event {
                SyncingEventType::BranchBalanceUpdate(balanceRefreshEvent) => {
                    // Update the particular branch render content
                    for temp_branch_content in branch_render_contents.iter_mut() {
                        if balanceRefreshEvent.branch_name == temp_branch_content.branch_name {
                            temp_branch_content.display_content =
                                balanceRefreshEvent.display_content.clone();
                            break;
                        }
                    }
                }
                SyncingEventType::DonationUpdate(donationEvent) => {
                    donation_desc = format!(
                        "{} is making donation: {}",
                        donationEvent.donor, donationEvent.donation_amount
                    );
                }
                SyncingEventType::DonationDone => {
                    donation_desc = "".to_string();
                }
            }

            // Redraw all screen contents
            let mut render_list = Vec::new();
            let title = "\nAll branches syncing result:".to_string();
            let splitter = "--------------------------------------------------".to_string();

            render_list.push(title);
            render_list.push(splitter.clone());
            for temp_branch_content in branch_render_contents.iter_mut() {
                render_list.push(format!(
                    "{}\t|   {}",
                    &temp_branch_content.branch_name, &temp_branch_content.display_content
                ));
            }
            render_list.push(splitter);

            if !&donation_desc.is_empty() {
                render_list.push(donation_desc.clone());
            }

            println!("{}", render_list.join("\n"));
        }
    }
}
