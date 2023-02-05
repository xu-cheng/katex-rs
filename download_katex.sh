#!/usr/bin/env bash

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
VENDOR_DIR="$ROOT_DIR/vendor"

VERSION="$(cat ./KATEX-VERSION)"
URL="https://github.com/KaTeX/KaTeX/releases/download/v${VERSION}/katex.tar.gz"

rm -rf "$VENDOR_DIR"
mkdir -p "$VENDOR_DIR"
echo "download ${URL}..."
curl -L https://raw.githubusercontent.com/KaTeX/KaTeX/master/LICENSE -o "$VENDOR_DIR/KATEX-LICENSE"
curl -L "$URL" | tar -x -z -C "$VENDOR_DIR" --strip-components 1 -f - katex/katex.min.js katex/contrib/mhchem.min.js

