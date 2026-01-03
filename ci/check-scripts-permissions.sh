#!/usr/bin/env sh

set -o errexit
set -o xtrace

exit_code=0
for script in ci/*.sh; do
	if [ -f "$script" ] && [ ! -x "$script" ]; then
		exit_code=1
	fi
done

exit $exit_code
