#!/usr/bin/env bash
set -euo pipefail # stop if errors occur

LIMIT=${1:-10} # first arg passed or default to 10
DEST_DIR="gh-rust-t$LIMIT"

mkdir -p "$DEST_DIR" # create dir if doesnt exist yet
cd "$DEST_DIR"

# CLI for github to fetch json
# jq iterates over json array, takes url pipes it
# while read goes line-by-line (url-by-url) and clones the source code
gh search repos \
  --language Rust \
  --sort stars \
  --order desc \
  --limit "$LIMIT" \
  --json name,url,stargazersCount | \
jq -r '.[] | .url' | \
while read -r url; do
  echo "Cloning $url"
  git clone --depth 1 "$url"
done
