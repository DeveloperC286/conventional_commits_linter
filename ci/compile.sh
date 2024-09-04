#!/usr/bin/env sh

set -o errexit
set -o xtrace

cargo build --verbose --workspace
cargo build --verbose --workspace --all-features
cargo build --verbose --workspace --no-default-features
