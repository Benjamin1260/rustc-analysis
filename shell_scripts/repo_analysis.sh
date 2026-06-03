#!/usr/bin/env bash
set -euo pipefail
trap 'echo "Error on line $LINENO" >&2' ERR # print the line at which we failed

OUT="counts.csv"
TMP=$(mktemp)
TABLE_HEADER="project,loc,async_count,await_count,ratio"

# clear file:
printf "" > "$OUT"

# scan source files
for dir in */; do
  dir_name=${dir%/}

  loc=$(find "$dir" -name '*.rs' -print0 | xargs -0 cat | wc -l)
  async_count=$(find "$dir" -name "*.rs" -print0 | xargs -0 cat | (grep -c "\basync\b" || [[ $? -eq 1 ]]))
  await_count=$(find "$dir" -name "*.rs" -print0 | xargs -0 cat | (grep -c "\bawait\b" || [[ $? -eq 1 ]]))

  printf "%s,%s,%s,%s\n" "$dir_name" "$loc" "$async_count" "$await_count" >> "$OUT"
done

# calculate ratio
awk --csv ' \
  BEGIN {OFS=","} \
  {print $0, ($3 + $4)/$2} \
' "$OUT" | \
sort -t, -k5 -gr > "$TMP" # sort using separator ',' on column 5 (1-indexed) using reversed floats

# insert table header and cleanup
printf '%s\n' "$TABLE_HEADER" > "$OUT"
cat "$TMP" >> "$OUT"
rm "$TMP"
