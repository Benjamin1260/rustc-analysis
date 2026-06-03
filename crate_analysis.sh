#!/usr/bin/bash

PAGE_COUNT="${1:-10}"
SLEEP_TIME="${2:-0}"
let CRATE_COUNT=$PAGE_COUNT*100
FILE_NAME="top_${CRATE_COUNT}_crates.json"
STAT_FILE_NAME="top_${CRATE_COUNT}_crates_stats.json"

rm "$FILE_NAME" || true

# 1) fetch data about crates from crate.io
for (( i=1; i<=PAGE_COUNT; ++i))
do
  curl "https://crates.io/api/v1/crates?page=${i}&per_page=100&sort=downloads" \
  | jq '.[].[] | {name: .name, repository: .repository}' >> "$FILE_NAME"
  sleep "$SLEEP_TIME"
done

# 2) analyze crates and group them by repository
cat "$FILE_NAME" \
  | sed -E 's/https?:\/\/([^.]*)\.[^\"]*/\1/' \
  | jq --slurp '.' \
  | jq 'group_by(.repository)
    | map({Repository: .[0].repository, Count: length, Names: [.[].name]})' \
  > "$STAT_FILE_NAME"

# 3) print brief overview of found results
cat "$STAT_FILE_NAME" | jq '.[] | {Repository: .Repository, Count: .Count}'
