#!/usr/bin/env bash
# check-sb3-import-coverage.sh
#
# Checks how well the current SB3 importer covers opcodes found in a .sb3 file.
#
# Usage:
#   ./scripts/sb3-wasm/check-sb3-import-coverage.sh <path/to/project.sb3>
#   ./scripts/sb3-wasm/check-sb3-import-coverage.sh examples/Listen.sb3 --strict
#
# Exit codes:
#   0 = analysis completed (and no unsupported opcodes in --strict mode)
#   2 = unsupported opcodes found in --strict mode
#   1 = usage/prerequisite/runtime error

set -euo pipefail

if [[ $# -lt 1 || $# -gt 2 ]]; then
  echo "Usage: $0 <path/to/project.sb3> [--strict]"
  exit 1
fi

SB3_PATH="$1"
STRICT_MODE="false"
if [[ ${2:-""} == "--strict" ]]; then
  STRICT_MODE="true"
elif [[ $# -eq 2 ]]; then
  echo "Unknown option: $2"
  echo "Usage: $0 <path/to/project.sb3> [--strict]"
  exit 1
fi

if [[ ! -f "$SB3_PATH" ]]; then
  echo "Error: SB3 file not found: $SB3_PATH"
  exit 1
fi

for cmd in unzip jq sort uniq comm sed mktemp grep; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Error: required command not found: $cmd"
    exit 1
  fi
done

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
PLUGIN_SOURCE="$ROOT_DIR/scripts/sb3-wasm/src/lib.rs"

if [[ ! -f "$PLUGIN_SOURCE" ]]; then
  echo "Error: plugin source not found: $PLUGIN_SOURCE"
  exit 1
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

ALL_OPCODES_FILE="$TMP_DIR/all_opcodes.txt"
UNIQUE_OPCODES_FILE="$TMP_DIR/unique_opcodes.txt"
SUPPORTED_OPCODES_FILE="$TMP_DIR/supported_opcodes.txt"
UNSUPPORTED_OPCODES_FILE="$TMP_DIR/unsupported_opcodes.txt"

unzip -p "$SB3_PATH" project.json | jq -r '
  .targets[]
  | .blocks
  | to_entries[]
  | select((.value | type) == "object")
  | .value.opcode
  | select(. != null)
' > "$ALL_OPCODES_FILE"

sort -u "$ALL_OPCODES_FILE" > "$UNIQUE_OPCODES_FILE"

# Extract opcodes from Rust match arms like: "opcode_name" => ...
if command -v rg >/dev/null 2>&1; then
  rg --no-filename --only-matching '"[A-Za-z0-9_]+"\s*=>' "$PLUGIN_SOURCE" \
    | sed -E 's/"([A-Za-z0-9_]+)"\s*=>/\1/' \
    | sort -u > "$SUPPORTED_OPCODES_FILE"
else
  grep -Eo '"[A-Za-z0-9_]+"[[:space:]]*=>' "$PLUGIN_SOURCE" \
    | sed -E 's/"([A-Za-z0-9_]+)"[[:space:]]*=>/\1/' \
    | sort -u > "$SUPPORTED_OPCODES_FILE"
fi

comm -23 "$UNIQUE_OPCODES_FILE" "$SUPPORTED_OPCODES_FILE" > "$UNSUPPORTED_OPCODES_FILE"

TOTAL_BLOCKS="$(wc -l < "$ALL_OPCODES_FILE" | tr -d ' ')"
TOTAL_UNIQUE="$(wc -l < "$UNIQUE_OPCODES_FILE" | tr -d ' ')"
TOTAL_UNSUPPORTED="$(wc -l < "$UNSUPPORTED_OPCODES_FILE" | tr -d ' ')"
TOTAL_SUPPORTED="$(comm -12 "$UNIQUE_OPCODES_FILE" "$SUPPORTED_OPCODES_FILE" | wc -l | tr -d ' ')"

echo "SB3 Import Coverage Report"
echo "=========================="
echo "File: $SB3_PATH"
echo "Total blocks in file: $TOTAL_BLOCKS"
echo "Unique opcodes in file: $TOTAL_UNIQUE"
echo "Supported opcodes in file: $TOTAL_SUPPORTED"
echo "Unsupported opcodes in file: $TOTAL_UNSUPPORTED"
echo

if [[ "$TOTAL_UNSUPPORTED" -gt 0 ]]; then
  echo "Unsupported opcodes (unique):"
  cat "$UNSUPPORTED_OPCODES_FILE"
  echo

  echo "Unsupported opcode counts in this file:"
  while IFS= read -r opcode; do
    count="$(grep -c -x "$opcode" "$ALL_OPCODES_FILE" || true)"
    printf "  %5d  %s\n" "$count" "$opcode"
  done < "$UNSUPPORTED_OPCODES_FILE"
  echo
else
  echo "All opcodes in this SB3 are currently mapped by the importer."
  echo
fi

if [[ "$STRICT_MODE" == "true" && "$TOTAL_UNSUPPORTED" -gt 0 ]]; then
  echo "Strict mode enabled: failing because unsupported opcodes were found."
  exit 2
fi

echo "Done."
