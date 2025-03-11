#!/usr/bin/env bash

IN_DIR="tests/input/cat"
OUT_DIR="target/tests/expected/cat"
[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

EMPTY_FILE="$IN_DIR/empty.txt"
FOX_FILE="$IN_DIR/fox.txt"
SPIDERS_FILE="$IN_DIR/spiders.txt"
BUSTLE_FILE="$IN_DIR/the_bustle.txt"
ALL_FILES=("$EMPTY_FILE" "$FOX_FILE" "$SPIDERS_FILE" "$BUSTLE_FILE")

for FILE in "${ALL_FILES[@]}"; do
  BASENAME="$(basename "$FILE")"
  BASENAME="${BASENAME%.*}"
  cat "$FILE" > "$OUT_DIR/$BASENAME.out"
  cat -n "$FILE" > "$OUT_DIR/$BASENAME.n.out"
  cat -b "$FILE" > "$OUT_DIR/$BASENAME.b.out"
done

cat "${ALL_FILES[@]}" > "$OUT_DIR/all.out"
cat -n "${ALL_FILES[@]}" > "$OUT_DIR/all.n.out"
cat -b "${ALL_FILES[@]}" > "$OUT_DIR/all.b.out"

BASENAME="$(basename "$BUSTLE_FILE")"
BASENAME="${BASENAME%.*}"
cat < "$BUSTLE_FILE" > "$OUT_DIR/$BASENAME.stdin.out"
cat -n < "$BUSTLE_FILE" > "$OUT_DIR/$BASENAME.stdin.n.out"
cat -b < "$BUSTLE_FILE" > "$OUT_DIR/$BASENAME.stdin.b.out"