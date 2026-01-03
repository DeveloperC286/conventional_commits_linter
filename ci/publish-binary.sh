#!/usr/bin/env sh

set -o errexit
set -o xtrace

if [ "$#" -ne 2 ]; then
	echo "Usage: $0 RELEASE_TAG TARGET"
	echo "$#"
	exit 1
fi

RELEASE="$1"
TARGET="$2"

tar -czvf "${TARGET}.tar.gz" -C "target/${TARGET}/release" "conventional_commits_linter"
gh release upload "${RELEASE}" "${TARGET}.tar.gz"
rm "${TARGET}.tar.gz"
