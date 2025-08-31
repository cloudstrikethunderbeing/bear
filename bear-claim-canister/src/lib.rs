use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::{call::call, time};
use ic_cdk::caller;
use ic_cdk_macros::*;
use once_cell::sync::Lazy;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

// ===== Aliases / Units =====
// BEAR uses 8 decimals (base unit). Prefer u128 for token math.
pub type Microusd = u64;    // USD * 1e6
pub type Tokens   = u128;   // BEAR base units
pub type E8s      = u64;    // ICP e8s
pub type Timestamp= u64;    // seconds

#[derive(Clone, CandidType, Deserialize, Serialize, Debug)]
pub struct Account { pub owner: Principal, pub subaccount: Option<Vec<u8>> }

impl Default for Account {
    fn default() -> Self {
        Self { owner: Principal::anonymous(), subaccount: None }
    }
}

#[derive(Clone, CandidType, Deserialize, Serialize, Debug)]
pub struct SnapshotRow { pub owner: Principal, pub bear_tokens: Tokens }

impl Default for SnapshotRow {
    fn default() -> Self {
        Self { owner: Principal::anonymous(), bear_tokens: 0 }
    }
}

#[derive(Clone, CandidType, Deserialize, Serialize, Debug)]
pub struct ContribRow  { pub owner: Principal, pub icp_e8s: E8s }

impl Default for ContribRow {
    fn default() -> Self {
        Self { owner: Principal::anonymous(), icp_e8s: 0 }
    }
}

#[derive(Clone, CandidType, Deserialize, Serialize, Debug)]
pub enum SlotStatus { Pending, Ready, Staked, Claimed }

impl Default for SlotStatus {
    fn default() -> Self { SlotStatus::Pending }
}

#[derive(Clone, CandidType, Deserialize, Serialize, Debug)]
pub struct LadderSlot {
    pub slot_index: u8,
    pub dissolve_delay_seconds: u64,
    pub amount: Tokens,
    pub status: SlotStatus,
    pub neuron_id: Option<u64>,
}

impl Default for LadderSlot {
    fn default() -> Self {
        Self {
            slot_index: 0,
            dissolve_delay_seconds: 0,
            amount: 0,
            status: SlotStatus::default(),
            neuron_id: None,
        }
    }
}

#[derive(Clone, CandidType, Deserialize, Serialize, Debug, Default)]
pub struct ClaimPreview {
    pub total_allocation: Tokens,
    pub ladder: Vec<LadderSlot>,
    pub points_breakdown: PointsBreakdown,
    pub per_principal_cap_tokens: Tokens,
    pub within_cap: bool,
}

#[derive(Clone, CandidType, Deserialize, Serialize, Debug, Default)]
pub struct PointsBreakdown { pub holder_points: u64, pub contributor_points: u64, pub total_points: u64 }

#[derive(Clone, CandidType, Deserialize, Serialize, Debug)]
pub struct InitConfig {
    pub sns_root: Principal,
    pub sns_governance: Principal,
    pub sns_ledger: Principal,
    pub icp_ledger: Option<Principal>,
    pub airdrop_pool_account: Account,
    pub icp_usd_rate_microusd_per_icp: Microusd,
    pub claim_start: Timestamp,
    pub claim_end: Timestamp,
    pub per_principal_max_tokens: Tokens,
    pub min_bear_stake_required: Tokens,
    pub ii_rate_limit_per_day: u32,
    pub weights: Weights,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            sns_root: Principal::anonymous(),
            sns_governance: Principal::anonymous(),
            sns_ledger: Principal::anonymous(),
            icp_ledger: None,
            airdrop_pool_account: Account::default(),
            icp_usd_rate_microusd_per_icp: 0,
            claim_start: 0,
            claim_end: 0,
            per_principal_max_tokens: 0,
            min_bear_stake_required: 0,
            ii_rate_limit_per_day: 0,
            weights: Weights::default(),
        }
    }
}

#[derive(Clone, CandidType, Deserialize, Serialize, Debug, Default)]
pub struct Weights { pub w_holder: u32, pub w_contrib: u32 }

#[derive(CandidType, Deserialize, Serialize, Debug, Default, Clone)]
pub struct ClaimRecord {
    pub total_allocation: Tokens,
    pub ladder: Vec<LadderSlot>,
    pub claimed_slots: BTreeSet<u8>,
    pub last_claim_ts: Option<Timestamp>,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Default, Clone)]
pub struct State {
    pub admins: BTreeSet<Principal>,
    pub config: InitConfig,

    // Pools & windows
    pub pool_balance: Tokens,
    pub claim_window: (Timestamp, Timestamp),

    // Data sources
    pub snapshot: BTreeMap<Principal, Tokens>,
    pub contribs: BTreeMap<Principal, E8s>,

    // Derived
    pub total_points: u64,
    pub user_points: BTreeMap<Principal, u64>,

    // User-state
    pub claims: BTreeMap<Principal, ClaimRecord>,
    pub claimed_count: u32,
}

static mut STATE: Option<State> = None;
static INIT_DONE: Lazy<std::sync::atomic::AtomicBool> = Lazy::new(|| std::sync::atomic::AtomicBool::new(false));

fn state() -> &'static mut State {
    unsafe { STATE.as_mut().expect("STATE not initialized") }
}

fn is_admin(p: Principal) -> bool { state().admins.contains(&p) }
fn require_admin() {
    if !is_admin(caller()) { ic_cdk::trap("Admin only"); }
}

fn now() -> Timestamp { (time() / 1_000_000_000) as u64 }

// ===== Lifecycle =====
#[init]
fn init(cfg: InitConfig) {
    if INIT_DONE.swap(true, std::sync::atomic::Ordering::SeqCst) {
        ic_cdk::trap("Already initialized");
    }
    let mut s = State::default();
    s.config = cfg.clone();
    s.admins.insert(caller()); // bootstrap: caller is admin; rotate via admin_set_acl
    s.pool_balance = 0;
    s.claim_window = (cfg.claim_start, cfg.claim_end);
    unsafe { STATE = Some(s); }
}

#[pre_upgrade]
fn pre_upgrade() {
    let s = state();
    ic_cdk::storage::stable_save::<(State,)>( (s.clone(),) ).expect("stable_save failed");
}

#[post_upgrade]
fn post_upgrade() {
    let (loaded,): (State,) = ic_cdk::storage::stable_restore().unwrap_or_default();
    unsafe { STATE = Some(loaded); }
    INIT_DONE.store(true, std::sync::atomic::Ordering::SeqCst);
}

// ===== Admin APIs =====
#[update]
fn admin_set_params(cfg: InitConfig) {
    require_admin();
    state().config = cfg.clone();
    state().claim_window = (cfg.claim_start, cfg.claim_end);
}

#[update]
fn admin_fund_pool_from_treasury(amount: Tokens) {
    require_admin();
    // NOTE: In production, you will verify an incoming ICRC-1 transfer into the canister's BEAR account
    // and then increment pool_balance accordingly. Here we just add for scaffold.
    state().pool_balance = state().pool_balance.saturating_add(amount);
}

#[update]
fn admin_ingest_snapshot(rows: Vec<SnapshotRow>) {
    require_admin();
    for r in rows { state().snapshot.insert(r.owner, r.bear_tokens); }
}

#[update]
fn admin_ingest_contributions(rows: Vec<ContribRow>) {
    require_admin();
    for r in rows { state().contribs.insert(r.owner, r.icp_e8s); }
}

#[update]
fn admin_close_claims() { require_admin(); state().claim_window.1 = now(); }

#[update]
fn admin_open_claims(start: Timestamp, end_: Timestamp) {
    require_admin(); state().claim_window = (start, end_);
}

#[update]
fn admin_set_acl(admins: Vec<Principal>) {
    require_admin();
    state().admins = admins.into_iter().collect();
}

// ===== Internal calc helpers (stubs / illustrative) =====
fn log10_like(x: Tokens) -> u64 {
    // Softly compress large balances; simple ln approximation can suffice; here: count digits
    let mut n = x; let mut d = 0u64; if n == 0 { return 0; }
    while n > 0 { n /= 10; d += 1; }
    d
}

fn icp_usd(icp_e8s: E8s, rate_micro_usd_per_icp: Microusd) -> u128 {
    // (icp_e8s / 1e8) * (rate / 1e6) in USD
    let icp = icp_e8s as u128; // e8s
    let rate = rate_micro_usd_per_icp as u128; // micro USD per ICP
    icp * rate / 100_000_000u128 // => microUSD
}

fn compute_points(p: Principal) -> (u64, u64, u64) {
    let s = state();
    let holder = s.snapshot.get(&p).cloned().unwrap_or_default();
    let holder_pts = log10_like(holder);

    let icp_e8s = s.contribs.get(&p).cloned().unwrap_or_default();
    let micro_usd = icp_usd(icp_e8s, s.config.icp_usd_rate_microusd_per_icp); // microUSD
    let usd = (micro_usd / 1_000_000u128) as u64;
    let base = usd.min(100_000);
    let excess = usd.saturating_sub(100_000);
    let contrib_pts = base + 2 * excess;

    let w1 = s.config.weights.w_holder as u64;
    let w2 = s.config.weights.w_contrib as u64;
    let total = w1 * holder_pts + w2 * contrib_pts;
    (holder_pts, contrib_pts, total)
}

fn default_ladder(total: Tokens) -> Vec<LadderSlot> {
    let parts = 8u128;
    let per = total / parts;
    (0..8).map(|i| LadderSlot{
        slot_index: i as u8,
        dissolve_delay_seconds: 6 * 30 * 24 * 60 * 60 * ((i as u64)+1), // approx 6m steps
        amount: per,
        status: SlotStatus::Pending,
        neuron_id: None,
    }).collect()
}

// ===== User APIs =====
#[query]
fn preview_claim() -> ClaimPreview {
    let p = caller();
    let s = state();
    let now = now();
    let (_start, end_) = s.claim_window;
    let (_hp, _cp, tp) = compute_points(p);

    // This scaffold omits global normalization; a real impl computes sum(points) over all users.
    // Here, we just show an indicative per-principal allocation capped by per_principal_max_tokens.
    let indicative = (s.pool_balance / 100) as Tokens; // placeholder: 1% indicative
    let capped = indicative.min(s.config.per_principal_max_tokens);

    let ladder = default_ladder(capped);

    ClaimPreview{
        total_allocation: capped,
        ladder,
        points_breakdown: PointsBreakdown{ holder_points: 0, contributor_points: 0, total_points: tp },
        per_principal_cap_tokens: s.config.per_principal_max_tokens,
        within_cap: capped <= s.config.per_principal_max_tokens,
    }
}

#[query]
fn prepare_claim() -> candid::types::reference::Func {
    // In a production implementation, return a rich struct with allowance instructions per slot.
    // We return dummy data here due to space; see DID for intended shape.
    ic_cdk::trap("prepare_claim: scaffold placeholder — return instructions including spender = this canister, ledger = SNS ledger");
}

#[update]
fn finalize_slot(slot_index: u8) -> Result<LadderSlot, String> {
    // TODO: Implement non-custodial flow using ICRC-2 approve + transfer_from + SNS governance stake
    Err("finalize_slot not implemented in scaffold; wire ledger/governance calls".into())
}

#[update]
fn finalize_all() -> Result<Vec<LadderSlot>, String> {
    // TODO batch finalize calling finalize_slot for 0..7
    Err("finalize_all not implemented in scaffold".into())
}

#[query]
fn has_claimed(of: Principal) -> bool {
    state().claims.get(&of).map(|r| r.claimed_slots.len() == 8).unwrap_or(false)
}

#[query]
fn get_status() -> candid::Nat {
    // Minimal placeholder: return pool balance as Nat
    candid::Nat::from(state().pool_balance)
}

// ===== Distribution APIs (stubs) =====
#[update]
fn dist_register_shares_from_contribs() { require_admin(); /* compute & store shares */ }

#[update]
fn dist_execute_payout_icp(_amount_e8s: E8s) -> Result<(), String> { require_admin(); Err("payout ICP not implemented".into()) }

#[update]
fn dist_execute_payout_bear(_amount: Tokens) -> Result<(), String> { require_admin(); Err("payout BEAR not implemented".into()) }

// ===== Export Candid =====
#[cfg(feature = "export_candid")]
#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String { ic_cdk::export_candid!() }
