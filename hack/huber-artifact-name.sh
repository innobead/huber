#!/usr/bin/env bash

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