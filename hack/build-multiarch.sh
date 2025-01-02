#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

PRJ_DIR=$(readlink -f "$(dirname "${BASH_SOURCE[0]}")/..")
BUILD_TARGET=${BUILD_TARGET:-debug}
JUST_TARGET=${JUST_TARGET:-build}
OUTPUT_DIR=${OUTPUT_DIR:-$PRJ_DIR/.output}

# linux/amd64, linux/riscv64, linux/ppc64le, linux/s390x, linux/386, linux/mips64le, linux/mips64, linux/arm/v7, linux/arm/v6, linux/arm64 supported in `docker buildx`
PLATFORMS=${PLATFORMS:-linux/arm64}

setup() {
  docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
  docker buildx create --name builder --driver-opt image=moby/buildkit:master
  docker buildx inspect builder --bootstrap
  docker buildx use builder
}

cleanup() {
  docker buildx rm builder
}

build() {
  docker buildx build \
    --platform "$PLATFORMS" \
    --build-arg="JUST_TARGET=$JUST_TARGET" \
    --build-arg="BUILD_TARGET=$BUILD_TARGET" \
    --output="type=local,dest=$OUTPUT_DIR" \
    -t huber_build:latest \
    -f "$PRJ_DIR"/Dockerfile.build .
}

if [[ $# -eq 0 ]]; then
  trap cleanup EXIT ERR INT TERM
  setup
  build
  exit 0
fi

case $1 in
setup | cleanup | build)
  $1
  ;;
*)
  echo "Unsupported command: $1" >/dev/stderr
  exit 1
  ;;
esac
