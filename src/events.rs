///
#[derive(Debug, Clone)]
pub struct BalanceRefreshEvent {
    pub branch_name: String,
    pub display_content: String,
}

///
#[derive(Debug, Clone)]
pub struct DonationEvent {
    pub donor: String,
    pub donation_amount: usize,
}

///
#[derive(Debug, Clone)]
pub enum SyncingEventType {
    BranchBalanceUpdate(BalanceRefreshEvent),
    DonationUpdate(DonationEvent),
    DonationDone
}
