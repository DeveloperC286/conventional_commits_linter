#!/usr/bin/env sh

set -o errexit
set -o xtrace

if [ $# -ne 1 ]; then
	echo "Usage: $0 <release>"
	exit 1
fi

RELEASE="$1"
REPOSITORY="$(basename "$(git rev-parse --show-toplevel)")"
IMAGE="ghcr.io/developerc286/${REPOSITORY}"

# Create and push the multi-architecture manifest.
docker buildx imagetools create --tag "${IMAGE}:${RELEASE}" \
	"${IMAGE}:${RELEASE}-amd64" \
	"${IMAGE}:${RELEASE}-arm64"

# Create alternate version tag (with/without 'v' prefix).
if [ "${RELEASE#v}" != "${RELEASE}" ]; then
	# Release has 'v' prefix (v1.2.3), also create version without 'v' prefix (1.2.3).
	docker buildx imagetools create --tag "${IMAGE}:${RELEASE#v}" "${IMAGE}:${RELEASE}"
else
	# Release has no 'v' prefix (1.2.3), also create version with 'v' prefix (v1.2.3).
	docker buildx imagetools create --tag "${IMAGE}:v${RELEASE}" "${IMAGE}:${RELEASE}"
fi
