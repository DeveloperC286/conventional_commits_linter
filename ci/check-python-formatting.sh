#!/usr/bin/env sh

set -o errexit
set -o xtrace

find "end-to-end-tests/features/" -type f | grep "[.]py$" | xargs -I {} autopep8 --exit-code --diff --aggressive --aggressive --max-line-length 120 "{}"
