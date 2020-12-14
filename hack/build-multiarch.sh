#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

PRJDIR=$(readlink -f "$(dirname "${BASH_SOURCE[0]}")/..")
PLATFORMS=${PLATFORMS:-linux/arm64}
MAKE_TARGET=${MAKE_TARGET:-build}
OUTPUT_DIR=${OUTPUT_DIR:-$PRJDIR/output}

function setup() {
  docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
  docker buildx create --name builder --driver-opt image=moby/buildkit:master
  docker buildx inspect builder --bootstrap
  docker buildx use builder
}

function cleanup() {
  docker buildx rm builder
}

function build() {
  docker buildx build --platform "$PLATFORMS" --build-arg="MAKE_TARGET=$MAKE_TARGET" --output="type=local,dest=$OUTPUT_DIR" -t huber_build:latest -f "$PRJDIR"/Dockerfile.build .
  echo "done"
}

trap cleanup EXIT ERR INT TERM

setup
build
