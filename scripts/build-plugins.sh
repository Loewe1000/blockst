#!/usr/bin/env bash
# Build the three WASM plugins from source and install them under libs/.
#
# Usage:
#   ./scripts/build-plugins.sh            # build all three
#   ./scripts/build-plugins.sh scratch    # one of: scratch, sb3, nepo
#   ./scripts/build-plugins.sh --check    # build, then fail if a committed .wasm differs
#
# The builds are byte-reproducible, but only with the toolchain they were
# made with: rustup's stable, which has the wasm32-unknown-unknown target.
# A Homebrew rust earlier on PATH has no such target and fails with "can't
# find crate for core" — and cargo takes `rustc` from PATH, so rustup's cargo
# alone is not enough. This script puts rustup first.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export PATH="$HOME/.cargo/bin:$PATH"
TARGET=wasm32-unknown-unknown

if ! rustup target list --installed 2>/dev/null | grep -q "^$TARGET$"; then
  echo "rustup toolchain without the $TARGET target; run: rustup target add $TARGET" >&2
  exit 1
fi

CHECK=0
WANTED=()
for arg in "$@"; do
  case "$arg" in
    --check) CHECK=1 ;;
    scratch|sb3|nepo) WANTED+=("$arg") ;;
    *) echo "Usage: $0 [--check] [scratch|sb3|nepo]..." >&2; exit 2 ;;
  esac
done
[ ${#WANTED[@]} -eq 0 ] && WANTED=(scratch sb3 nepo)

# macOS ships bash 3.2, so no associative arrays: a function per lookup.
crate_dir() { case "$1" in scratch) echo scratchblocks-wasm ;; sb3) echo sb3-wasm ;; nepo) echo nepo-wasm ;; esac; }
built_file() { case "$1" in scratch) echo scratchblocks_wasm.wasm ;; sb3) echo sb3_wasm.wasm ;; nepo) echo nepo_wasm.wasm ;; esac; }
dest_file() { case "$1" in scratch) echo libs/scratch/plugins/scratchblocks_wasm.wasm ;; sb3) echo libs/scratch/plugins/sb3_wasm.wasm ;; nepo) echo libs/nepo/plugins/nepo_wasm.wasm ;; esac; }

status=0
for name in "${WANTED[@]}"; do
  dir="$ROOT/scripts/$(crate_dir "$name")"
  echo "== $name ($(rustc --version))"
  (cd "$dir" && cargo build --release --target "$TARGET" --quiet)
  built="$dir/target/$TARGET/release/$(built_file "$name")"
  dest="$ROOT/$(dest_file "$name")"
  if [ "$CHECK" -eq 1 ]; then
    if cmp -s "$built" "$dest"; then
      echo "   unchanged: $(dest_file "$name")"
    else
      echo "   DIFFERS:   $(dest_file "$name") ($(stat -f %z "$dest") bytes committed, $(stat -f %z "$built") built)"
      status=1
    fi
  else
    cp "$built" "$dest"
    echo "   installed: $(dest_file "$name") ($(stat -f %z "$dest") bytes)"
  fi
done
exit $status
