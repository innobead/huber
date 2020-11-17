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
    filename="huber-darwin-amd64"
  ;;
*)
  echo "The platform is not supported" > /dev/stderr
  exit 1
  ;;
esac

echo $filename
# shellcheck disable=SC2046
curl -sfSLO "https://github.com/innobead/huber/releases/download/$(get_latest_release)/$filename" && \
 chmod +x $filename && \
 make -p ~/.huber/bin && \
 mv $filename ~/.huber/bin

export_statement="export PATH=\$HOME/.huber/bin:\$PATH"
if ! grep -Fxq "$export_statement"  ~/.bashrc; then
  echo "$export_statement" >> ~/.bashrc
fi
