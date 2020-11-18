#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail
#set -o xtrace

PROJECT_DIR=$(readlink -f "$(dirname "${BASH_SOURCE[0]}")/..")

content=$(cat <<'EOF'
## Huber Managed Packages

```console
{value}
```
EOF
)

content=${content/\{value\}/$1}
echo "$content" > "${PROJECT_DIR}"/doc/packages.md