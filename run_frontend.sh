#!/usr/bin/env bash
set -e
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if command -v trunk >/dev/null 2>&1; then
    cd "$DIR/crates/frontend" && trunk serve --port 3000 "$@"
else
    nix develop "$DIR" --command bash -c "cd '$DIR/crates/frontend' && trunk serve --port 3000 \"\$@\"" -- "$@"
fi
