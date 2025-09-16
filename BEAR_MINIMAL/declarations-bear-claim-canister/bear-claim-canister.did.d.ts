import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface ClaimPreview {
  'per_principal_cap_tokens' : Tokens,
  'within_cap' : boolean,
  'ladder' : Array<LadderSlot>,
  'total_allocation' : Tokens,
  'points_breakdown' : {
    'total_points' : bigint,
    'holder_points' : bigint,
    'contributor_points' : bigint,
  },
}
export interface ContribRow { 'owner' : Principal, 'icp_e8s' : E8s }
export type E8s = bigint;
export interface InitConfig {
  'icp_usd_rate_microusd_per_icp' : Microusd,
  'min_bear_stake_required' : Tokens,
  'airdrop_pool_account' : Account,
  'weights' : { 'w_contrib' : number, 'w_holder' : number },
  'claim_end' : Timestamp,
  'icp_ledger' : [] | [Principal],
  'sns_root' : Principal,
  'per_principal_max_tokens' : Tokens,
  'sns_governance' : Principal,
  'claim_start' : Timestamp,
  'oc_public_key' : [] | [string],
  'ii_rate_limit_per_day' : number,
  'sns_ledger' : Principal,
}
export interface LadderSlot {
  'status' : { 'Claimed' : null } |
    { 'Staked' : null } |
    { 'Ready' : null } |
    { 'Pending' : null },
  'dissolve_delay_seconds' : bigint,
  'slot_index' : number,
  'amount' : Tokens,
  'neuron_id' : [] | [bigint],
}
export type Microusd = bigint;
export interface SnapshotRow { 'owner' : Principal, 'bear_tokens' : Tokens }
export type Timestamp = bigint;
export type Tokens = bigint;
export interface _SERVICE {
  'admin_close_claims' : ActorMethod<[], undefined>,
  'admin_fund_pool_from_treasury' : ActorMethod<[Tokens], undefined>,
  'admin_ingest_contributions' : ActorMethod<[Array<ContribRow>], undefined>,
  'admin_ingest_snapshot' : ActorMethod<[Array<SnapshotRow>], undefined>,
  'admin_open_claims' : ActorMethod<[Timestamp, Timestamp], undefined>,
  'admin_set_acl' : ActorMethod<[Array<Principal>], undefined>,
  'admin_set_params' : ActorMethod<[InitConfig], undefined>,
  'dist_execute_payout_bear' : ActorMethod<
    [Tokens],
    { 'ok' : null } |
      { 'err' : string }
  >,
  'dist_execute_payout_icp' : ActorMethod<
    [E8s],
    { 'ok' : null } |
      { 'err' : string }
  >,
  'dist_register_shares_from_contribs' : ActorMethod<[], undefined>,
  'finalize_all' : ActorMethod<
    [],
    { 'ok' : Array<LadderSlot> } |
      { 'err' : string }
  >,
  'finalize_slot' : ActorMethod<
    [number],
    { 'ok' : LadderSlot } |
      { 'err' : string }
  >,
  'get_status' : ActorMethod<
    [],
    {
      'total_points' : bigint,
      'claim_window' : { 'end_' : Timestamp, 'start' : Timestamp },
      'claimed_count' : number,
      'pool_balance' : Tokens,
    }
  >,
  'has_claimed' : ActorMethod<[Principal], boolean>,
  'init' : ActorMethod<[InitConfig], undefined>,
  'prepare_claim' : ActorMethod<
    [],
    {
      'ladder' : Array<LadderSlot>,
      'allowance_instructions' : Array<
        {
          'ledger' : Principal,
          'slot_index' : number,
          'amount' : Tokens,
          'spender' : Principal,
        }
      >,
      'total_allocation' : Tokens,
    }
  >,
  'preview_claim' : ActorMethod<[], ClaimPreview>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
