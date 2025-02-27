#!/usr/bin/env sh

set -o errexit
set -o xtrace

if [ $# -ne 1 ]; then
	echo "Usage: $0 <from>"
	exit 1
fi

clean_git_history "$1"
