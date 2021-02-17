#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

PRJDIR=$(readlink -f "$(dirname "${BASH_SOURCE[0]}")/..")

# linux/amd64, linux/riscv64, linux/ppc64le, linux/s390x, linux/386, linux/mips64le, linux/mips64, linux/arm/v7, linux/arm/v6, linux/arm64 supported in `docker buildx`
PLATFORMS=${PLATFORMS:-linux/arm64}

BUILD_TARGET=${BUILD_TARGET:-debug}
MAKE_TARGET=${MAKE_TARGET:-build}
OUTPUT_DIR=${OUTPUT_DIR:-$PRJDIR/.output}

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
  docker buildx build \
    --platform "$PLATFORMS" \
    --build-arg="MAKE_TARGET=$MAKE_TARGET" \
    --build-arg="BUILD_TARGET=$BUILD_TARGET" \
    --output="type=local,dest=$OUTPUT_DIR" \
    -t huber_build:latest \
    -f "$PRJDIR"/Dockerfile.build .
}

case $1 in
setup | cleanup | build)
  $1
  ;;
*)
  trap cleanup EXIT ERR INT TERM
  setup
  build
  ;;
esac
