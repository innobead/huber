#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

function get_latest_release() {
  curl -sfSL "https://api.github.com/repos/innobead/huber/releases/latest" |
    grep '"tag_name":' |
    sed -E 's/.*"([^"]+)".*/\1/'
}

os=$(uname)
arch=$(uname -m)
filename="huber-linux-amd64"

case $os in
"Linux")
  if [[ "$arch" == "aarch64" ]]; then
    filename="huber-linux-arm64"
  fi
  ;;
"Darwin")
    filename="huber-darwin-adm64"
  ;;
esac

echo $filename
# shellcheck disable=SC2046
curl -sfSLO "https://github.com/innobead/kubefire/releases/download/$(get_latest_release)/$filename" && \
 chmod +x $filename && \
 make -p ~/.huber/sbin && \
 mv $filename /usr/local/bin/kubefire
