#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

export_statement="export PATH=\$HOME/.huber/bin:\$PATH"
if ! grep -Fxq "$export_statement"  ~/.bashrc; then
  echo "$export_statement" >> ~/.bashrc
fi
