# This is a simple utility to help containerize a protonmail-bridge

# Install

1. Install rustup from https://rustup.rs/
2. Once rust has installed, run `cargo install imposter-pass`

# Usage
1. Make sure that protonmail-bridge is not running
2. Run
   ```bash
   imposter-pass --store store.json fool protonmail-bridge
   ```
   This will cause `protonmail-bridge` to run through the authentication process as usual. The credentials will be saved to store.json.
3. You can now place the contents on `store.json` in the environment variable 
   ```bash
   IMPOSTER_PASS_STORE="$(cat store.json)" imposter-pass [pass cmd]
   ```
   or use 
   ```bash
   imposter-pass --store store.json [pass cmd]
   ```
   to interact with `imposter-pass` as you would with `pass [pass cmd]`.
4. You can use
   ```bash
   IMPOSTER_PASS_STORE="$(cat store.json)" imposter-pass fool [exec] -- [args...]
   ```
   or
   ```bash
   imposter-pass --store store.json fool [exec] -- [args...]
   ```
   to launch `exec [args...]` with the saved credentials.
