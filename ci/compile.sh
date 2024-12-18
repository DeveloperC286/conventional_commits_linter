#!/usr/bin/env sh

set -o errexit
set -o xtrace

cargo build --verbose --locked
