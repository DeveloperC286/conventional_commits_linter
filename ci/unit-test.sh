#!/usr/bin/env sh

set -o errexit
set -o xtrace

cargo test --verbose --locked
