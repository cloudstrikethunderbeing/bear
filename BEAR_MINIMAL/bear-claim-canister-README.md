# BEAR Claim/Airdrop Canister — README

This repo ships a non‑custodial claim/airdrop canister for the BEAR SNS. It converts DAO‑funded allocations into time‑laddered neurons owned by the user via ICRC‑2 approvals and SNS Governance calls.

⸻

## Quick Start

### Build & Deploy (local)

```bash
rustup target add wasm32-unknown-unknown
cargo install ic-cdk-optimizer --locked

# Start local replica
dfx start --clean --background

# Deploy
dfx deploy bear_claim
```

dfx.json already contains an optimized build command using ic-cdk-optimizer.

⸻

## End‑to‑End Flow (UX)
1. DAO funds the pool (BEAR tokens) to the canister’s BEAR ICRC‑1 account, then calls `admin_fund_pool_from_treasury(amount)`.
2. Admin ingests:
   - `admin_ingest_snapshot(vec<SnapshotRow>)` (pre‑SNS holders)
   - `admin_ingest_contributions(vec<ContribRow>)` (SNS swap ICP)
3. Admin opens window: `admin_open_claims(start, end)`.
4. User opens dapp:
   - calls `prepare_claim()` → receives their ladder and a list of ICRC‑2 approval instructions.
   - frontend issues one `icrc2_approve` per ladder slot (spender = claim canister).
   - calls `finalize_all_safe()` (or `finalize_slot(i)`), which:
     - `icrc2_transfer_from` the slot amount from user → SNS governance staking subaccount
     - `manage_neuron(ClaimOrRefresh(MemoAndController))` to assign neuron to the user
     - increases dissolve delay to the slot’s target and enables auto‑stake maturity
5. User ends with 8 neurons unlocking at 6‑month intervals (default ladder).

⸻

## Canister Methods (User)
- `prepare_claim() -> PrepareClaimResp`
  - Returns `ladder[]` and `allowance_instructions[]` with: `{ slot_index, amount, spender, ledger }`.
- `finalize_slot(slot_index) -> Result<LadderSlot, String>`
- `finalize_all() -> Result<Vec<LadderSlot>, String>`
- `finalize_all_safe() -> Vec<{ slot_index, status: Result<LadderSlot, String> }>`

(Admin APIs are documented inside `claim.did` and `src/lib.rs`.)

⸻

## Frontend: ICRC‑2 approve + finalize (Plug / NFID / @dfinity/agent)

Replace the placeholders with your canister IDs.

```js
import { HttpAgent, Actor } from "@dfinity/agent";
import { idlFactory as ledgerIDL } from "./ledger.idl";        // ICRC ledger IDL
import { idlFactory as claimIDL } from "./claim.idl";          // this canister’s IDL

const LEDGER_ID = "<BEAR_SNS_LEDGER_CANISTER_ID>";             // e.g., from sns_init.json result
const CLAIM_ID  = "<BEAR_CLAIM_CANISTER_ID>";

const agent = new HttpAgent({ host: "https://icp0.io" });
const ledger = Actor.createActor(ledgerIDL, { agent, canisterId: LEDGER_ID });
const claim  = Actor.createActor(claimIDL,  { agent, canisterId: CLAIM_ID  });

// 1) Get ladder + instructions
const prep = await claim.prepare_claim();

// 2) Approve each slot amount (spender = claim canister)
for (const row of prep.allowance_instructions) {
  await ledger.icrc2_approve({
    from_subaccount: [],          // or your subaccount if you use one
    spender: { owner: CLAIM_ID, subaccount: [] },
    amount: row.amount,           // Nat (BEAR base units)
    expected_allowance: [],       // optional race‑safety
    expires_at: [],               // optional expiry
    fee: [],                      // use default
    memo: [],
    created_at_time: [BigInt(Date.now() * 1_000_000)],
  });
}

// 3) Stake all slots → neurons for the caller
const results = await claim.finalize_all_safe();
console.log(results);
```

NFID / Plug: use their provider injections to sign the agent. The approve & update calls are identical.

⸻

## Environment / Constants
- SNS Ledger: `state().config.sns_ledger` (ICRC‑1/2 BEAR ledger)
- SNS Governance: `state().config.sns_governance`
- Staking subaccount derivation (inside the canister):

  `sub = sha256("neuron-stake" || controller_principal_raw || memo_be_u64)`

Tokens move via `icrc2_transfer_from` into `(owner = sns_governance, subaccount = sub)`.

⸻

## Safety & Anti‑Sybil
- Per‑principal cap (`per_principal_max_tokens`)
- Claim window (`claim_start..claim_end`)
- Optional min BEAR stake
- II rate‑limit per day
- Pre‑SNS holder points use a compressed function (e.g., log10) to damp whales

⸻

## Testing

Run host tests:

```bash
cargo test -- --nocapture
```

Recommended extras:
- property tests for point normalization
- integration tests with a local SNS/ledger (mock canisters)

⸻

## Common Errors
- `claim window closed` — call during open window.
- `transfer_from err: InsufficientAllowance` — user didn’t approve enough.
- `claim response missing neuron_id` — governance didn’t return an id; check ledger transfer / memo / controller.

⸻

## Operational Playbook
1. DAO tops up pool → `admin_fund_pool_from_treasury`.
2. `admin_ingest_*` rows → `admin_open_claims`.
3. Users: `prepare_claim` → approvals → `finalize_all_safe`.
4. Close: `admin_close_claims`.
5. Payouts (optional module): `dist_execute_payout_icp` / `dist_execute_payout_bear` via DAO motions.

⸻

## Notes
- The scaffold uses non‑custodial neuron creation: users remain neuron controllers.
- Dissolve delays are applied per slot and auto‑stake maturity is enabled by default for compounding.
- Swap in precise 6‑month seconds or calendar‑month math if you prefer.
