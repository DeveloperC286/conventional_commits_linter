#!/usr/bin/env sh

set -o errexit
set -o xtrace

cargo fmt --all -- --check --config=group_imports=StdExternalCrate
