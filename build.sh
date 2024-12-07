#!/usr/bin/env bash

set -euxo pipefail

cargo fmt --check --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build
cargo test
