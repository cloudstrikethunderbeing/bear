// scripts/airdrop.js
// Node.js script to trigger monthly $BEAR airdrop and monitor status
// Requires: npm install @dfinity/agent @dfinity/candid

const { HttpAgent, Actor } = require('@dfinity/agent');
const { idlFactory } = require('../.dfx/local/canisters/airdrop/airdrop.did.js'); // Adjust path for mainnet
const canisterId = 'YOUR_AIRDROP_CANISTER_ID'; // Replace with deployed canister ID

async function main() {
  const agent = new HttpAgent({ host: 'https://ic0.app' });
  // Optionally: await agent.fetchRootKey(); // For local dev only

  const airdrop = Actor.createActor(idlFactory, { agent, canisterId });

  // Trigger monthly airdrop
  try {
    await airdrop.monthlyAirdrop();
    console.log('Monthly airdrop triggered!');
  } catch (e) {
    console.error('Airdrop failed:', e);
  }

  // Monitor treasury and participants
  try {
    const treasury = await airdrop.treasury();
    const participants = await airdrop.participants();
    console.log('Treasury:', treasury.toString());
    console.log('Participants:', participants);
  } catch (e) {
    console.error('Status fetch failed:', e);
  }
}

main();
