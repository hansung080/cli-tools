#!/usr/bin/env bash

IN_DIR="tests/input/head"
OUT_DIR="target/tests/expected/head"
[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

EMPTY_FILE="$IN_DIR/empty.txt"
ONE_FILE="$IN_DIR/one.txt"
TWO_FILE="$IN_DIR/two.txt"
THREE_FILE="$IN_DIR/three.txt"
TWELVE_FILE="$IN_DIR/twelve.txt"
ALL_FILES=("$EMPTY_FILE" "$ONE_FILE" "$TWO_FILE" "$THREE_FILE" "$TWELVE_FILE")

for FILE in "${ALL_FILES[@]}"; do
  BASENAME="$(basename "$FILE")"
  BASENAME="${BASENAME%.*}"
  head "$FILE" > "$OUT_DIR/$BASENAME.out"
  head -n 2 "$FILE" > "$OUT_DIR/$BASENAME.n2.out"
  head -n 4 "$FILE" > "$OUT_DIR/$BASENAME.n4.out"
  head -c 1 "$FILE" > "$OUT_DIR/$BASENAME.c1.out"
  head -c 2 "$FILE" > "$OUT_DIR/$BASENAME.c2.out"
  head -c 4 "$FILE" > "$OUT_DIR/$BASENAME.c4.out"
done

head "${ALL_FILES[@]}" > "$OUT_DIR/all.out"
head -n 2 "${ALL_FILES[@]}" > "$OUT_DIR/all.n2.out"
head -n 4 "${ALL_FILES[@]}" > "$OUT_DIR/all.n4.out"
head -c 1 "${ALL_FILES[@]}" > "$OUT_DIR/all.c1.out"
head -c 2 "${ALL_FILES[@]}" > "$OUT_DIR/all.c2.out"
head -c 4 "${ALL_FILES[@]}" > "$OUT_DIR/all.c4.out"

BASENAME="$(basename "$ONE_FILE")"
BASENAME="${BASENAME%.*}"
head < "$ONE_FILE" > "$OUT_DIR/$BASENAME.stdin.out"
head -n 2 < "$ONE_FILE" > "$OUT_DIR/$BASENAME.stdin.n2.out"
head -n 4 < "$ONE_FILE" > "$OUT_DIR/$BASENAME.stdin.n4.out"
head -c 1 < "$ONE_FILE" > "$OUT_DIR/$BASENAME.stdin.c1.out"
head -c 2 < "$ONE_FILE" > "$OUT_DIR/$BASENAME.stdin.c2.out"
head -c 4 < "$ONE_FILE" > "$OUT_DIR/$BASENAME.stdin.c4.out"