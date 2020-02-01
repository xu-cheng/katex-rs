#!/usr/bin/env bash

VERSION="v0.11.1"
URL="https://github.com/KaTeX/KaTeX/releases/download/${VERSION}/katex.tar.gz"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
VENDOR_DIR="$ROOT_DIR/vendor"

mkdir -p "$VENDOR_DIR"
rm -f "$VENDOR_DIR/katex.min.js"
curl -L "$URL" | tar -x -z -C "$VENDOR_DIR" --strip-components 1 -f - katex/katex.min.js
