#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

install_linux_deps() {
  if [[ $(command -v zypper) ]]; then
    # sudo zypper install -y -t pattern devel_basis
    sudo zypper install -y make libopenssl-devel libarchive-devel git pkg-config curl sudo
  elif [[ $(command -v apt) ]]; then
    sudo apt update
    # sudo apt install -y build-essential
    sudo DEBIAN_FRONTEND=noninteractive apt install -y make libssl-dev libarchive-dev git pkg-config curl sudo
  else
    echo "Only openSUSE, Ubuntu supported" >/dev/stderr
    exit 1
  fi

  if [[ -z $(command -v just 2>/dev/null) ]]; then
    curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin
  fi
}

install_macos_deps() {
  if [[ ! $(command -v brew) ]]; then
    curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh | sh -
  fi

  # https://github.com/libarchive/libarchive/blob/master/.github/workflows/ci.yml
  brew update
  # brew upgrade

  for pkg in \
    autoconf \
    automake \
    libtool \
    pkg-config \
    cmake \
    libarchive \
    openssl \
    just; do
    if ! (brew list $pkg && brew upgrade $pkg); then
      if [[ $pkg == "libarchive" ]]; then
        # fix https://github.com/libarchive/libarchive/pull/1813, use 3.6.1 instead
        export
        curl -L "https://raw.githubusercontent.com/Homebrew/homebrew-core/8a1f0e9b4df/Formula/libarchive.rb" >libarchive.rb &&
          HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK=1 brew install libarchive.rb
        continue
      fi

      brew install $pkg
    fi
  done

  {
    echo "export PATH=/usr/local/opt/libarchive/bin:\$PATH"
    echo "export LDFLAGS=-L/usr/local/opt/libarchive/lib"
    echo "export CPPFLAGS=-I/usr/local/opt/libarchive/include"
    echo "export PKG_CONFIG_PATH=/usr/local/opt/libarchive/lib/pkgconfig"
  } >>"$HOME"/.bashrc

  source "$HOME"/.bashrc
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
}

os=$(uname)
case $os in
"Linux")
  install_linux_deps
  ;;
"Darwin")
  install_macos_deps
  ;;
esac

install_rust_deps
