#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SCRATCHBLOCKS_DIR="${SCRATCHBLOCKS_DIR:-/tmp/scratchblocks}"
AUDIT_FILE="$(mktemp /tmp/blockst-sb-audit-XXXXXX.typ)"
I18N_AUDIT_FILE="$(mktemp /tmp/blockst-sb-i18n-audit-XXXXXX.typ)"
OUT_FILE="/tmp/blockst-sb-audit.svg"
I18N_OUT_FILE="/tmp/blockst-sb-i18n-audit.svg"

cleanup() {
  rm -f "$AUDIT_FILE" "$I18N_AUDIT_FILE"
}
trap cleanup EXIT

if ! command -v typst >/dev/null 2>&1; then
  echo "Fehler: typst ist nicht installiert oder nicht im PATH." >&2
  exit 1
fi

if ! command -v git >/dev/null 2>&1; then
  echo "Fehler: git ist nicht installiert oder nicht im PATH." >&2
  exit 1
fi

if [ ! -d "$SCRATCHBLOCKS_DIR/.git" ]; then
  echo "Clone scratchblocks nach $SCRATCHBLOCKS_DIR ..."
  rm -rf "$SCRATCHBLOCKS_DIR"
  git clone --depth 1 https://github.com/scratchblocks/scratchblocks "$SCRATCHBLOCKS_DIR"
fi

if [ ! -f "$SCRATCHBLOCKS_DIR/tests/all-blocks.txt" ]; then
  echo "Fehler: scratchblocks Testdatei nicht gefunden: $SCRATCHBLOCKS_DIR/tests/all-blocks.txt" >&2
  exit 1
fi

cat > "$AUDIT_FILE" <<TYP
#import "$REPO_ROOT/libs/scratch/text/parser.typ": _order-defs, _parse-block-line, _normalise-line, default-statement-defs-raw, default-expression-defs-raw

#let lines = read("$SCRATCHBLOCKS_DIR/tests/all-blocks.txt").replace("\\r", "").split("\\n")
#let statement-defs = _order-defs(default-statement-defs-raw, "en")
#let expression-defs = _order-defs(default-expression-defs-raw, "en")

#let misses = ()

#for raw in lines {
  let line = _normalise-line(raw)
  if line == "" or line.starts-with("//") or line == "end" or line == "else" {
    continue
  }

  let parsed-statement = _parse-block-line(line, "en", statement-defs, expression-defs)
  if parsed-statement == none {
    let parsed-expression = _parse-block-line(line, "en", statement-defs, expression-defs, allow-body: false, expression-only: true)
    if parsed-expression == none {
      misses.push(line)
    }
  }
}

#if misses.len() > 0 {
  panic("UNSUPPORTED (" + str(misses.len()) + "):\\n" + misses.join("\\n"))
} else {
  [ALL SUPPORTED]
}
TYP

echo "Pruefe Scratchblocks-Syntax-Coverage ..."
typst compile --root / "$AUDIT_FILE" "$OUT_FILE"
echo "OK: Alle Zeilen aus scratchblocks/tests/all-blocks.txt werden erkannt."

cat > "$I18N_AUDIT_FILE" <<TYP
#import "$REPO_ROOT/libs/scratch/text/parser.typ": default-statement-defs-raw, default-expression-defs-raw
#import "$REPO_ROOT/libs/scratch/core.typ": get-template

#let all-defs = (..default-statement-defs-raw, ..default-expression-defs-raw)
#let langs = ("de", "fr")
#let misses = ()

#for lang in langs {
  for entry in all-defs {
    let id = entry.id
    let template = get-template(id, lang)
    if template == id {
      misses.push(lang + ":" + id)
    }
  }
}

#if misses.len() > 0 {
  panic("MISSING TRANSLATIONS (" + str(misses.len()) + "):\\n" + misses.join("\\n"))
} else {
  [ALL TRANSLATIONS PRESENT]
}
TYP

echo "Pruefe DE/FR-Template-Abdeckung ..."
typst compile --root / "$I18N_AUDIT_FILE" "$I18N_OUT_FILE"
echo "OK: DE/FR enthalten Templates fuer alle Parser-Block-IDs."
