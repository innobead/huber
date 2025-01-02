#!/usr/bin/env bash

os=$(uname)
arch=$(uname -m)
filename="huber-linux-amd64"

case $os in
"Linux")
  case $arch in
  "aarch64")
    filename="huber-linux-arm64"
    ;;
  "armv7l")
    filename="huber-linux-armv7"
    ;;
  "x86_64")
    filename="huber-linux-amd64"
    ;;
  *)
    echo "The architecture ($arch) is not supported" >/dev/stderr
    exit 1
    ;;
  esac
  ;;
"Darwin")
  filename="huber-darwin-amd64"
  ;;
*)
  echo "The platform ($os) is not supported" >/dev/stderr
  exit 1
  ;;
esac

echo $filename
