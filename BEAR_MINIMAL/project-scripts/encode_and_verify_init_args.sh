#!/bin/bash
# Automate Candid argument encoding and verification for BEAR SNS launch
set -e

# Encode Candid argument
init_args_txt="bear-claim-canister/init_args_candid.txt"
init_args_blob="bear-claim-canister/init_args_blob.bin"
claim_did="src/declarations/bear-claim-canister/bear-claim-canister.did"

didc encode -d "$claim_did" -f blob < "$init_args_txt" > "$init_args_blob"

echo "Encoded argument blob to $init_args_blob"

# Verify blob
if didc decode -d "$claim_did" "$init_args_blob"; then
  echo "Blob verified. Ready for dfx deploy."
else
  echo "ERROR: Argument blob is invalid. Check init_args_candid.txt formatting."
  exit 1
fi
