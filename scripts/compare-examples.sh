#!/usr/bin/env bash
# compare-examples.sh
#
# Renders every example against the working tree and compares the result to a
# stored baseline. This is the acceptance check for changes that are supposed
# to leave the output alone — a refactor of the renderer, a rebuilt plugin, a
# new option that defaults to the old behaviour.
#
# Usage:
#   ./scripts/compare-examples.sh baseline    # record the current output
#   ./scripts/compare-examples.sh check       # compare against the recording
#
# The examples import @preview/blockst:<version>, so they are compiled against
# a package assembled from the working tree rather than against whatever is in
# the Typst cache. That is the point: it tests the code in front of you.
#
# Requires: typst, python3 with Pillow.

set -euo pipefail

MODE="${1:-check}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VERSION="$(grep -m1 '^version' "$ROOT/typst.toml" | cut -d'"' -f2)"
WORK="${BLOCKST_COMPARE_DIR:-${TMPDIR:-/tmp}/blockst-compare}"
PKG="$WORK/packages/preview/blockst/$VERSION"
BASELINE="$WORK/baseline"
CURRENT="$WORK/current"

case "$MODE" in
  baseline) TARGET="$BASELINE" ;;
  check)    TARGET="$CURRENT" ;;
  *) echo "Usage: $0 [baseline|check]" >&2; exit 2 ;;
esac

command -v typst >/dev/null || { echo "typst not found" >&2; exit 1; }

echo "Assembling blockst $VERSION from the working tree"
rm -rf "$PKG"
mkdir -p "$PKG"
rsync -a \
  --exclude '.git' --exclude 'examples' --exclude 'docs' \
  --exclude 'scripts/*/target' --exclude 'scripts/*/sources' \
  --exclude '.DS_Store' --exclude '*.pdf' \
  "$ROOT/" "$PKG/"

echo "Rendering examples"
rm -rf "$TARGET"
mkdir -p "$TARGET"
shopt -s nullglob
for file in "$ROOT"/examples/*.typ "$ROOT"/examples/catalog/*.typ; do
  name="$(basename "$file" .typ)"
  case "$name" in
    showcase-data) continue ;;                 # imported, not a document
    showcase-rtl) out="$TARGET/$name-{p}.png" ;;  # multi-page
    *) out="$TARGET/$name.png" ;;
  esac
  if ! typst compile --root "$ROOT" --package-path "$WORK/packages" \
        "$file" "$out" --ppi 96 2>"$TARGET/$name.log"; then
    echo "  FAILED $name"
    sed -n '1,5p' "$TARGET/$name.log"
    exit 1
  fi
done
rm -f "$TARGET"/*.log
echo "  $(ls "$TARGET" | wc -l | tr -d ' ') images"

if [ "$MODE" = "baseline" ]; then
  echo "Baseline stored in $BASELINE"
  exit 0
fi

python3 - "$BASELINE" "$CURRENT" <<'PY'
import sys, pathlib
from PIL import Image, ImageChops

baseline, current = (pathlib.Path(p) for p in sys.argv[1:3])
if not baseline.exists():
    sys.exit("no baseline recorded — run: ./scripts/compare-examples.sh baseline")

names = sorted({p.name for p in baseline.glob('*.png')} | {p.name for p in current.glob('*.png')})
differences = []
for name in names:
    a, b = baseline / name, current / name
    if not a.exists():
        differences.append(f"{name}: new, not in the baseline")
        continue
    if not b.exists():
        differences.append(f"{name}: missing from this run")
        continue
    first, second = Image.open(a).convert('RGB'), Image.open(b).convert('RGB')
    if first.size != second.size:
        differences.append(f"{name}: size {first.size} -> {second.size}")
        continue
    box = ImageChops.difference(first, second).getbbox()
    if box:
        pixels = sum(1 for p in ImageChops.difference(first, second).getdata() if max(p))
        differences.append(f"{name}: {pixels} pixels differ, bounding box {box}")

if differences:
    print(f"\n{len(differences)} of {len(names)} examples changed:\n")
    for line in differences:
        print(f"  {line}")
    sys.exit(1)

print(f"\nAll {len(names)} examples render identically.")
PY
