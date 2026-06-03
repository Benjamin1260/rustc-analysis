#!/usr/bin/bash

PAGE_COUNT="${1:-100}"
SLEEP_TIME="${2:-2}"
let CRATE_COUNT=$PAGE_COUNT*100
FILE_NAME="top_${CRATE_COUNT}_crates.json"

LIMIT="${3:-100}"
TMP=$(mktemp)

rm "$FILE_NAME" || true

# 1) fetch crate data from crate.io
for (( i=1; i<=PAGE_COUNT; ++i))
do
  curl "https://crates.io/api/v1/crates?page=${i}&per_page=100&sort=downloads" \
  | jq '.[].[] | {name: .name, repository: .repository}' >> "$FILE_NAME"
  sleep "$SLEEP_TIME"
done

# 2) get top x github repositories
gh search repos \
  --language Rust \
  --sort stars \
  --order desc \
  --limit "$LIMIT" \
  --json name,url,stargazersCount \
| jq '.[].url' \
> "$TMP"

# 3) check if github repository URLS can be found in top-y crates from crates.io
grep -f "$TMP" "$FILE_NAME" \
| sort --unique \
| grep '.*' -c

