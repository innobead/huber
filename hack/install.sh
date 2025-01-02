#!/usr/bin/env sh

set -o errexit
set -o nounset
set -o xtrace

get_latest_release() {
  curl -sfSL "https://api.github.com/repos/innobead/huber/releases/latest" |
    grep '"tag_name":' |
    sed -E 's/.*"([^"]+)".*/\1/'
}

os=$(uname)
arch=$(uname -m)
filename="huber-linux-amd64"

case $os in
"Linux")
  case $arch in
  "aarch64")
    filename="huber-aarch64-unknown-linux-gnu" # musl
    ;;
  "armv7l")
    filename="huber-arm-unknown-linux-gnueabihf"
    ;;
  "x86_64")
    filename="huber-x86_64-unknown-linux-gnu"
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
    filename="huber-aarch64-apple-darwin"
    ;;
  "x86_64")
    filename="huber-x86_64-apple-darwin"
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

# shellcheck disable=SC2046
curl -sfSLO "https://github.com/innobead/huber/releases/download/$(get_latest_release)/$filename" &&
  chmod +x $filename &&
  mkdir -p ~/.huber/bin &&
  mv $filename ~/.huber/bin/huber

export_statement="export PATH=\$HOME/.huber/bin:\$PATH"
if ! grep -Fxq "$export_statement" ~/.bashrc; then
  echo "$export_statement" >>~/.bashrc
fi

cat <<EOF
The installation script has updated the \$PATH environment variable in $HOME/.bashrc.
Please restart the shell or source again to make it take effect.

If you use other shell, please update the \$PATH environment variable accordingly.
EOF
