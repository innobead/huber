#!/usr/bin/env bash

set -o errexit
set -o pipefail
#set -o nounset
#set -o xtrace

readlink=readlink
if command -v greadlink &> /dev/null; then
  readlink=greadlink
fi

PROJECT_DIR=$($readlink -f "$(dirname "${BASH_SOURCE[0]}")/..")
PKG_INDEXES_MD="${PROJECT_DIR}"/docs/packages.md
PKG_INDEXES_CONTENT=${1:-}

if [ -z "$PKG_INDEXES_CONTENT" ]; then
  echo "No generated package lists from \`huber search\`, so it was unable to generate ${PKG_INDEXES_MD}" >> /dev/stderr
  exit 1
fi

content=$(cat <<'EOF'
## Huber Managed Packages

```console
{value}
```
EOF
)

content=${content/\{value\}/$PKG_INDEXES_CONTENT}
echo "$content" > "$PKG_INDEXES_MD"