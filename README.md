# 🐻 BEAR SNS Launch – Developer Setup Guide (VS Code)

This repo/setup will walk you through creating and submitting a Service Nervous System (SNS) launch proposal for the BEAR token.

We are using:
• SNS (to decentralize governance + raise ~$100k in ICP)
• NNS (to approve and spawn the SNS canisters)
• Your Principal ID → pkt5m-vzera-uztne-or4se-vgejr-xajuz-ulw55-zdxon-3euz7-gvakp-5qe
• Target raise: ~5,000 ICP (≈ $100k at $20/ICP)
• Use case: Fund RWA-backed projects + profit-share via BEAR buybacks

⸻

## Revenue Sharing & Buyback Model

- Contributors to the SNS Launchpad receive BEAR tokens proportional to their ICP contribution during the community sale (swap).
- 10% of all profits from RWA (Real World Asset) projects are used to buy back BEAR tokens from the market, benefiting all holders.
- All ICP raised (e.g., $100,000+) is used to fund RWA projects and DAO operations.

⸻

1. Prerequisites

Make sure you (or your dev) install:

# Install dfx and quill if not yet installed
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

dfx --version
quill --version

• You’ll also need:
  • An NNS neuron (on your Plug/NNS wallet) with voting power (this is used to submit the proposal).
  • Your principal and account IDs (already noted above).

⸻

2. Create sns_init.json

In your VS Code project root:

touch sns_init.json

Paste the following JSON into sns_init.json:

```json
{
  "dapp_canisters": [],
  "fallback_controller_principal_ids": [
    "pkt5m-vzera-uztne-or4se-vgejr-xajuz-ulw55-zdxon-3euz7-gvakp-5qe"
  ],
  "token_name": "BEAR Impact Ledger",
  "token_symbol": "BEAR",
  "token_logo": "https://bearimpact-pdv.caffeine.xyz/logo.png",
  "token_decimals": 8,
  "name": "BEAR SNS",
  "description": "Decentralized governance + fundraising for global RWA startups under the BEAR (Blockchain-Enabled Advancement Reserve) mission. 10% of RWA profits earmarked for BEAR buybacks. Contributors to the SNS Launchpad receive BEAR tokens proportional to their ICP contribution.",
  "url": "https://bearimpact-pdv.caffeine.xyz/",
  "governance_parameters": {
    "proposal_reject_cost_e8s": 100000000,
    "neuron_minimum_stake_e8s": 100000000,
    "neuron_management_fee_per_proposal_e8s": 0,
    "max_number_of_proposals_with_ballots": 1000,
    "wait_for_quiet_deadline_increase_seconds": 86400,
    "initial_voting_period_seconds": 432000
  },
  "initial_token_distribution": {
    "FractionalDeveloperVotingPower": {
      "developer_distribution": {
        "developer_neurons": [
          {
            "controller": "pkt5m-vzera-uztne-or4se-vgejr-xajuz-ulw55-zdxon-3euz7-gvakp-5qe",
            "stake_e8s": 1000000000000,
            "memo": 1,
            "dissolve_delay_seconds": 31536000,
            "vesting_period_seconds": 31536000
          }
        ]
      },
      "treasury_distribution": {
        "total_e8s": 2000000000000
      },
      "swap_distribution": {
        "total_e8s": 5000000000000
      },
      "airdrop_distribution": {
        "airdrop_neurons": [
          {
            "controller": "pkt5m-vzera-uztne-or4se-vgejr-xajuz-ulw55-zdxon-3euz7-gvakp-5qe",
            "stake_e8s": 50000000000,
            "memo": 10001,
            "dissolve_delay_seconds": 0,
            "vesting_period_seconds": 0
          }
        ]
      }
    }
  },
  "swap_parameters": {
    "minimum_participants": 50,
    "min_icp_e8s": 1000000000,
    "max_icp_e8s": 500000000000,
    "min_participant_icp_e8s": 100000000,
    "max_participant_icp_e8s": 5000000000,
    "start_timestamp_seconds": 0,
    "duration_seconds": 1209600,
    "neurons_fund_participation": "Unspecified"
  }
}
```

👉 Notes:
• max_icp_e8s = 500000000000 = 5,000 ICP hard cap (≈ $100k target).
• developer_neurons → locks BEAR for you with a 1-year vesting.
• treasury_distribution → DAO treasury BEAR pool.
• swap_distribution → portion sold in the SNS community sale (proportional to ICP contributed).
• airdrop_distribution → rewards early BEAR community (you can add more members here).

⸻

3. Generate Proposal with Quill

Open your terminal in VS Code:

# Replace with your actual neuron ID (NNS neuron, not principal)
export NEURON_ID=<YOUR_NNS_NEURON_ID>

# Generate proposal message
quill sns make-proposal \
  --canister-ids-file sns_init.json \
  $NEURON_ID > message.json

⸻

4. Submit Proposal

quill send message.json

If valid, you’ll see a response with a Proposal ID.
Your SNS proposal will then show up on the NNS Dashboard for community voting.

⸻

5. After Approval
• SNS canisters (governance, ledger, root, swap, index) are created.
• Sale begins per your swap_parameters.
• Community can participate using ICP or ckUSDC (if integrated).
• After success: funds → BEAR treasury, developers get neurons, BEAR ledger is live.
• 10% of RWA project profits are used for BEAR buybacks, benefiting all holders.

⸻

📌 Next Steps / To-Dos
• ✅ Confirm your NNS neuron ID (needed for step 3).
• ✅ Decide if you want multisig fallback controllers (add other principals to fallback_controller_principal_ids).
• ✅ Add your OpenChat community members to airdrop_distribution.
• ✅ Adjust max_icp_e8s to reflect current ICP price ($100k target).

⸻
