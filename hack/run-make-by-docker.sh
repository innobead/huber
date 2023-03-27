#!/usr/bin/env bash

if [ $# -le 0 ]; then
  echo "Please run this script with make target like \"${BASH_SOURCE[0]} make generate\"" >/dev/stderr
  exit 1
fi

PROJECT_DIR=$(readlink -f "$(dirname "${BASH_SOURCE[0]}")/..")
CARGO_OPTS=${CARGO_OPTS:-}
GITHUB_KEY=${GITHUB_KEY:-}
GITHUB_TOKEN=${GITHUB_TOKEN:-}

docker build -t huber-dev -f "$PROJECT_DIR"/Dockerfile.dev "$PROJECT_DIR"

# shellcheck disable=SC2068
docker run --name huber-dev --rm \
  -e GITHUB_KEY="${GITHUB_KEY}" \
  -e GITHUB_TOKEN="${GITHUB_TOKEN}" \
  -e CARGO_OPTS="${CARGO_OPTS}" \
  -v "${PROJECT_DIR}":/workspace \
  -u "$(id -u)" \
  huber-dev \
  $@
