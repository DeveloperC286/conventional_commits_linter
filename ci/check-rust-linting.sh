#!/usr/bin/env sh

set -o errexit
set -o xtrace

cargo clippy --verbose --locked -- -D warnings
