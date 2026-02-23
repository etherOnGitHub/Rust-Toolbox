# Rust Toolbox

Modular Rust workspace scaffold.

## Layout

- `apps/cli`: binary entrypoint
- `crates/config`: runtime/environment config types
- `crates/core`: domain + application logic
- `crates/storage`: persistence adapters
- `crates/utils`: shared helper utilities

## Commands

- `cargo check --workspace`
- `cargo run -p rtb-cli`
