#![allow(warnings)]
mod branch;
mod charity_account;
mod headquarter_office;

///
fn main() {
    use branch::Branch;
    use charity_account::InternationalCharityAccount;
    use headquarter_office::HeadquarterOffice;
    use std::sync::{Arc, RwLock};

    // Singleton charity account instance
    let account = InternationalCharityAccount::new();
    let global_lock = Arc::new(RwLock::new(account));

    // Create branch thread
    let us_branch = Branch::new("US - New York", Arc::clone(&global_lock));
    let nz_branch = Branch::new("NZ - Auckland", Arc::clone(&global_lock));
    let cn_branch = Branch::new("CN - Shang Hai", Arc::clone(&global_lock));

    HeadquarterOffice::sync_balance_from_all_branches(vec![us_branch, nz_branch, cn_branch]);
}
