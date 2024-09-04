#!/usr/bin/env sh

set -o errexit
set -o xtrace

cargo test --verbose --workspace
cargo test --verbose --workspace --all-features
cargo test --verbose --workspace --no-default-features
