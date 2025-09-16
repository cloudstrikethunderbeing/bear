// scripts/submitProposal.js
// Script to help submit a proposal for SNS launch on NNS

// SNS Proposal Submission Script
// Requires: npm install @dfinity/agent @dfinity/nns

const fs = require('fs');
const path = require('path');
const { HttpAgent } = require('@dfinity/agent');
const { NNSGovernanceCanister, make_propose_to_open_sns_token_swap } = require('@dfinity/nns');

// Load config
const defaultConfigPath = path.join(__dirname, '../sns_config.json');
const config = JSON.parse(fs.readFileSync(defaultConfigPath, 'utf8'));

async function main() {
  // Set up agent
  const agent = new HttpAgent({
    host: 'https://ic0.app',
    // Add identity logic here (e.g., from PEM or hardware wallet)
  });

  // Prepare proposal payload
  const proposalPayload = make_propose_to_open_sns_token_swap({
    title: config.name,
    summary: config.description + '\nWebsite: ' + config.website,
    url: config.website,
    logo: config.logo_url,
    target_icp_e8s: config.icp_target * 1e8,
    // Add more fields as needed
  });

  // Submit proposal (requires identity)
  const governance = NNSGovernanceCanister.create({ agent });
  try {
    const result = await governance.submitProposal({
      proposal: proposalPayload,
      // Add principal/controller logic here
    });
    console.log('Proposal submitted! Result:', result);
  } catch (e) {
    console.error('Error submitting proposal:', e);
  }
}

main();
