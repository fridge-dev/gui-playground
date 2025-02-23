#!/usr/bin/env bash

set -euxo pipefail

cargo build --target wasm32-unknown-unknown --release

cp ./target/wasm32-unknown-unknown/release/caterpillar.wasm ./docs/
cp ./target/wasm32-unknown-unknown/release/mastermind.wasm ./docs/
cp ./target/wasm32-unknown-unknown/release/turn-time-tracker.wasm ./docs/
