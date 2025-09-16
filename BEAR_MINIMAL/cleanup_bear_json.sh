#!/bin/bash
# Cleanup duplicate and legacy JSON files for BEAR SNS launch
set -e

# Keep only these JSON files
KEEP_FILES=(
  "dfx.json"
  "canister_ids.json"
  "project/sns_config.json"
  "project/sns_init.json"
)

# Find all JSON files except in .dfx, target, node_modules, and build
find . -type f -name '*.json' \
  ! -path './.dfx/*' \
  ! -path './bear-claim-canister/target/*' \
  ! -path './node_modules/*' \
  ! -path './project/web/build/*' \
  ! -path './project/web/node_modules/*' \
  ! -path './project/web/project/web/build/*' \
  | while read f; do
    # If not in KEEP_FILES, delete
    keep=false
    for k in "${KEEP_FILES[@]}"; do
      [[ "$f" == ./$k ]] && keep=true && break
    done
    if ! $keep; then
      echo "Deleting $f"
      rm "$f"
    fi
  done

echo "JSON cleanup complete. Only main configs remain."
