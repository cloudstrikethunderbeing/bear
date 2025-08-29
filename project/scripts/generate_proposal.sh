#!/bin/bash
# Helper script to generate SNS proposal message with Quill
# Usage: ./scripts/generate_proposal.sh <YOUR_NNS_NEURON_ID>

NEURON_ID=2306615
quill sns make-proposal \
  --canister-ids-file sns_init.json \
  $NEURON_ID > message.json

echo "Proposal message generated as message.json."
