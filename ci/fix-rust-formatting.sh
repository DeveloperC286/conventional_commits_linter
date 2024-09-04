#!/usr/bin/env sh

set -o errexit
set -o xtrace

cargo fmt --all -- --config=group_imports=StdExternalCrate
