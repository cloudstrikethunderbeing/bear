# BEAR Project Canister Details

## Canister: airdrop
- **Type:** Rust
- **Directory:** `airdrop/`
- **Main Source:** `airdrop/lib.rs`
- **Candid Interface:** `airdrop/airdrop.did`
- **Cargo Manifest:** `airdrop/Cargo.toml`
- **dfx.json config:**
  ```json
  "airdrop": {
    "type": "rust",
    "main": "airdrop/lib.rs",
    "candid": "airdrop/airdrop.did",
    "package": "airdrop"
  }
  ```
- **Deployment:**
  - Run from workspace root: `dfx deploy airdrop --network ic`
  - Requires sufficient cycles on mainnet
- **Canister ID (last deployed):** See `canister_ids.json` (auto-generated)

## Canister: bear_frontend
- **Type:** Assets
- **Directory:** `project/src/www/assets/`
- **dfx.json config:**
  ```json
  "bear_frontend": {
    "type": "assets",
    "source": ["src/www/assets"],
    "controller": ["cloudstrikethunderbeing"]
  }
  ```

## Deployment Notes
- Always run dfx commands from the workspace root (`/Users/justinjackbear/BEAR`).
- For Rust canisters, the canister directory (with `Cargo.toml`, `lib.rs`, and `.did`) must be at the workspace root.
- The `canister_ids.json` file is generated on deployment and contains the latest canister IDs.
- If you encounter build errors, check for:
  - Correct dfx version (0.14+ recommended)
  - No duplicate or stray canister directories
  - File permissions and encoding

## SNS & Scripts
- SNS proposal and airdrop scripts are in `project/scripts/`
- Web frontend is in `project/web/`

---

For more details, see the `dfx.json` and canister source files.
