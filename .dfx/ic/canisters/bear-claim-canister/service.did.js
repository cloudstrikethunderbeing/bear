export const idlFactory = ({ IDL }) => {
  const Tokens = IDL.Nat;
  const E8s = IDL.Nat64;
  const ContribRow = IDL.Record({ 'owner' : IDL.Principal, 'icp_e8s' : E8s });
  const SnapshotRow = IDL.Record({
    'owner' : IDL.Principal,
    'bear_tokens' : Tokens,
  });
  const Timestamp = IDL.Nat64;
  const Microusd = IDL.Nat64;
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const InitConfig = IDL.Record({
    'icp_usd_rate_microusd_per_icp' : Microusd,
    'min_bear_stake_required' : Tokens,
    'airdrop_pool_account' : Account,
    'weights' : IDL.Record({ 'w_contrib' : IDL.Nat32, 'w_holder' : IDL.Nat32 }),
    'claim_end' : Timestamp,
    'icp_ledger' : IDL.Opt(IDL.Principal),
    'sns_root' : IDL.Principal,
    'per_principal_max_tokens' : Tokens,
    'sns_governance' : IDL.Principal,
    'claim_start' : Timestamp,
    'oc_public_key' : IDL.Opt(IDL.Text),
    'ii_rate_limit_per_day' : IDL.Nat32,
    'sns_ledger' : IDL.Principal,
  });
  const LadderSlot = IDL.Record({
    'status' : IDL.Variant({
      'Claimed' : IDL.Null,
      'Staked' : IDL.Null,
      'Ready' : IDL.Null,
      'Pending' : IDL.Null,
    }),
    'dissolve_delay_seconds' : IDL.Nat64,
    'slot_index' : IDL.Nat8,
    'amount' : Tokens,
    'neuron_id' : IDL.Opt(IDL.Nat64),
  });
  const ClaimPreview = IDL.Record({
    'per_principal_cap_tokens' : Tokens,
    'within_cap' : IDL.Bool,
    'ladder' : IDL.Vec(LadderSlot),
    'total_allocation' : Tokens,
    'points_breakdown' : IDL.Record({
      'total_points' : IDL.Nat64,
      'holder_points' : IDL.Nat64,
      'contributor_points' : IDL.Nat64,
    }),
  });
  return IDL.Service({
    'admin_close_claims' : IDL.Func([], [], []),
    'admin_fund_pool_from_treasury' : IDL.Func([Tokens], [], []),
    'admin_ingest_contributions' : IDL.Func([IDL.Vec(ContribRow)], [], []),
    'admin_ingest_snapshot' : IDL.Func([IDL.Vec(SnapshotRow)], [], []),
    'admin_open_claims' : IDL.Func([Timestamp, Timestamp], [], []),
    'admin_set_acl' : IDL.Func([IDL.Vec(IDL.Principal)], [], []),
    'admin_set_params' : IDL.Func([InitConfig], [], []),
    'dist_execute_payout_bear' : IDL.Func(
        [Tokens],
        [IDL.Variant({ 'ok' : IDL.Null, 'err' : IDL.Text })],
        [],
      ),
    'dist_execute_payout_icp' : IDL.Func(
        [E8s],
        [IDL.Variant({ 'ok' : IDL.Null, 'err' : IDL.Text })],
        [],
      ),
    'dist_register_shares_from_contribs' : IDL.Func([], [], []),
    'finalize_all' : IDL.Func(
        [],
        [IDL.Variant({ 'ok' : IDL.Vec(LadderSlot), 'err' : IDL.Text })],
        [],
      ),
    'finalize_slot' : IDL.Func(
        [IDL.Nat8],
        [IDL.Variant({ 'ok' : LadderSlot, 'err' : IDL.Text })],
        [],
      ),
    'get_status' : IDL.Func(
        [],
        [
          IDL.Record({
            'total_points' : IDL.Nat64,
            'claim_window' : IDL.Record({
              'end_' : Timestamp,
              'start' : Timestamp,
            }),
            'claimed_count' : IDL.Nat32,
            'pool_balance' : Tokens,
          }),
        ],
        ['query'],
      ),
    'has_claimed' : IDL.Func([IDL.Principal], [IDL.Bool], ['query']),
    'init' : IDL.Func([InitConfig], [], []),
    'prepare_claim' : IDL.Func(
        [],
        [
          IDL.Record({
            'ladder' : IDL.Vec(LadderSlot),
            'allowance_instructions' : IDL.Vec(
              IDL.Record({
                'ledger' : IDL.Principal,
                'slot_index' : IDL.Nat8,
                'amount' : Tokens,
                'spender' : IDL.Principal,
              })
            ),
            'total_allocation' : Tokens,
          }),
        ],
        ['query'],
      ),
    'preview_claim' : IDL.Func([], [ClaimPreview], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
