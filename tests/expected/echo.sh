#!/usr/bin/env bash

#set -u
#PROJECT_HOME="$(dirname "$(readlink -f "$0")")/../.."
OUT_DIR="target/tests/expected/echo"
[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

echo 'apple    banana' > "$OUT_DIR/fruit_1.out"
echo apple    banana > "$OUT_DIR/fruit_2.out"
echo -n 'apple    banana' > "$OUT_DIR/fruit_1.n.out"
echo -n apple    banana > "$OUT_DIR/fruit_2.n.out"