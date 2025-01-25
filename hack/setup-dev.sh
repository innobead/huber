#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

install_linux_deps() {
  if [[ $(command -v zypper) ]]; then
    sudo zypper install -y make libopenssl-devel git curl mingw64-cross-gcc-c++ cross-aarch64-gcc14 cross-arm-linux-gnueabi-gcc
  elif [[ $(command -v apt) ]]; then
    sudo apt update
    sudo DEBIAN_FRONTEND=noninteractive apt install -y make libssl-dev git curl mingw-w64 gcc-aarch64-linux-gnu gcc-arm-linux-gnueabihf
  else
    echo "Only openSUSE, Ubuntu supported" >/dev/stderr
    exit 1
  fi

  if [[ -z $(command -v just 2>/dev/null) ]]; then
    curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin
  fi
}

install_rust_deps() {
  if [[ -z $(command -v cargo 2>/dev/null) ]]; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustup toolchain install nightly
    source "$HOME"/.cargo/env
    cargo version
  fi

  export_statement="export PATH=\$HOME/.cargo/bin:\$PATH"
  if ! grep -Fxq "$export_statement" ~/.bashrc; then
    echo "$export_statement" >>"$HOME"/.bashrc
  fi

  if [[ -f "$HOME"/.cargo/env ]]; then
    source "$HOME"/.cargo/env
  fi

  rustup component add rustfmt clippy
  cargo install default-target
  cargo install --git https://github.com/DevinR528/cargo-sort.git --tag v1.1.0 cargo-sort
  cargo install cargo-udeps
}

if [ "$(uname)" == "Linux" ]; then
  install_linux_deps
fi
install_rust_deps
