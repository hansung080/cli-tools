#!/usr/bin/env bash

PROJECT_HOME="$(dirname "$(readlink -f "$0")")/../.."
OUT_DIR="$PROJECT_HOME/target/tests/expected"
[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

echo 'apple    banana' > "$OUT_DIR/echo_1.txt"
echo apple    banana > "$OUT_DIR/echo_2.txt"
echo -n 'apple    banana' > "$OUT_DIR/echo_n_1.txt"
echo -n apple    banana > "$OUT_DIR/echo_n_2.txt"