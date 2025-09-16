#!/bin/bash
# Clean BEAR repo for ICP Ninja import (SNS launch)
# Keeps only essential source, configs, Candid, and scripts
# Usage: bash cleanup_for_icp_ninja.sh

set -e

# Create minimal export folder
EXPORT_DIR="BEAR_MINIMAL"
rm -rf "$EXPORT_DIR"
mkdir "$EXPORT_DIR"

# Copy essential files
cp dfx.json "$EXPORT_DIR/"
cp canister_ids.json "$EXPORT_DIR/"
cp -r bear-claim-canister/src "$EXPORT_DIR/bear-claim-canister-src"
cp bear-claim-canister/Cargo.toml "$EXPORT_DIR/bear-claim-canister-Cargo.toml"
cp bear-claim-canister/README.md "$EXPORT_DIR/bear-claim-canister-README.md"
cp bear-claim-canister/claim.did "$EXPORT_DIR/bear-claim-canister-claim.did"
cp bear-claim-canister/init_args_candid.txt "$EXPORT_DIR/bear-claim-canister-init_args_candid.txt"
cp bear-claim-canister/minimal_init_args_candid.txt "$EXPORT_DIR/bear-claim-canister-minimal_init_args_candid.txt"
cp -r project/scripts "$EXPORT_DIR/project-scripts"
cp project/sns_config.json "$EXPORT_DIR/project-sns_config.json"
cp project/sns_init.json "$EXPORT_DIR/project-sns_init.json"
cp project/README.md "$EXPORT_DIR/project-README.md"
cp -r src/declarations/bear-claim-canister "$EXPORT_DIR/declarations-bear-claim-canister"

# Optionally copy other .did/.md/.sh files
find . -maxdepth 1 -type f \( -name '*.did' -o -name '*.md' -o -name '*.sh' \) -exec cp {} "$EXPORT_DIR/" \;

# Print result
echo "Minimal export created in $EXPORT_DIR. Ready for GitHub import (<5MB)."
