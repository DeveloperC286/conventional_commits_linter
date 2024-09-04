#!/usr/bin/env sh

set -o errexit
set -o xtrace

cargo clippy --verbose --workspace --all-targets -- -D warnings
cargo clippy --verbose --workspace --all-targets --all-features -- -D warnings
cargo clippy --verbose --workspace --all-targets --no-default-features -- -D warnings
