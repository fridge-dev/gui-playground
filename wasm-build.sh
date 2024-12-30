#!/usr/bin/env bash

set -euxo pipefail

cargo build --target wasm32-unknown-unknown --release
