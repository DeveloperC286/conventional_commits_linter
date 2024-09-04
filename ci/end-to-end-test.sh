#!/usr/bin/env sh

set -o errexit
set -o xtrace

cd "conventional_commits_linter/end-to-end-tests/"
behave
