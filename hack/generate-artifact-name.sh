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
  "x86_64")
    filename="huber-linux-amd64"
    ;;
  *)
    echo "$os:$arch is not supported" >/dev/stderr
    exit 1
    ;;
  esac
  ;;
"Darwin")
  case $arch in
  "arm64")
    filename="huber-darwin-arm64"
    ;;
  "x86_64")
    filename="huber-darwin-amd64"
    ;;
  *)
    echo "$os:$arch is not supported" >/dev/stderr
    exit 1
    ;;
  esac
  ;;
*)
  echo "$os:$arch is not supported" >/dev/stderr
  exit 1
  ;;
esac

echo $filename
