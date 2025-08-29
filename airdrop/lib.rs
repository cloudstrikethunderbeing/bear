// src/airdrop/lib.rs

use ic_cdk::api::caller;
use ic_cdk_macros::*;
use ic_cdk::export::Principal;
use std::collections::HashMap;

thread_local! {
    static PARTICIPANTS: std::cell::RefCell<HashMap<Principal, u64>> = std::cell::RefCell::new(HashMap::new());
    static TREASURY: std::cell::RefCell<u64> = std::cell::RefCell::new(0);
    static ADMIN: Principal = Principal::from_text("pkt5m-vzera-uztne-or4se-vgejr-xajuz-ulw55-zdxon-3euz7-gvakp-5qe").unwrap();
}

#[update]
fn add_contribution(principal: Principal, icp: u64) {
    PARTICIPANTS.with(|p| {
        let mut map = p.borrow_mut();
        *map.entry(principal).or_insert(0) += icp;
    });
}

#[update]
fn set_treasury(amount: u64) {
    let caller = caller();
    ADMIN.with(|admin| {
        if &caller != admin {
            ic_cdk::trap("Only admin can set treasury");
        }
    });
    TREASURY.with(|t| *t.borrow_mut() = amount);
}

#[update]
fn monthly_airdrop() {
    let caller = caller();
    ADMIN.with(|admin| {
        if &caller != admin {
            ic_cdk::trap("Only admin can run airdrop");
        }
    });
    PARTICIPANTS.with(|p| {
        TREASURY.with(|t| {
            let mut map = p.borrow_mut();
            let mut treasury = t.borrow_mut();
            for (_principal, icp) in map.iter() {
                let payout = *icp; // 1:1 airdrop
                if *treasury >= payout {
                    // TODO: Call SNS ledger transfer here
                    *treasury -= payout;
                } else {
                    // TODO: Partial payout if not enough treasury
                    *treasury = 0;
                    break;
                }
            }
        });
    });
}

#[query]
fn get_participants() -> Vec<Principal> {
    PARTICIPANTS.with(|p| p.borrow().keys().cloned().collect())
}

#[query]
fn get_treasury() -> u64 {
    TREASURY.with(|t| *t.borrow())
}

#[query]
fn get_contribution(principal: Principal) -> u64 {
    PARTICIPANTS.with(|p| {
        *p.borrow().get(&principal).unwrap_or(&0)
    })
}
