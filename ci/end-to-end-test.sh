#!/usr/bin/env sh

set -o errexit
set -o xtrace

cd "end-to-end-tests/"
behave
