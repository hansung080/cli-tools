#!/usr/bin/env bash

PROJECT_HOME="$(dirname "$(readlink -f "$0")")/../.."
OUT_DIR="$PROJECT_HOME/target/tests/expected/echo"
[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

echo 'apple    banana' > "$OUT_DIR/fruit_1.out"
echo apple    banana > "$OUT_DIR/fruit_2.out"
echo -n 'apple    banana' > "$OUT_DIR/fruit_1.n.out"
echo -n apple    banana > "$OUT_DIR/fruit_2.n.out"