#!/usr/bin/env sh

set -o errexit
set -o xtrace

actionlint --verbose .github/workflows/*
