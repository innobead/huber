#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

export_statement="export PATH=\$HOME/.huber/bin:\$PATH"

shells=(bashrc zshrc)
for s in "${shells[@]}"; do
  if [ -f "$HOME"/."$s" ] && ! grep -Fxq "$export_statement" "$HOME"/."$s"; then
    echo "$export_statement" >>"$HOME"/."$s"
  fi
done
