#!/usr/bin/env bash

set -o errexit
set -o xtrace

yamlfmt -lint -dstar .github/**/*
