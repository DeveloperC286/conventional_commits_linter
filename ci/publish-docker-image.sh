#!/usr/bin/env sh

set -o errexit
set -o xtrace

if [ $# -ne 4 ]; then
	echo "Usage: $0 <release> <platform> <target> <suffix>"
	exit 1
fi

RELEASE="$1"
PLATFORM="$2"
TARGET="$3"
SUFFIX="$4"

REPOSITORY="$(basename "$(git rev-parse --show-toplevel)")"
IMAGE="ghcr.io/developerc286/${REPOSITORY}"

# Download and extract pre-built binary from the GitHub release for this architecture.
gh release download "${RELEASE}" --pattern "${TARGET}.tar.gz"
mkdir -p "target/${TARGET}/release"
tar -xzf "${TARGET}.tar.gz" -C "target/${TARGET}/release"

# Build and push the Docker image for this architecture natively (no QEMU emulation).
docker buildx build --platform "${PLATFORM}" --build-arg TARGET="${TARGET}" --tag "${IMAGE}:${RELEASE}-${SUFFIX}" --file Dockerfile . --push
