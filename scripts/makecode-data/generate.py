#!/usr/bin/env python3
"""Generate blockst block data for the Microsoft MakeCode dialect.

MakeCode (https://www.microsoft.com/makecode) is the block/JavaScript editor
family behind makecode.microbit.org and makecode.calliope.cc. Its blocks come
from two places:

  * blocks declared in TypeScript with `//%` annotations, one per exported
    function or ambient shim declaration (`//% blockId=... block="..."`) —
    this is how micro:bit's own namespaces (basic, input, led, control,
    pins, serial, loops, game, radio) and Calliope mini's extra namespaces
    (motors, the RGB LED) are defined;
  * a handful of blocks the *editor* (pxt, the engine both targets run on)
    builds itself: the loop/logic/math/variables/array/text/function blocks
    that every MakeCode target shares. Those are declared imperatively in
    pxt's pxtlib/blocks.ts rather than as `//%`-annotated TypeScript, so they
    are transcribed here (see BUILTIN_BLOCKS below) the same way
    scripts/blockly-data/generate.py transcribes Blockly's five procedure
    blocks — they are not JSON, so scraping would mean interpreting code.

Both are turned into the TOML shape blockst already uses for Scratch and
Blockly: a `[specs]` table mapping a block id to its localized text with
%1-style placeholders, plus [shapes]/[categories]/[slots] siblings. This
script writes locales for micro:bit (English and German) and for Calliope
mini (which inherits the micro:bit ones and adds its motor/RGB-LED blocks),
plus a profile per target carrying the toolbox category colours.

Sources are downloaded once into sources/ and reused; pass --offline to
work from the cache alone.

    python3 scripts/makecode-data/generate.py

Licenses of the generated data: pxt, pxt-microbit and pxt-calliope are all
MIT-licensed. Noted again in the header of every generated file.

A note on German coverage
--------------------------
MakeCode's per-block texts (`basic.showNumber|block` and friends) are
translated on Crowdin and served to the running editor from a translation
table (`pxtc.apiLocalizationStrings`) that is populated only for blocks the
editor has actually rendered in that session, and is not exposed by the
public `/api/translations` endpoint the way the editor's general UI strings
are (that endpoint serves `strings.json`/`target-strings.json`/
`sim-strings.json` but returns an empty object for the per-block
`bundled-strings.json`, tested against several languages). There is no
committed file in pxt-microbit/pxt-calliope with this data either — only the
English source strings live in the repo, under `_locales/*-strings.json`.

So, unlike the Blockly generator (whose German comes wholesale from
Blockly's and bebras-modules' own bundled locale files), this generator's
German text for the custom namespace blocks is a hand-curated table
(GERMAN_TEXT below), built from: (a) strings read directly off the running
German editor during development of this script (recorded in comments where
that happened), and (b) otherwise-well-established MakeCode DE wording. Any
block without an entry there is left in English in the German locale and
listed in REPORT.md, exactly as an untranslated jwinf world block is.
The loop/logic/math/variables/array/text/function blocks are the exception:
their German *is* reproducibly sourced, because pxt translates them through
the same general-purpose string table the editor UI uses (Util.blf / lf()),
which the public endpoint does serve — the exact English/German pairs below
were read from that table.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
import urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCES = HERE / "sources"
OUT = HERE.parent / "scratchblocks-wasm" / "data" / "dialects"

# Pinned commits, so a re-run months from now still generates the same
# output. Bump these by hand (and re-run) to pick up upstream changes.
MICROBIT_SHA = "b4a2d7e070f62839fe58c94ce57e69d496443a24"
CALLIOPE_SHA = "b9d5a6e3db0258547a223cfa2d462e848bb29ee7"

MICROBIT_RAW = f"https://raw.githubusercontent.com/microsoft/pxt-microbit/{MICROBIT_SHA}"
CALLIOPE_RAW = f"https://raw.githubusercontent.com/microsoft/pxt-calliope/{CALLIOPE_SHA}"

DOWNLOADS = {
    "microbit_pxtarget.json": f"{MICROBIT_RAW}/pxtarget.json",
    "microbit_core_shims.d.ts": f"{MICROBIT_RAW}/libs/core/shims.d.ts",
    "microbit_basic.ts": f"{MICROBIT_RAW}/libs/core/basic.ts",
    "microbit_input.ts": f"{MICROBIT_RAW}/libs/core/input.ts",
    "microbit_led.ts": f"{MICROBIT_RAW}/libs/core/led.ts",
    "microbit_control.ts": f"{MICROBIT_RAW}/libs/core/control.ts",
    "microbit_pins.ts": f"{MICROBIT_RAW}/libs/core/pins.ts",
    "microbit_serial.ts": f"{MICROBIT_RAW}/libs/core/serial.ts",
    "microbit_loops.ts": f"{MICROBIT_RAW}/libs/core/loops.ts",
    "microbit_game.ts": f"{MICROBIT_RAW}/libs/core/game.ts",
    "microbit_icons.ts": f"{MICROBIT_RAW}/libs/core/icons.ts",
    "microbit_radio_shims.d.ts": f"{MICROBIT_RAW}/libs/radio/shims.d.ts",
    "microbit_radio_targetoverrides.ts": f"{MICROBIT_RAW}/libs/radio/targetoverrides.ts",
    "calliope_pxtarget.json": f"{CALLIOPE_RAW}/pxtarget.json",
    "calliope_motors.ts": f"{CALLIOPE_RAW}/libs/core-mini-codal/motors.ts",
    "calliope_rgbled.ts": f"{CALLIOPE_RAW}/libs/core-mini-codal/rgbled.ts",
    "calliope_shims.d.ts": f"{CALLIOPE_RAW}/libs/core-mini-codal/shims.d.ts",
}

# Which downloaded files feed the micro:bit vs. the Calliope-only catalog.
MICROBIT_FILES = [
    ("microbit_core_shims.d.ts", None),
    ("microbit_basic.ts", None),
    ("microbit_input.ts", None),
    ("microbit_led.ts", None),
    ("microbit_control.ts", None),
    ("microbit_pins.ts", None),
    ("microbit_serial.ts", None),
    ("microbit_loops.ts", None),
    ("microbit_game.ts", None),
    ("microbit_icons.ts", None),
    ("microbit_radio_shims.d.ts", None),
    ("microbit_radio_targetoverrides.ts", None),
]
CALLIOPE_FILES = [
    ("calliope_motors.ts", None),
    ("calliope_rgbled.ts", None),
    ("calliope_shims.d.ts", None),
]

# Namespaces we do not want blocks from even if a file defines some: too
# large (melody/sound-effect editors), or shadow-only picker blocks with no
# toolbox presence of their own. See "What was left out" in the module
# docstring's neighbourhood / REPORT.md.
SKIP_NAMESPACES = {"Math"}  # helpers.ts's convert()/randomBoolean() — not fetched, listed for clarity


def fetch(offline: bool) -> None:
    SOURCES.mkdir(parents=True, exist_ok=True)
    for name, url in DOWNLOADS.items():
        target = SOURCES / name
        if target.exists() and target.stat().st_size > 0:
            continue
        if offline:
            sys.exit(f"missing source {target} and --offline was given")
        print(f"  fetching {name}")
        with urllib.request.urlopen(url) as response:
            target.write_bytes(response.read())


# --------------------------------------------------------------------------
# `//%`-annotated TypeScript blocks (basic, input, led, control, pins,
# serial, loops, game, radio; motors and the RGB LED for Calliope)
# --------------------------------------------------------------------------

# A block's %name/$name token may carry a shadow block (`%value=math_number`)
# that fills the socket with something other than a bare value editor. What
# kind of slot each shadow draws as, for the [slots] table.
SHADOW_KIND = {
    "math_number": "value",
    "variables_get": "value",
    "toggleOnOff": "field",
    "colorNumberPicker": "field",
    "speedPicker": "value",
    "timePicker": "value",
    "longTimePicker": "value",
    "device_note": "dropdown",
    "device_beat": "dropdown",
    "device_arrow": "dropdown",
    "device_builtin_melody": "dropdown",
    "control_event_source_id": "dropdown",
    "control_event_value_id": "dropdown",
    "serial_delimiter_conv": "dropdown",
    "serial_readbuffer": "value",
}

# A handful of TS types that are not enums even though they are capitalized,
# so the generic "Capitalized = dropdown" rule below would misclassify them.
TYPE_KIND_OVERRIDES = {
    "Buffer": "value",
    "Image": "value",
    "Packet": "value",
    "Action": "value",
}

TOPLEVEL_NS_RE = re.compile(r"^(?:export\s+)?(?:declare\s+)?namespace\s+(\w+)\s*\{")
TOPLEVEL_CLOSE_RE = re.compile(r"^\}")
FUNC_START_RE = re.compile(r"^\s*(?:export\s+)?(?:public\s+)?function\s+(\w+)\s*\(")
FUNC_TAIL_RE = re.compile(r"^\s*(?::\s*([^;{]+?))?\s*[;{]\s*$")
ANNOT_RE = re.compile(r"^\s*//%\s?(.*)$")
BLOCKID_RE = re.compile(r'\bblockId=("(?:[^"\\]|\\.)*"|\S+)')
BLOCK_RE = re.compile(r'\bblock=("(?:[^"\\]|\\.)*")')
CALLBACK_RE = re.compile(r"^\([^()]*\)\s*=>\s*void$")


class RawBlock:
    def __init__(self, key: str, category: str, block_text: str, params: dict[str, str]):
        self.key = key
        self.category = category
        self.block_text = block_text
        self.params = params  # name -> TS type, callback params already removed
        self.shape = "stack"


def split_top_level(params_raw: str) -> list[str]:
    parts: list[str] = []
    depth = 0
    current = ""
    for ch in params_raw:
        if ch in "([{<":
            depth += 1
        elif ch in ")]}>":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(current)
            current = ""
        else:
            current += ch
    if current.strip():
        parts.append(current)
    return parts


def parse_params(params_raw: str) -> tuple[dict[str, str], bool]:
    """Return {name: TS type} plus whether a `() => void` callback was seen."""
    params: dict[str, str] = {}
    has_callback = False
    for part in split_top_level(params_raw):
        part = part.strip()
        if not part or ":" not in part:
            continue
        name, type_part = part.split(":", 1)
        name = name.strip().rstrip("?")
        type_part = type_part.strip()
        if "=>" not in type_part:
            type_part = type_part.split("=")[0].strip()  # drop a trailing `= default`
        if CALLBACK_RE.match(type_part):
            has_callback = True
            continue
        params[name] = type_part
    return params, has_callback


def match_function_decl(line: str) -> tuple[str, str, str | None] | None:
    """Match a one-line `function name(params): ret;`/`{` declaration.

    Not a single regex because a callback parameter's own `() => void`
    carries a paren pair of its own — `\\(([^)]*)\\)` stops at the first
    `)` it sees, which is the callback's, not the function's. The
    parameter list is found by counting parens by hand instead.
    """
    start = FUNC_START_RE.match(line)
    if not start:
        return None
    name = start.group(1)
    depth = 1
    index = start.end()
    while index < len(line) and depth > 0:
        if line[index] == "(":
            depth += 1
        elif line[index] == ")":
            depth -= 1
        index += 1
    if depth != 0:
        return None  # params spill onto another line — not handled
    params_raw = line[start.end() : index - 1]
    tail = FUNC_TAIL_RE.match(line[index:])
    if not tail:
        return None
    return name, params_raw, tail.group(1)


def extract_blocks(text: str) -> list[RawBlock]:
    blocks: list[RawBlock] = []
    current_ns: str | None = None
    pending: list[str] = []
    for line in text.split("\n"):
        ns_match = TOPLEVEL_NS_RE.match(line)
        if ns_match:
            current_ns = ns_match.group(1)
            pending = []
            continue
        if TOPLEVEL_CLOSE_RE.match(line):
            current_ns = None
            pending = []
            continue
        annot_match = ANNOT_RE.match(line)
        if annot_match:
            pending.append(annot_match.group(1))
            continue
        func_match = match_function_decl(line)
        if func_match and pending and current_ns and current_ns not in SKIP_NAMESPACES:
            name, params_raw, ret = func_match
            annot = " ".join(pending)
            pending = []
            if name.startswith("_"):
                continue  # internal shadow-only helper, not a toolbox block
            if re.search(r"\b(deprecated|blockHidden)\b", annot) or re.search(r"\bhidden\s*=\s*1\b", annot):
                continue
            block_match = BLOCK_RE.search(annot)
            if not block_match:
                continue  # annotated, but not exposed as its own block text
            raw_text = json.loads(block_match.group(1))
            blockid_match = BLOCKID_RE.search(annot)
            if not blockid_match:
                key = f"{current_ns}.{name}"
            else:
                raw_id = blockid_match.group(1)
                key = json.loads(raw_id) if raw_id.startswith('"') else raw_id.strip('"')
            params, has_callback = parse_params(params_raw)
            ret = (ret or "void").strip()
            block = RawBlock(key, current_ns.lower(), raw_text, params)
            if has_callback:
                block.shape = "hat"
            elif ret == "boolean":
                block.shape = "boolean"
            elif ret not in ("void", ""):
                block.shape = "reporter"
            else:
                block.shape = "stack"
            blocks.append(block)
            continue
        if line.strip() and not line.strip().startswith(("/**", "*", "//")):
            pending = []
    return blocks


TOKEN_RE = re.compile(r"[%$](\w+)(?:=([\w.]+))?")


def clean_and_slot(raw_text: str, params: dict[str, str]) -> tuple[str, list[str]] | None:
    """Turn a raw `block="..."` template into (text with %1.., slot kinds)."""
    # `||` marks the start of an optional (expandable) tail; only the part
    # before it is ever typed on a worksheet, so it is dropped outright.
    visible = raw_text.split("||")[0]
    tokens = TOKEN_RE.findall(visible)
    # `|` is MakeCode's own line-break hint inside the toolbox flyout; it is
    # not part of the text an author types.
    cleaned = visible.replace("|", " ")
    cleaned = re.sub(r"\s+", " ", cleaned).strip()

    slots: list[str] = []
    counter = 0

    def replace(match: re.Match[str]) -> str:
        nonlocal counter
        counter += 1
        return f"%{counter}"

    positioned = re.sub(r"[%$]\w+(?:=[\w.]+)?", replace, cleaned)

    if not tokens:
        return positioned, []
    if re.fullmatch(r"(\s*%\d+\s*)+", positioned):
        return None  # placeholders alone would match any bare input

    # MakeCode's block-text placeholder names do not always match the TS
    # parameter names verbatim (`%NAME` for a `button: Button` parameter is
    # common) — but their left-to-right order does match the declaration
    # order (aside from the trailing callback, already removed from
    # `params`). Try an exact/case-insensitive name match first and fall
    # back to position.
    ordered_params = list(params.items())
    for index, (name, shadow) in enumerate(tokens):
        if shadow:
            slots.append(SHADOW_KIND.get(shadow, "value"))
            continue
        ts_type = params.get(name)
        if ts_type is None:
            for pname, ptype in ordered_params:
                if pname.lower() == name.lower():
                    ts_type = ptype
                    break
        if ts_type is None and index < len(ordered_params):
            ts_type = ordered_params[index][1]
        if ts_type is None:
            slots.append("value")
            continue
        base = ts_type.rstrip("[]")
        if base in TYPE_KIND_OVERRIDES:
            slots.append(TYPE_KIND_OVERRIDES[base])
        elif base == "boolean":
            slots.append("field")
        elif base in ("number", "int32", "string"):
            slots.append("value")
        elif re.match(r"^[A-Z]", base):
            slots.append("dropdown")  # enum type — Button, Gesture, DigitalPin, ...
        else:
            slots.append("value")
    return positioned, slots


def renumber(raw_text: str) -> str:
    """Turn a hand-written `%name`-style template into %1/%2/... form, the
    same way clean_and_slot() does for scraped English text. Used for the
    curated German text in GERMAN_TEXT, which is written with the same
    named tokens as the English source it translates (see that table's
    docstring) so the two stay in the same left-to-right slot order."""
    visible = raw_text.split("||")[0]
    cleaned = re.sub(r"\s+", " ", visible.replace("|", " ")).strip()
    counter = 0

    def replace(match: re.Match[str]) -> str:
        nonlocal counter
        counter += 1
        return f"%{counter}"

    return re.sub(r"[%$]\w+(?:=[\w.]+)?", replace, cleaned)


# --------------------------------------------------------------------------
# The blocks pxt (the engine) builds itself, not any target: loops, logic,
# math, variables, arrays, text, functions. Transcribed from
# https://github.com/microsoft/pxt pxtlib/blocks.ts `cacheBlockDefinitions`
# (pinned at the same commit as the rest of this run's pxt-derived data
# would be if we fetched it — pxt itself is not downloaded because this
# table is hand-built the same way scripts/blockly-data/generate.py
# transcribes Blockly's five procedure blocks: the source is imperative
# TypeScript, not JSON, so scraping it would mean interpreting code).
#
# English is pxtlib/blocks.ts's own `Util.blf(...)` argument. German was
# read from the same table the running editor uses for these particular
# strings — unlike the per-namespace blocks below, pxt routes these through
# its general-purpose UI string table (Util.blf/lf()), which the public
# `/api/translations?filename=target-strings.json` endpoint *does* serve,
# and which was captured from a live makecode.microbit.org session in
# German while developing this script (`pxt.Util.getLocalizedStrings()`).
# Every entry here was matched verbatim against that capture.
#
# pxtlib declares `controls_simple_for`/`controls_for_of` as text-identical
# twins of `pxt_controls_for`/`pxt_controls_for_of` (same message, same
# category — they differ only in which internal Blockly field type backs
# the loop variable). Kept apart they would be indistinguishable to a text
# matcher, so — the same rule scripts/blockly-data/generate.py applies to
# Blockly's own `controls_repeat` vs. `controls_repeat_ext` — only the
# `pxt_`-prefixed one is kept here.
BUILTIN_BLOCKS = {
    "device_while": {
        "en": "while %1", "de": "während %1", "category": "loops", "shape": "c-block",
        "slots": ["value"], "mouth_en": "do", "mouth_de": "mache",
    },
    "pxt_controls_for": {
        "en": "for %1 from 0 to %2", "de": "für %1 von 0 bis %2", "category": "loops", "shape": "c-block",
        "slots": ["field", "value"], "mouth_en": "do", "mouth_de": "mache",
    },
    "pxt_controls_for_of": {
        "en": "for element %1 of %2", "de": "für Element %1 von %2", "category": "loops", "shape": "c-block",
        "slots": ["field", "value"], "mouth_en": "do", "mouth_de": "mache",
    },
    "controls_repeat_ext": {
        "en": "repeat %1 times", "de": "%1-mal wiederholen", "category": "loops", "shape": "c-block",
        "slots": ["value"], "mouth_en": "do", "mouth_de": "mache",
    },
    "pxt_pause_until": {
        "en": "pause until %1", "de": "warten bis %1", "category": "loops", "shape": "stack",
        "slots": ["value"],
    },
    "pxt_break": {"en": "break", "de": "abbrechen", "category": "loops", "shape": "cap", "slots": []},
    "pxt_continue": {"en": "continue", "de": "fortsetzen", "category": "loops", "shape": "cap", "slots": []},
    "pxt_on_start": {
        "en": "on start", "de": "beim Start", "category": "loops", "shape": "hat", "slots": [],
    },
    "controls_if": {
        "en": "if %1", "de": "wenn %1", "category": "logic", "shape": "c-block",
        "slots": ["value"], "mouth_en": "then", "mouth_de": "dann",
    },
    "logic_compare_eq": {"en": "%1 = %2", "de": "%1 = %2", "category": "logic", "shape": "boolean", "slots": ["value", "value"]},
    "logic_compare_ne": {"en": "%1 ≠ %2", "de": "%1 ≠ %2", "category": "logic", "shape": "boolean", "slots": ["value", "value"]},
    "logic_compare_lt": {"en": "%1 < %2", "de": "%1 < %2", "category": "logic", "shape": "boolean", "slots": ["value", "value"]},
    "logic_compare_le": {"en": "%1 ≤ %2", "de": "%1 ≤ %2", "category": "logic", "shape": "boolean", "slots": ["value", "value"]},
    "logic_compare_gt": {"en": "%1 > %2", "de": "%1 > %2", "category": "logic", "shape": "boolean", "slots": ["value", "value"]},
    "logic_compare_ge": {"en": "%1 ≥ %2", "de": "%1 ≥ %2", "category": "logic", "shape": "boolean", "slots": ["value", "value"]},
    "logic_operation_and": {"en": "%1 and %2", "de": "%1 und %2", "category": "logic", "shape": "boolean", "slots": ["value", "value"]},
    "logic_operation_or": {"en": "%1 or %2", "de": "%1 oder %2", "category": "logic", "shape": "boolean", "slots": ["value", "value"]},
    "logic_negate": {"en": "not %1", "de": "nicht %1", "category": "logic", "shape": "boolean", "slots": ["value"]},
    "math_arithmetic_add": {"en": "%1 + %2", "de": "%1 + %2", "category": "math", "shape": "reporter", "slots": ["value", "value"], "inline": True},
    "math_arithmetic_sub": {"en": "%1 - %2", "de": "%1 - %2", "category": "math", "shape": "reporter", "slots": ["value", "value"], "inline": True},
    "math_arithmetic_mul": {"en": "%1 × %2", "de": "%1 × %2", "category": "math", "shape": "reporter", "slots": ["value", "value"], "inline": True},
    "math_arithmetic_div": {"en": "%1 / %2", "de": "%1 / %2", "category": "math", "shape": "reporter", "slots": ["value", "value"], "inline": True},
    "math_arithmetic_pow": {"en": "%1 ** %2", "de": "%1 ** %2", "category": "math", "shape": "reporter", "slots": ["value", "value"], "inline": True},
    "math_modulo": {"en": "remainder of %1 / %2", "de": "Rest von %1 / %2", "category": "math", "shape": "reporter", "slots": ["value", "value"]},
    "math_op2": {"en": "%1 of %2 and %3", "de": "%1 von %2 und %3", "category": "math", "shape": "reporter", "slots": ["dropdown", "value", "value"]},
    "math_op3": {"en": "absolute of %1", "de": "Betrag von %1", "category": "math", "shape": "reporter", "slots": ["value"]},
    "variables_set": {"en": "set %1 to %2", "de": "setze %1 auf %2", "category": "variables", "shape": "stack", "slots": ["dropdown", "value"]},
    "variables_change": {"en": "change %1 by %2", "de": "ändere %1 um %2", "category": "variables", "shape": "stack", "slots": ["dropdown", "value"]},
    "lists_create_with": {"en": "array of", "de": "Array von", "category": "arrays", "shape": "reporter", "slots": [], "icon": "mutator"},
    "lists_create_empty": {"en": "empty array", "de": "leeres Array", "category": "arrays", "shape": "reporter", "slots": []},
    "lists_length": {"en": "length of array %1", "de": "Array-Länge %1", "category": "arrays", "shape": "reporter", "slots": ["value"]},
    "lists_index_get": {"en": "%1 get value at %2", "de": "%1 rufe Wert ab bei %2", "category": "arrays", "shape": "reporter", "slots": ["value", "value"]},
    "lists_index_set": {"en": "%1 set value at %2 to %3", "de": "%1 Wert festlegen bei %2 auf %3", "category": "arrays", "shape": "stack", "slots": ["value", "value", "value"]},
    "text_length": {"en": "length of %1", "de": "Länge von %1", "category": "text", "shape": "reporter", "slots": ["value"]},
    "text_join": {"en": "join", "de": "verbinde", "category": "text", "shape": "reporter", "slots": [], "icon": "mutator"},
    "procedures_defnoreturn": {"en": "function", "de": "Funktion", "category": "functions", "shape": "cap", "slots": []},
    "procedures_callnoreturn": {"en": "call function", "de": "Funktion aufrufen", "category": "functions", "shape": "stack", "slots": []},
    # pxtlib's function_return block carries two message variants
    # (`message_with_value`/`message_no_value`) chosen at runtime by
    # whether the enclosing function returns a value; only one text can
    # live under one block id here, so the more general (with-value) form
    # is kept.
    "function_return": {"en": "return %1", "de": "%1 zurückgeben", "category": "functions", "shape": "cap", "slots": ["value"]},
}


# --------------------------------------------------------------------------
# Hand-curated German text for the custom namespace blocks. Keyed the same
# way the generated spec is: MakeCode's own blockId, falling back to
# `namespace.function` when a block declares none. See the module docstring
# for why this table exists instead of a scraped one. Entries marked
# "(live)" were read directly off a running makecode.microbit.org / .cc
# session in German while developing this script; the rest reflect
# well-established MakeCode DE wording that was not re-verified live this
# session — both kinds are listed separately in REPORT.md.
GERMAN_TEXT = {
    # -- live-verified against the running German editor --
    "device_show_number": "zeige Zahl %number",  # (live)
    "basic_show_icon": "zeige Symbol %i",  # (live)
    "device_print_message": "zeige Text %text",  # (live)
    "device_pause": "pausiere (ms) %pause",  # (live)
    "device_forever": "dauerhaft",  # (live)
    "device_plot": "Zeichne x %x y %y",  # (live)
    "device_button_event": "wenn Knopf %NAME geklickt",  # (live)
    "device_get_light_level": "Lichtstärke",  # (live)
    # -- established DE wording, not re-verified live this session --
    "device_clear_display": "Bildschirm löschen",
    "device_show_leds": "zeige LEDs",
    "device_get_brightness": "Helligkeit",
    "device_set_brightness": "setze Helligkeit auf %value",
    "device_stop_animation": "Animation stoppen",
    "device_unplot": "lösche x %x y %y",
    "device_get_button2": "Knopf %NAME ist gedrückt",
    "device_acceleration": "Beschleunigung (mg) %NAME",
    "device_heading": "Kompasspeilung (°)",
    "device_temperature": "Temperatur (°C)",
    "device_get_digital_pin": "digital lesen Pin %name",
    "device_set_digital_pin": "digital schreiben Pin %name auf %value",
    "device_get_analog_pin": "analog lesen Pin %name",
    "device_set_analog_pin": "analog schreiben Pin %name auf %value",
    "control_reset": "neu starten",
    "control_running_time": "Laufzeit (ms)",
    "control_in_background": "im Hintergrund ausführen",
    "serial_writestring": "seriell schreibe Text %text",
    "serial_writenumber": "seriell schreibe Zahl %value",
    "serial_writeline": "seriell schreibe Zeile %text",
    "radio_set_group": "lege Funkgruppe auf %ID fest",
    "game_score": "Punktestand",
    "game_add_score": "ändere Punktestand um %points",
    "game_set_score": "setze Punktestand auf %points",
    "game_game_over": "Spiel vorbei",
    # -- Calliope mini extras --
    "block_dual_motor": "Motor %motor mit %percent \\%",
    "device_set_led_colors": "setze RGB-LED auf %color1 %color2 %color3",
}


# --------------------------------------------------------------------------
# Category colours. Read from each namespace's own `//% color=...`
# annotation (see the file + line noted per entry) or, for the blocks pxt
# itself builds, from the target's `appTheme.blockColors` in pxtarget.json.
# --------------------------------------------------------------------------

MICROBIT_COLORS = {
    # generic pxt categories — pxtarget.json appTheme.blockColors
    "logic": "#00A4A6",
    "loops": "#00AA00",
    "math": "#9400D3",
    "variables": "#DC143C",
    "text": "#B8860B",
    "functions": "#3455DB",
    "arrays": "#E65722",
    # micro:bit's own namespaces — libs/core/*.ts, libs/radio/*.ts
    "basic": "#1E90FF",   # shims.d.ts:132 (namespace basic)
    "input": "#D400D4",   # input.ts:4
    "led": "#5C2D91",     # led.ts:4
    "music": "#E63022",   # music.ts:179
    "pins": "#B22222",    # pins.ts:4
    "serial": "#002050",  # serial.ts:29
    "game": "#007A4B",    # game.ts:24
    "control": "#333333",  # control.ts:4
    "radio": "#E3008C",   # radio/shims.d.ts:5
    "images": "#7600A8",  # shims.d.ts:7
}

# Calliope mini overrides most of these in its own pxtarget.json
# appTheme.blockColors (its LED/RGB blocks and buttons are physically
# different, hence the different palette); anything not listed here is
# inherited from the micro:bit profile unchanged.
CALLIOPE_COLORS = {
    "basic": "#54C9C9",
    "input": "#C94600",
    "music": "#DF4600",
    "led": "#8169E6",
    "radio": "#E3008C",
    "motors": "#008272",
    "logic": "#006970",
    "loops": "#107C10",
    "math": "#712672",
    "variables": "#A80000",
    "text": "#996600",
    "functions": "#005A9E",
    "arrays": "#E65722",
}


# --------------------------------------------------------------------------
# Assembling specs
# --------------------------------------------------------------------------


def build_catalog(files: list[tuple[str, None]]) -> tuple[dict, list[str]]:
    """Return ({block_id: spec}, [block ids dropped as placeholder-only])."""
    specs: dict[str, dict] = {}
    dropped: list[str] = []
    for filename, _ in files:
        text = (SOURCES / filename).read_text(encoding="utf-8")
        for raw in extract_blocks(text):
            if raw.key in specs:
                continue  # first definition wins (shims vs. TS wrapper dupes)
            result = clean_and_slot(raw.block_text, raw.params)
            if result is None:
                dropped.append(raw.key)
                continue
            text_positioned, slots = result
            de_text = GERMAN_TEXT.get(raw.key)
            specs[raw.key] = {
                "en": text_positioned,
                "de": renumber(de_text) if de_text else text_positioned,
                "de_source": "curated" if de_text else "en-fallback",
                "shape": raw.shape,
                "category": raw.category,
                "slots": slots,
            }
    return specs, dropped


def escape(value: str) -> str:
    return value.replace("\\", "\\\\").replace('"', '\\"')


def write_locale(
    path: Path,
    title: str,
    license_note: str,
    specs: dict,
    lang: str,
    inherits: str | None = None,
    else_word: str | None = None,
    aliases: dict | None = None,
) -> None:
    """Write one locale TOML. `lang` is "en" or "de"; picks which text/mouth
    keys to read out of each spec and which end/else marker words to use."""
    markers = {"scratchblocks:end": "end" if lang == "en" else "ende", "control_else": else_word or ("else" if lang == "en" else "sonst")}
    marker_shapes = {"control_else": ("celse", "logic"), "scratchblocks:end": ("cend", "logic")}

    lines = [
        f"# {title}",
        "# Generated by scripts/makecode-data/generate.py — do not edit by hand.",
        f"# {license_note}",
        "",
    ]
    if inherits:
        lines += ["# Blocks not listed here are looked up in the base locale.", f'inherits = "{inherits}"', ""]

    lines.append("[specs]")
    for key, value in markers.items():
        lines.append(f'"{key}" = "{escape(value)}"' if ":" in key else f'{key} = "{escape(value)}"')
    for block_id in sorted(specs):
        lines.append(f'{block_id} = "{escape(specs[block_id][lang])}"')

    lines += ["", "[shapes]"]
    for key in markers:
        lines.append(f'"{key}" = "{marker_shapes[key][0]}"' if ":" in key else f'{key} = "{marker_shapes[key][0]}"')
    for block_id in sorted(specs):
        lines.append(f'{block_id} = "{specs[block_id]["shape"]}"')

    lines += ["", "[categories]"]
    for key in markers:
        lines.append(f'"{key}" = "{marker_shapes[key][1]}"' if ":" in key else f'{key} = "{marker_shapes[key][1]}"')
    for block_id in sorted(specs):
        lines.append(f'{block_id} = "{specs[block_id]["category"]}"')

    slots = {k: v["slots"] for k, v in specs.items() if v.get("slots")}
    if slots:
        lines += ["", "# What each %n is: a value socket, a field box, a dropdown, a statement mouth.", "[slots]"]
        for block_id in sorted(slots):
            lines.append(f'{block_id} = "{",".join(slots[block_id])}"')

    inline = {k: v["inline"] for k, v in specs.items() if "inline" in v}
    if inline:
        lines += ["", "# inputsInline as the editor has it: false puts every input on a row of its own.", "[inline]"]
        for block_id in sorted(inline):
            lines.append(f"{block_id} = {str(bool(inline[block_id])).lower()}")

    mouth_key = f"mouth_{lang}"
    mouths = {k: v[mouth_key] for k, v in specs.items() if v.get(mouth_key)}
    if mouths:
        lines += ["", "# Label drawn next to the mouth of a C-block.", "[mouths]"]
        for block_id in sorted(mouths):
            lines.append(f'{block_id} = "{escape(mouths[block_id])}"')

    icons = {k: v["icon"] for k, v in specs.items() if v.get("icon")}
    if icons:
        lines += ["", "# Icon in the first row: the mutator gear.", "[icons]"]
        for block_id in sorted(icons):
            lines.append(f'{block_id} = "{icons[block_id]}"')

    if aliases:

        lines += ["", "# Other spellings an author may use for a block.", "[aliases]"]

        for alias_spec, block_id in sorted(aliases.items()):

            lines.append(f'"{escape(alias_spec)}" = "{block_id}"')


    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def write_profile(path: Path, name: str, title: str, note: str, colors: dict, inherits: str | None) -> None:
    lines = [
        f"# {title}",
        "# Generated by scripts/makecode-data/generate.py — do not edit by hand.",
        f"# {note}",
        "",
        f'name = "{name}"',
    ]
    if inherits:
        lines.append(f'inherits = "{inherits}"')
    lines += ["", "[colors]"]
    for key in sorted(colors):
        lines.append(f'{key} = "{colors[key]}"')
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


# The boolean literal: a shadow in the editor (a 32px hexagon with a
# dropdown), typed as `<wahr>` / `<true>` here.
BOOLEAN_LITERALS = {
    "logic_boolean_true": {"en": "true", "de": "wahr", "shape": "boolean", "category": "logic", "slots": []},
    "logic_boolean_false": {"en": "false", "de": "falsch", "shape": "boolean", "category": "logic", "slots": []},
}


# Shadow blocks the toolbox never lists on their own: the melody of
# `spiele %1 %2` is a `music_string_playable` shadow holding the melody
# editor (`melody_editor`, an 8-note grid) and a tempo; read off the block
# in the running editor (fields: Melodie / mit Tempo / (bpm)).
EDITOR_BLOCKS = {
    "music_string_playable": {"en": "melody %1 at tempo %2 (bpm)", "de": "Melodie %1 mit Tempo %2 (bpm)", "shape": "reporter", "category": "music", "slots": ["melody", "value"], "inline": True},
}


def merge_builtins(specs: dict) -> None:
    """Fold BUILTIN_BLOCKS (pxt's own loop/logic/math/... blocks) into a
    per-language-agnostic spec dict shaped like build_catalog()'s output."""
    for block_id, entry in {**BUILTIN_BLOCKS, **BOOLEAN_LITERALS, **EDITOR_BLOCKS}.items():
        merged = {
            "en": entry["en"],
            "de": entry["de"],
            "de_source": "curated",
            "shape": entry["shape"],
            "category": entry["category"],
            "slots": entry["slots"],
        }
        if "mouth_en" in entry:
            merged["mouth_en"] = entry["mouth_en"]
            merged["mouth_de"] = entry["mouth_de"]
        if entry.get("inline"):
            merged["inline"] = True
        if entry.get("icon"):
            merged["icon"] = entry["icon"]
        specs[block_id] = merged


# ---------------------------------------------------------------------------
# The running editors as the source of truth for block texts
#
# MakeCode loads its translations at run time and publishes no per-block
# string files, so sources/live/{target}-{lang}.jsonl hold what the editors
# actually show: one line per toolbox block, read through Blockly's API by
# sources/live/crawl.js (see there). They carry the exact text in both
# languages, the slot kinds, the mouth labels and the colour of every block
# in the toolbox — including blocks the annotation extractor cannot see
# (methods, melody editors). Where a live record exists it wins over the
# source-derived one; the source catalog still supplies blocks the toolbox
# hides.
# ---------------------------------------------------------------------------

LIVE = SOURCES / "live"
LIVE_SLOT_KIND = {"value": "value", "boolean": "value", "dropdown": "dropdown", "number": "field", "text": "field", "variable": "dropdown", "checkbox": "field", "field": "field"}
LIVE_SHAPE = {"stack": "stack", "c-block": "c-block", "cap": "cap", "event": "c-block hat", "hat": "hat", "reporter": "reporter", "boolean": "boolean"}
PLACEHOLDER_ONLY_RE = re.compile(r"^(%\d+\s*)+$")


def load_live(target: str, lang: str) -> dict[str, dict]:
    """block id -> live record; for an id seen twice, the variant with the
    most mouths (the if/else block carries the else label)."""
    path = LIVE / f"{target}-{lang}.jsonl"
    records: dict[str, dict] = {}
    if not path.exists():
        return records
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        rec = json.loads(line)
        # one micro:bit block id carries a stray quote (control_event_value")
        rec["t"] = rec["t"].strip('"')
        prev = records.get(rec["t"])
        if prev is None or len(rec.get("mouths", [])) > len(prev.get("mouths", [])):
            records[rec["t"]] = rec
    return records


def colour_category(block_id: str, colour: str, colours: dict[str, str], fallback: str) -> str:
    """A block keeps its own colour wherever the toolbox shows it —
    variables_set is red in the Arrays category too — so the colour names
    the category when the palette has it. Two categories may share a colour
    (Calliope's pins and variables); then the block id decides, then the
    toolbox category, and an unresolved tie keeps the toolbox category."""
    wanted = colour.lower()
    matches = sorted(cat for cat, hex_ in colours.items() if hex_.lower() == wanted)
    if len(matches) == 1:
        return matches[0]
    for cat in matches:
        if block_id.startswith(cat):
            return cat
    return fallback


def live_colours(target: str) -> dict[str, str]:
    """Each toolbox category's colour as the editor shows it: the colour most
    of its blocks carry."""
    from collections import Counter
    votes: dict[str, Counter] = {}
    for lang in ("en", "de"):
        for rec in load_live(target, lang).values():
            cat = rec.get("cat", "").lower()
            if cat and rec.get("c"):
                votes.setdefault(cat, Counter())[rec["c"].lower()] += 1
    return {cat: counter.most_common(1)[0][0] for cat, counter in votes.items()}


def live_entry(rec_en: dict | None, rec_de: dict | None, colours: dict[str, str] | None = None) -> dict | None:
    base = rec_en or rec_de
    if base is None:
        return None
    text_en = (rec_en or rec_de)["spec"].strip()
    text_de = (rec_de or rec_en)["spec"].strip()
    if not text_en or PLACEHOLDER_ONLY_RE.match(text_en):
        return None
    slots = [LIVE_SLOT_KIND.get(k, "field") for k in base.get("slots", [])]
    # A field whose default is a backtick grid is the 5x5 LED matrix editor;
    # a value whose default shadow is the melody block gets the melody editor
    # through that shadow (see EDITOR_BLOCKS).
    for i, default in enumerate(base.get("def", [])):
        if i < len(slots) and default.startswith("`"):
            slots[i] = "matrix"
    entry = {
        "en": text_en,
        "de": text_de,
        "de_source": "live" if rec_de else "en-fallback",
        "shape": LIVE_SHAPE.get(base["s"], "stack"),
        "category": colour_category(base["t"], base.get("c", ""), colours or {}, base.get("cat", "").lower() or "advanced"),
        "slots": slots,
        "colour": base.get("c", ""),
    }
    # inputsInline as the editor has it; false puts every input on a row of
    # its own (the LED matrix under its label).
    entry["inline"] = bool(base.get("inline"))
    mouths_en = (rec_en or rec_de).get("mouths", [])
    mouths_de = (rec_de or rec_en).get("mouths", [])
    if mouths_en and mouths_en[0].get("label"):
        entry["mouth_en"] = mouths_en[0]["label"]
        entry["mouth_de"] = (mouths_de[0].get("label") if mouths_de else "") or mouths_en[0]["label"]
    if base["t"] == "controls_if":
        entry["icon"] = "mutator"
    return entry


# How an author may spell an operator block, beside the glyph the editor shows.
OPERATOR_ALIASES = {
    "%1 * %2": "math_arithmetic_mul",
    "%1 ÷ %2": "math_arithmetic_div",
    "%1 ^ %2": "math_arithmetic_pow",
    "%1 >= %2": "logic_compare_ge",
    "%1 <= %2": "logic_compare_le",
    "%1 != %2": "logic_compare_ne",
    "%1 == %2": "logic_compare_eq",
}


def apply_live(specs: dict, target: str) -> dict:
    """Lay the live records over the source catalog. Returns the else
    markers per language, read off the if/else block."""
    en = load_live(target, "en")
    de = load_live(target, "de")
    static = MICROBIT_COLORS if target == "microbit" else {**MICROBIT_COLORS, **CALLIOPE_COLORS}
    colours = {**{k: v.lower() for k, v in static.items()}, **live_colours(target)}
    for block_id in sorted(set(en) | set(de)):
        entry = live_entry(en.get(block_id), de.get(block_id), colours)
        if entry is None:
            continue
        specs[block_id] = entry
    markers = {}
    for lang, recs in (("en", en), ("de", de)):
        rec = recs.get("controls_if")
        if rec and len(rec.get("mouths", [])) > 1:
            markers[lang] = rec["mouths"][1]["head"].strip()
    return markers


def drop_twins(specs: dict, lang: str) -> list[str]:
    """Two ids with the same text cannot both be matched; keep the first."""
    seen: dict[str, str] = {}
    dropped = []
    for block_id in sorted(specs):
        text = specs[block_id][lang]
        if text in seen:
            dropped.append(f"{block_id} (same {lang} text as {seen[text]})")
        else:
            seen[text] = block_id
    for item in dropped:
        specs.pop(item.split(" ")[0], None)
    return dropped


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--offline", action="store_true", help="use the cached sources only")
    args = parser.parse_args()

    print("Sources")
    fetch(args.offline)

    (OUT / "locales").mkdir(parents=True, exist_ok=True)
    (OUT / "profiles").mkdir(parents=True, exist_ok=True)

    print("micro:bit namespace blocks")
    microbit_specs, microbit_dropped = build_catalog(MICROBIT_FILES)
    merge_builtins(microbit_specs)
    microbit_markers = apply_live(microbit_specs, "microbit")
    twins = {lang: drop_twins(microbit_specs, lang) for lang in ("en", "de")}
    print(f"  {len(microbit_specs)} blocks")

    print("Calliope mini extra blocks")
    calliope_specs, calliope_dropped = build_catalog(CALLIOPE_FILES)
    calliope_markers = apply_live(calliope_specs, "calliope")
    for lang in ("en", "de"):
        drop_twins(calliope_specs, lang)
    # Blocks calliope shares verbatim with micro:bit (basic/input/led/...)
    # already come from the inherited locale; only genuinely new or
    # overridden ids are kept here.
    def differs(k: str, v: dict) -> bool:
        m = microbit_specs.get(k)
        if m is None:
            return True
        return any(v.get(f) != m.get(f) for f in ("en", "de", "shape", "slots", "mouth_en", "mouth_de", "category"))
    calliope_only = {k: v for k, v in calliope_specs.items() if differs(k, v)}
    print(f"  {len(calliope_only)} blocks")

    LICENSE_NOTE = (
        "Blocks from pxt, pxt-microbit and pxt-calliope (all MIT); "
        f"pinned at pxt-microbit@{MICROBIT_SHA[:10]}, pxt-calliope@{CALLIOPE_SHA[:10]}."
    )

    for lang, title_word in (("en", "English"), ("de", "German")):
        write_locale(
            OUT / "locales" / f"makecode-{lang}.toml",
            f"MakeCode (micro:bit), {title_word}.",
            LICENSE_NOTE,
            microbit_specs,
            lang,
            else_word=microbit_markers.get(lang),
            aliases={a: b for a, b in OPERATOR_ALIASES.items() if b in microbit_specs},
        )
        write_locale(
            OUT / "locales" / f"makecode-calliope-{lang}.toml",
            f"MakeCode (Calliope mini), {title_word} — extra and overridden blocks.",
            LICENSE_NOTE,
            calliope_only,
            lang,
            inherits=f"makecode-{lang}",
            else_word=calliope_markers.get(lang) or microbit_markers.get(lang),
        )

    print("Profiles")
    write_profile(
        OUT / "profiles" / "makecode.toml",
        "makecode",
        "MakeCode (micro:bit).",
        "Colours from pxt-microbit's pxtarget.json appTheme.blockColors plus "
        "each namespace's own //% color=... annotation (see MICROBIT_COLORS "
        "in the generator for the exact file/line of each).",
        {**MICROBIT_COLORS, **{k: v.upper() for k, v in live_colours("microbit").items() if k in MICROBIT_COLORS or k not in ("advancedcollapsed", "addpackage")}},
        inherits=None,
    )
    write_profile(
        OUT / "profiles" / "makecode-calliope.toml",
        "makecode-calliope",
        "MakeCode (Calliope mini).",
        "Colours from pxt-calliope's own pxtarget.json appTheme.blockColors, "
        "which overrides most categories from the micro:bit palette; "
        "categories not listed there (pins, serial, control, game, images) "
        "are inherited unchanged.",
        {**CALLIOPE_COLORS, **{k: v.upper() for k, v in live_colours("calliope").items() if k not in ("advancedcollapsed", "addpackage")}},
        inherits="makecode",
    )

    all_specs = {**microbit_specs, **{f"calliope:{k}": v for k, v in calliope_only.items()}}
    untranslated = sorted(k for k, v in all_specs.items() if v["de_source"] == "en-fallback")
    live_count = sum(1 for v in all_specs.values() if v["de_source"] == "live")
    categories = sorted({v["category"] for v in all_specs.values()})
    shapes = sorted({v["shape"] for v in all_specs.values()})

    report = [
        "",
        "## MakeCode",
        "",
        "Regenerate with `python3 scripts/makecode-data/generate.py`.",
        "",
        "### Counts",
        "",
        "| Set | Blocks |",
        "| --- | ---: |",
        f"| micro:bit (incl. shared loops/logic/math/variables/arrays/text/functions) | {len(microbit_specs)} |",
        f"| Calliope mini, extra/overridden | {len(calliope_only)} |",
        f"| dropped as placeholder-only (micro:bit) | {len(microbit_dropped)} |",
        f"| dropped as placeholder-only (Calliope) | {len(calliope_dropped)} |",
        "",
        "### By category",
        "",
        "| Category | micro:bit | Calliope extra |",
        "| --- | ---: | ---: |",
    ]
    for cat in categories:
        report.append(
            f"| {cat} | {sum(1 for v in microbit_specs.values() if v['category'] == cat)} "
            f"| {sum(1 for v in calliope_only.values() if v['category'] == cat)} |"
        )
    report += ["", "### By shape", "", "| Shape | Count |", "| --- | ---: |"]
    for shape in shapes:
        report.append(f"| {shape} | {sum(1 for v in all_specs.values() if v['shape'] == shape)} |")

    report += [
        "",
        "### German coverage",
        "",
        "MakeCode does not publish its per-block translations as a file, and "
        "the public translation endpoint that serves the editor's general UI "
        "strings returns nothing for the per-block ones (see the generator's "
        "module docstring). The loop/logic/math/variables/arrays/text/"
        "functions blocks pxt itself builds *are* reproducibly sourced "
        "(pxt routes those through the general string table instead); "
        "everything else's German comes from a hand-curated table "
        "(GERMAN_TEXT in the generator), part of it read live off the "
        "running editor and part of it well-established MakeCode wording "
        "that was not re-verified live this session.",
        "",
        f"- {live_count} blocks with text read off the running editors (sources/live), in both languages",
        f"- {len(untranslated)} blocks with no German text — kept in English: "
        + (", ".join(untranslated) if untranslated else "none"),
        "",
        "### Twins dropped (same text as another block, first id kept)",
        "",
        *([f"- {t}" for t in twins["en"] + [x for x in twins["de"] if x not in twins["en"]]] or ["- none"]),
        "",
        "### Colour table",
        "",
        "| Category | micro:bit | Calliope mini |",
        "| --- | --- | --- |",
    ]
    for cat in sorted(set(MICROBIT_COLORS) | set(CALLIOPE_COLORS)):
        report.append(f"| {cat} | {MICROBIT_COLORS.get(cat, '(inherited)')} | {CALLIOPE_COLORS.get(cat, '(inherited)')} |")

    report_path = OUT / "REPORT.md"
    existing = report_path.read_text(encoding="utf-8") if report_path.exists() else ""
    # Replace a previously appended "## MakeCode" section on re-run, rather
    # than growing the file every time.
    marker = "\n## MakeCode\n"
    if marker in existing:
        existing = existing.split(marker)[0].rstrip("\n") + "\n"
    report_path.write_text(existing.rstrip("\n") + "\n" + "\n".join(report) + "\n", encoding="utf-8")

    print(f"\nWritten to {OUT}")
    print("See scripts/scratchblocks-wasm/data/dialects/REPORT.md (MakeCode section) for counts and gaps.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
