#!/usr/bin/env python3
"""Generate blockst block data for the Blockly dialect.

Two families of blocks end up on a jwinf.de worksheet:

  * the standard Blockly blocks (loops, logic, math, text, lists, variables,
    procedures), whose definitions live in Blockly itself and whose German
    wording comes from Blockly's own message tables;
  * the world blocks (robot, turtle), which come from France-IOI's
    bebras-modules — the library jwinf actually loads.

Both are turned into the TOML shape blockst already uses for Scratch:
a `[specs]` table mapping a block id to its localized text with %1
placeholders. Alongside them this writes the profile files that carry
geometry and palette.

Sources are downloaded once into sources/ and reused; pass --offline to
work from the cache alone.

    python3 scripts/blockly-data/generate.py

Licenses of the generated data: Blockly is Apache-2.0, bebras-modules is
MIT. Both are noted in the header of every generated file.
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

BLOCKLY_RAW = "https://raw.githubusercontent.com/google/blockly/master"
BEBRAS_RAW = "https://raw.githubusercontent.com/France-ioi/bebras-modules/master"

# Blockly ships its blocks split by category; colour.ts no longer exists.
BLOCK_FILES = [
    "loops",
    "logic",
    "math",
    "text",
    "lists",
    "variables",
    # variables_dynamic is left out on purpose: its blocks carry the same
    # German text as the plain variable blocks, so the two would be
    # indistinguishable to a text matcher, and jwinf does not use them.
    "procedures",
]

DOWNLOADS = {
    **{f"blocks_{name}.ts": f"{BLOCKLY_RAW}/blocks/{name}.ts" for name in BLOCK_FILES},
    "msg_de.json": f"{BLOCKLY_RAW}/msg/json/de.json",
    "msg_en.json": f"{BLOCKLY_RAW}/msg/json/en.json",
    # The Blockly generation jwinf actually loads, with its own German wording.
    "msg_de_classic.js": f"{BEBRAS_RAW}/ext/blockly/de.js",
    "robot_lib.js": f"{BEBRAS_RAW}/pemFioi/blocklyRobot_lib-1.1.js",
    "turtle_lib.js": f"{BEBRAS_RAW}/pemFioi/blocklyTurtle_lib.js",
}

# Blocks that read identically to another block and draw identically too, so
# a text matcher could only pick between them at random. Blockly itself calls
# controls_repeat_ext "preferred as it is more flexible".
SKIP_BLOCKS = {
    "controls_repeat",
    # Same text as controls_if ("falls %1"); whether an else branch is drawn
    # is decided by the author writing `sonst`, not by the block id.
    "controls_ifelse",
}

# Blockly's msg/messages.js declares some keys as synonyms of others rather
# than translating them separately — the "do" label of every loop is one
# string. The JSON tables leave the synonyms out, so they are applied here.
MESSAGE_SYNONYMS = {
    "CONTROLS_IF_MSG_THEN": "CONTROLS_REPEAT_INPUT_DO",
    "CONTROLS_FOREACH_INPUT_DO": "CONTROLS_REPEAT_INPUT_DO",
    "CONTROLS_FOR_INPUT_DO": "CONTROLS_REPEAT_INPUT_DO",
    "CONTROLS_WHILEUNTIL_INPUT_DO": "CONTROLS_REPEAT_INPUT_DO",
    "CONTROLS_IF_IF_TITLE_IF": "CONTROLS_IF_MSG_IF",
    "CONTROLS_IF_ELSEIF_TITLE_ELSEIF": "CONTROLS_IF_MSG_ELSEIF",
    "CONTROLS_IF_ELSE_TITLE_ELSE": "CONTROLS_IF_MSG_ELSE",
    "PROCEDURES_DEFRETURN_TITLE": "PROCEDURES_DEFNORETURN_TITLE",
    "PROCEDURES_DEFRETURN_PROCEDURE": "PROCEDURES_DEFNORETURN_PROCEDURE",
    "LISTS_GET_SUBLIST_INPUT_IN_LIST": "LISTS_GET_INDEX_INPUT_IN_LIST",
    "LISTS_SET_INDEX_INPUT_IN_LIST": "LISTS_GET_INDEX_INPUT_IN_LIST",
    "LISTS_INDEX_OF_INPUT_IN_LIST": "LISTS_GET_INDEX_INPUT_IN_LIST",
    "TEXT_CREATE_JOIN_ITEM_TITLE_ITEM": "VARIABLES_DEFAULT_NAME",
    "MATH_CHANGE_TITLE_ITEM": "VARIABLES_DEFAULT_NAME",
}


def with_synonyms(messages: dict[str, str]) -> dict[str, str]:
    out = dict(messages)
    for key, source in MESSAGE_SYNONYMS.items():
        if key not in out and source in out:
            out[key] = out[source]
    return out

# Blockly names its categories through block styles.
STYLE_TO_CATEGORY = {
    "loop_blocks": "schleifen",
    "logic_blocks": "logik",
    "math_blocks": "mathe",
    "text_blocks": "text",
    "list_blocks": "listen",
    "variable_blocks": "variablen",
    "procedure_blocks": "funktionen",
    "colour_blocks": "farben",
}

# France-IOI groups its blocks by context rather than by style.
BEBRAS_CATEGORY = {
    "robot": "aktionen",
    "turtle": "schildkroete",
    "printer": "ausgeben",
    "reader": "einlesen",
}


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
# Blockly standard blocks
# --------------------------------------------------------------------------


def balanced(text: str, open_index: int) -> str:
    """Return the body of the {...} that starts at `open_index`."""
    depth = 0
    in_string: str | None = None
    escaped = False
    for index in range(open_index, len(text)):
        char = text[index]
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == in_string:
                in_string = None
        elif char in "\"'":
            in_string = char
        elif char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return text[open_index + 1 : index]
    return ""


def js_objects(text: str, start: int) -> list[str]:
    """Return the top-level {...} chunks of the array starting at `start`.

    Written by hand rather than with a regex because the definitions nest
    braces and brackets several levels deep, and because they are JS object
    literals rather than JSON — quotes and trailing commas are normalized
    afterwards.
    """
    chunks: list[str] = []
    depth = 0
    begin = None
    in_string: str | None = None
    escaped = False

    index = start
    while index < len(text):
        char = text[index]
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == in_string:
                in_string = None
        elif char in "\"'":
            in_string = char
        elif char == "{":
            if depth == 0:
                begin = index
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0 and begin is not None:
                chunks.append(text[begin : index + 1])
                begin = None
        elif char == "]" and depth == 0:
            break
        index += 1
    return chunks


def js_to_json(chunk: str) -> str:
    """Turn a JS object literal into JSON."""
    # Drop line comments, which appear between definitions.
    chunk = re.sub(r"^\s*//.*$", "", chunk, flags=re.M)
    # Single-quoted strings become double-quoted ones.
    def requote(match: re.Match[str]) -> str:
        body = match.group(1).replace('"', '\\"').replace("\\'", "'")
        return f'"{body}"'

    chunk = re.sub(r"'((?:[^'\\]|\\.)*)'", requote, chunk)
    # Unquoted keys.
    chunk = re.sub(r"([{,]\s*)([A-Za-z_][A-Za-z0-9_]*)(\s*:)", r'\1"\2"\3', chunk)
    # Trailing commas.
    chunk = re.sub(r",(\s*[}\]])", r"\1", chunk)
    return chunk


# The procedure blocks are the one family Blockly builds imperatively, in
# `init` functions rather than as JSON, because their fields depend on a
# mutator. Parsing that would mean interpreting the calls; there are five
# blocks, so they are declared here instead — with the same message keys the
# upstream code passes to appendField.
PROCEDURE_BLOCKS = [
    {
        "type": "procedures_defnoreturn",
        "message0": "%{BKY_PROCEDURES_DEFNORETURN_TITLE} %1",
        "args0": [{"type": "field_input", "name": "NAME"}, {"type": "input_statement", "name": "STACK"}],
        "style": "procedure_blocks",
    },
    {
        "type": "procedures_defreturn",
        "message0": "%{BKY_PROCEDURES_DEFNORETURN_TITLE} %1",
        "args0": [{"type": "field_input", "name": "NAME"}, {"type": "input_statement", "name": "STACK"}],
        "message1": "%{BKY_PROCEDURES_DEFRETURN_RETURN} %1",
        "args1": [{"type": "input_value", "name": "RETURN"}],
        "style": "procedure_blocks",
    },
    {
        "type": "procedures_callnoreturn",
        "message0": "%1",
        "args0": [{"type": "field_label", "name": "NAME"}],
        "previousStatement": None,
        "nextStatement": None,
        "style": "procedure_blocks",
    },
    {
        "type": "procedures_callreturn",
        "message0": "%1",
        "args0": [{"type": "field_label", "name": "NAME"}],
        "output": None,
        "style": "procedure_blocks",
    },
    {
        "type": "procedures_ifreturn",
        "message0": "%{BKY_CONTROLS_IF_MSG_IF} %1 %{BKY_PROCEDURES_DEFRETURN_RETURN} %2",
        "args0": [
            {"type": "input_value", "name": "CONDITION", "check": "Boolean"},
            {"type": "input_value", "name": "VALUE"},
        ],
        "previousStatement": None,
        "style": "procedure_blocks",
    },
]


def load_block_definitions() -> list[dict]:
    definitions: list[dict] = []
    for name in BLOCK_FILES:
        text = (SOURCES / f"blocks_{name}.ts").read_text(encoding="utf-8")
        marker = text.find("createBlockDefinitionsFromJsonArray([")
        if marker < 0:
            # procedures.ts defines its blocks imperatively; see above.
            continue
        for chunk in js_objects(text, marker):
            try:
                definitions.append(json.loads(js_to_json(chunk)))
            except json.JSONDecodeError:
                # Definitions carrying inline functions are not JSON; those
                # blocks are mutators we do not render anyway.
                continue
    return definitions + PROCEDURE_BLOCKS


def load_messages_json(path: Path) -> dict[str, str]:
    data = json.loads(path.read_text(encoding="utf-8"))
    return {k: v for k, v in data.items() if isinstance(v, str)}


def unescape(value: str) -> str:
    """Resolve JS string escapes without touching the UTF-8 around them.

    Decoding the whole string with `unicode_escape` would reinterpret every
    multi-byte character as latin-1 — the same mistake that made non-Latin
    Scratch locales hash as mojibake. Only the escapes are resolved here.
    """
    value = re.sub(r"\\u([0-9a-fA-F]{4})", lambda m: chr(int(m.group(1), 16)), value)
    return value.replace('\\"', '"').replace("\\'", "'").replace("\\\\", "\\")


def load_messages_classic(path: Path) -> dict[str, str]:
    """Read the pre-2019 message file: Blockly.Msg.KEY = "text";"""
    text = path.read_text(encoding="utf-8")
    found: dict[str, str] = {}
    for match in re.finditer(r'Blockly\.Msg\.([A-Z0-9_]+)\s*=\s*"((?:[^"\\]|\\.)*)"', text):
        found[match.group(1)] = unescape(match.group(2))
    return found


def resolve(message: str, messages: dict[str, str]) -> str | None:
    """Replace %{BKY_KEY} references with the localized string."""
    missing: list[str] = []

    def replace(match: re.Match[str]) -> str:
        key = match.group(1)
        if key in messages:
            return messages[key]
        missing.append(key)
        return ""

    resolved = re.sub(r"%\{BKY_([A-Z0-9_]+)\}", replace, message)
    return None if missing else resolved


def shape_of(definition: dict) -> str:
    has_output = "output" in definition
    has_previous = "previousStatement" in definition
    has_next = "nextStatement" in definition
    has_statement_input = any(
        arg.get("type") == "input_statement"
        for key, args in definition.items()
        if key.startswith("args")
        for arg in args
    )

    if has_output:
        check = definition.get("output")
        return "boolean" if check == "Boolean" else "reporter"
    if has_statement_input:
        return "c-block"
    if has_previous and not has_next:
        return "cap"
    if has_next and not has_previous:
        return "hat"
    return "stack"


SLOT_KINDS = {
    "input_value": "value",
    "input_statement": "statement",
    "field_dropdown": "dropdown",
    "field_variable": "dropdown",
    "field_number": "field",
    "field_input": "field",
    "field_angle": "field",
    "field_checkbox": "field",
    "field_colour": "field",
    "field_label": "field",
    "field_image": "field",
}


def slots_of(definition: dict) -> list[str]:
    """What each %n placeholder is, across message0, message1, …

    The renderer needs this to tell a socket (a value block plugs in) from
    a field (a white box the block owns): both are written `(…)` in the text
    notation, and Blockly draws them very differently.
    """
    kinds: list[str] = []
    index = 0
    while f"args{index}" in definition:
        for arg in definition[f"args{index}"]:
            kind = SLOT_KINDS.get(arg.get("type", ""))
            if kind:
                kinds.append(kind)
        index += 1
    return kinds


def block_specs(definitions: list[dict], messages: dict[str, str]) -> tuple[dict, list[str]]:
    specs: dict[str, dict] = {}
    gaps: list[str] = []

    for definition in definitions:
        block_type = definition.get("type")
        if not block_type or block_type in SKIP_BLOCKS:
            continue

        header = definition.get("message0")
        if not isinstance(header, str):
            continue
        text = resolve(header, messages)
        if text is None:
            gaps.append(block_type)
            continue
        # A spec made of placeholders alone — "%1", "%1 %2 %3" — would match
        # any bare input, so `(7)` on its own would become a procedure call.
        # Such blocks are named by the author (a procedure's name, an
        # operator dropdown) and are written as free text instead.
        if re.fullmatch(r"(\s*%\d+\s*)+", text):
            continue

        entry = {
            "text": clean(text),
            "shape": shape_of(definition),
            "category": STYLE_TO_CATEGORY.get(definition.get("style", ""), "sonstige"),
            "slots": slots_of(definition),
        }
        if "inputsInline" in definition:
            entry["inline"] = bool(definition["inputsInline"])

        # A C-block carries a second row ("mache %1") that labels its mouth.
        mouth = definition.get("message1")
        if isinstance(mouth, str) and entry["shape"] == "c-block":
            label = resolve(mouth, messages)
            if label:
                label = label.replace("%1", "").strip()
                if label:
                    entry["mouth"] = label

        specs[block_type] = entry

    return specs, gaps


# --------------------------------------------------------------------------
# France-IOI world blocks
# --------------------------------------------------------------------------


def world_labels(text: str, language: str) -> dict[str, str]:
    """Collect the label table of one language out of a pemFioi library.

    A library carries several sections per language — one per sub-context —
    and the label table does not always sit at the top of one. Both nest, so
    the brace matching has to be real rather than a regex guess.
    """
    labels: dict[str, str] = {}
    for marker in re.finditer(rf"\b{language}\s*:\s*\{{", text):
        section = balanced(text, text.index("{", marker.start()))
        label_marker = re.search(r"\blabel\s*:\s*\{", section)
        if not label_marker:
            continue
        table = balanced(section, section.index("{", label_marker.start()))
        for match in re.finditer(r'(\w+)\s*:\s*"((?:[^"\\]|\\.)*)"', table):
            labels.setdefault(match.group(1), clean(unescape(match.group(2))))
    return labels


def clean(value: str) -> str:
    """Drop zero-width characters that survive in some source strings."""
    return re.sub(r"[​‌‍﻿]", "", value).strip()


def world_slots(entry: str) -> list[str]:
    """Slot kinds of a France-IOI block entry.

    A `blocklyJson` carries Blockly arg types; a bare `params: [null, …]`
    means one value socket per parameter; no parameters, no slots.
    """
    kinds = [SLOT_KINDS[t] for t in re.findall(r'"type"\s*:\s*"(\w+)"', entry) if t in SLOT_KINDS]
    if kinds:
        return kinds
    params = re.search(r"params\s*:\s*\[([^\]]*)\]", entry)
    if params:
        return ["value"] * (params.group(1).count("null") + params.group(1).count("{"))
    return []


def load_world_blocks(path: Path, context: str) -> tuple[dict, list[str]]:
    """Read block metadata and German labels out of a pemFioi library.

    The libraries keep two things apart: a per-block entry with category,
    type and a `yieldsValue` flag, and a per-language table of labels keyed
    by block name. Both are needed, so both are read and joined here.
    """
    text = path.read_text(encoding="utf-8")

    labels = world_labels(text, "de")
    english = world_labels(text, "en")

    # Each block entry ends in a `block: { name: … }` object; the category
    # sits a few lines above it. Walking back from the block object is more
    # robust than trying to match a whole entry, because the entries nest
    # per-language string tables several levels deep.
    blocks: dict[str, dict] = {}
    gaps: list[str] = []
    for match in re.finditer(r"\bblock:\s*\{([^{}]*)\}", text, re.S):
        body = match.group(1)
        name_match = re.search(r'name:\s*"(\w+)"', body)
        if not name_match:
            continue
        name = name_match.group(1)

        preceding = text[max(0, match.start() - 1500) : match.start()]
        category_match = None
        for category_match in re.finditer(r'category:\s*"(\w+)"', preceding):
            pass
        category = category_match.group(1) if category_match else context

        if name not in labels:
            gaps.append(name)
            continue

        blocks[name] = {
            "text": labels[name],
            "shape": "reporter" if "yieldsValue" in body else "stack",
            "category": BEBRAS_CATEGORY.get(category, category),
            "untranslated": english.get(name) == labels[name],
            "slots": world_slots(body),
        }

    # The turtle library uses a second, terser form: a `customBlocks` table
    # that groups plain `{name: …}` entries under their category. Both forms
    # occur, sometimes in the same file, so both are read.
    start = text.find("customBlocks")
    if start >= 0:
        region = text[start:]
        category = context
        for token in re.finditer(
            r'(?P<cat>\w+)\s*:\s*\[|name\s*:\s*"(?P<name>\w+)"(?P<rest>[^\n]*)', region
        ):
            if token.group("cat"):
                category = token.group("cat")
                continue
            name = token.group("name")
            if name in blocks:
                continue
            if name not in labels:
                if name not in gaps:
                    gaps.append(name)
                continue
            # The entry may run over several lines when it carries a
            # blocklyJson; take everything up to the next `{ name:`.
            tail = region[token.end(): token.end() + 900]
            entry = token.group("rest") + tail.split("{ name:")[0].split("{name:")[0]
            blocks[name] = {
                "text": labels[name],
                "shape": "reporter" if "yieldsValue" in token.group("rest") else "stack",
                "category": BEBRAS_CATEGORY.get(category, category),
                "untranslated": english.get(name) == labels[name],
                "slots": world_slots(entry),
            }

    return blocks, gaps


# --------------------------------------------------------------------------
# Writing
# --------------------------------------------------------------------------


def escape(value: str) -> str:
    return value.replace("\\", "\\\\").replace('"', '\\"')


def write_locale(
    path: Path,
    title: str,
    license_note: str,
    specs: dict,
    inherits: str | None = None,
    markers: dict | None = None,
    aliases: dict | None = None,
) -> None:
    lines = [
        f"# {title}",
        "# Generated by scripts/blockly-data/generate.py — do not edit by hand.",
        f"# {license_note}",
        "",
    ]
    if inherits:
        lines += [
            "# Blocks not listed here are looked up in the base locale.",
            f'inherits = "{inherits}"',
            "",
        ]
    lines.append("[specs]")
    # The parser reads the end-of-block and else keywords from the specs
    # table, as the Scratch locales do.
    for key, value in (markers or {}).items():
        lines.append(f'"{key}" = "{escape(value)}"' if ":" in key else f'{key} = "{escape(value)}"')
    for block_id in sorted(specs):
        lines.append(f'{block_id} = "{escape(specs[block_id]["text"])}"')

    # The keywords are pseudo-blocks to the parser, exactly as blocks.toml
    # declares them for Scratch: `celse` opens the else branch, `cend` closes
    # the block. Without a shape they would parse as ordinary grey blocks.
    marker_shapes = {"control_else": ("celse", "logik"), "scratchblocks:end": ("cend", "logik")}

    lines += ["", "[shapes]"]
    for key in (markers or {}):
        if key in marker_shapes:
            lines.append(f'"{key}" = "{marker_shapes[key][0]}"' if ":" in key else f'{key} = "{marker_shapes[key][0]}"')
    for block_id in sorted(specs):
        lines.append(f'{block_id} = "{specs[block_id]["shape"]}"')

    lines += ["", "[categories]"]
    for key in (markers or {}):
        if key in marker_shapes:
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
        lines += ["", "# Blocks that declare inputsInline explicitly.", "[inline]"]
        for block_id in sorted(inline):
            lines.append(f'{block_id} = {str(inline[block_id]).lower()}')

    mouths = {k: v["mouth"] for k, v in specs.items() if "mouth" in v}
    if mouths:
        lines += ["", "# Label drawn next to the mouth of a C-block.", "[mouths]"]
        for block_id in sorted(mouths):
            lines.append(f'{block_id} = "{escape(mouths[block_id])}"')

    if aliases:
        lines += ["", "[aliases]"]
        for alias, block_id in aliases.items():
            lines.append(f'"{escape(alias)}" = "{block_id}"')

    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


PROFILES = {
    "blockly-modern": {
        "title": "Blockly, current look (thrasos). The default.",
        "note": (
            "thrasos defines no constants of its own: it uses Blockly's base\n"
            "# ConstantProvider unchanged and differs from geras only in its measure\n"
            "# pass. It also has no drawer of its own, which is why there is no bevel."
        ),
        "geometry": {
            "notch_width": 15,
            "notch_height": 4,
            "notch_offset": 15,
            "tab_width": 8,
            "tab_height": 15,
            "tab_offset_from_top": 5,
            "corner_radius": 8,
            "row_height": 24,
            "statement_indent": 20,
            "field_height": 16,
            "field_radius": 4,
            "field_padding_x": 5,
            "field_padding_y": 3,
            "padding_small": 3,
            "padding_medium": 5,
            "padding_large": 10,
            "hat_width": 100,
            "hat_height": 15,
            "bevel": False,
        },
        "colors": {
            "logik": "#5b80a5",
            "schleifen": "#5ba55b",
            "mathe": "#5b67a5",
            "text": "#5ba58c",
            "listen": "#745ba5",
            "farben": "#a5745b",
            "variablen": "#a55b80",
            "funktionen": "#995ba5",
            "sonstige": "#8c8c8c",
        },
    },
    "blockly-klassisch": {
        "title": "Blockly as it looked before 2019 — the generation jwinf loads.",
        "note": (
            "Values read from the blockly_compressed.js bundled with\n"
            "# bebras-modules, which is what jwinf.de actually renders with."
        ),
        "geometry": {
            "notch_width": 30,
            "notch_height": 4,
            "notch_path": "l 6,4 3,0 6,-4",
            "tab_width": 8,
            "tab_height": 20,
            "corner_radius": 8,
            "row_height": 25,
            "statement_indent": 20,
            "field_height": 16,
            "field_radius": 4,
            "sep_space_x": 10,
            "sep_space_y": 10,
            "hat_width": 100,
            "hat_height": 15,
            "bevel": True,
        },
        "colors": {},
        "inherits": "blockly-modern",
    },
    "jwinf": {
        "title": "Jugendwettbewerb Informatik (jwinf.de), training tasks.",
        "note": (
            "Colours read from the DOM of the running editors, not estimated.\n"
            "# They are set per task: the robot training tasks (Befehlsfolgen,\n"
            "# Bedingte Anweisungen, Variablen und Logik, ...) use the scheme\n"
            "# below; the Freie Turtle-Umgebung colours schleifen #47cccc,\n"
            "# logik #73cc47 and mathe #475dcc instead. This profile follows the\n"
            "# training tasks."
        ),
        "geometry": {},
        "colors": {
            "start": "#4789cc",
            "aktionen": "#723ca5",
            "schildkroete": "#7347cc",
            "sensoren": "#2b7ca5",
            "ausgeben": "#479fcc",
            "einlesen": "#7347cc",
            "schleifen": "#2fb5bd",
            "logik": "#81b31d",
            "mathe": "#3950a5",
            "farben": "#cc7347",
            "variablen": "#a5416b",
            "funktionen": "#9911a5",
            "text": "#6638a5",
            "listen": "#d8892b",
        },
        "inherits": "blockly-klassisch",
    },
}


def write_profile(path: Path, name: str, profile: dict) -> None:
    lines = [
        f"# {profile['title']}",
        "# Generated by scripts/blockly-data/generate.py — do not edit by hand.",
        f"# {profile['note']}",
        "",
        f'name = "{name}"',
    ]
    if "inherits" in profile:
        lines.append(f'inherits = "{profile["inherits"]}"')

    if profile["geometry"]:
        lines += ["", "[geometry]"]
        for key, value in profile["geometry"].items():
            if isinstance(value, bool):
                lines.append(f"{key} = {str(value).lower()}")
            elif isinstance(value, str):
                lines.append(f'{key} = "{value}"')
            else:
                lines.append(f"{key} = {value}")

    if profile["colors"]:
        lines += ["", "[colors]"]
        for key, value in profile["colors"].items():
            lines.append(f'{key} = "{value}"')

    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def write_report(path: Path, sections: list[str]) -> None:
    path.write_text("\n".join(sections) + "\n", encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--offline", action="store_true", help="use the cached sources only")
    args = parser.parse_args()

    print("Sources")
    fetch(args.offline)

    (OUT / "locales").mkdir(parents=True, exist_ok=True)
    (OUT / "profiles").mkdir(parents=True, exist_ok=True)

    print("Blockly standard blocks")
    definitions = load_block_definitions()
    modern_messages = with_synonyms(load_messages_json(SOURCES / "msg_de.json"))
    classic_messages = with_synonyms(load_messages_classic(SOURCES / "msg_de_classic.js"))

    modern, modern_gaps = block_specs(definitions, modern_messages)
    classic, classic_gaps = block_specs(definitions, classic_messages)
    print(f"  {len(definitions)} definitions, {len(modern)} with German text")

    # Blockly's operator blocks read "%1 %2 %3" with a dropdown in the
    # middle, which nobody can type, and the parser aligns a matched spec
    # with what was typed placeholder by placeholder — so an alias with two
    # placeholders for a spec with three loses its inputs. Each operator
    # therefore becomes a block of its own with a two-placeholder spec; the
    # renderer draws the operator label between two values as the dropdown
    # Blockly shows. Alternative spellings are aliases of those.
    operators = {
        "logic_compare": ("boolean", "logik", {"eq": ("=", ["=="]), "ne": ("≠", ["!="]), "lt": ("<", []), "le": ("≤", ["<="]), "gt": (">", []), "ge": ("≥", [">="])}),
        "math_arithmetic": ("reporter", "mathe", {"add": ("+", []), "sub": ("-", ["−"]), "mul": ("×", ["*"]), "div": ("÷", ["/"]), "pow": ("^", [])}),
        "logic_operation": ("boolean", "logik", {"and": ("und", []), "or": ("oder", [])}),
    }
    # controls_whileUntil reads "%1 %2": a dropdown whose options are
    # "wiederhole solange" / "wiederhole bis", then the condition. Written
    # out, that is "wiederhole [solange v] <…>".
    modern["controls_whileUntil"] = {"text": "wiederhole %1 %2", "shape": "c-block", "category": "schleifen", "slots": ["dropdown", "value"], "mouth": modern_messages.get("CONTROLS_REPEAT_INPUT_DO", "mache")}
    classic["controls_whileUntil"] = {"text": "wiederhole %1 %2", "shape": "c-block", "category": "schleifen", "slots": ["dropdown", "value"], "mouth": classic_messages.get("CONTROLS_REPEAT_INPUT_DO", "mache")}

    # math_number_property reads "%1 %2" too, its predicate sitting in the
    # dropdown ("ist gerade", …). jwinf's editor shows it as "0 ist [gerade]",
    # which is also how a worksheet writes it: <(0) ist [gerade v]>.
    for table in (modern, classic):
        table["math_number_property"] = {"text": "%1 ist %2", "shape": "boolean", "category": "mathe", "slots": ["value", "dropdown"], "inline": True}

    operator_aliases = {}
    for base, (shape, category, table) in operators.items():
        modern.pop(base, None)
        classic.pop(base, None)
        for suffix, (symbol, spellings) in table.items():
            block_id = f"{base}_{suffix}"
            # Blockly's compare and arithmetic blocks are inline; the logic
            # operation stacks its two values on two rows, `und` on the second.
            modern[block_id] = {"text": f"%1 {symbol} %2", "shape": shape, "category": category, "slots": ["value", "value"], "inline": base != "logic_operation"}
            for spelling in spellings:
                operator_aliases[f"%1 {spelling} %2"] = block_id

    # Keywords the text syntax needs beyond the blocks themselves: how a
    # C-block is closed and how its else branch is introduced.
    else_word = modern_messages.get("CONTROLS_IF_MSG_ELSE", "sonst")
    markers = {"scratchblocks:end": "ende", "control_else": else_word}
    aliases = {"ende": "scratchblocks:end", "Ende": "scratchblocks:end", **operator_aliases}
    write_locale(
        OUT / "locales" / "blockly-de.toml",
        "Blockly standard blocks, German (current wording).",
        "Text from Blockly (Apache-2.0), msg/json/de.json.",
        modern,
        markers=markers,
        aliases=aliases,
    )

    print("jwinf world blocks")
    robot, robot_gaps = load_world_blocks(SOURCES / "robot_lib.js", "robot")
    turtle, turtle_gaps = load_world_blocks(SOURCES / "turtle_lib.js", "turtle")
    print(f"  robot {len(robot)}, turtle {len(turtle)}")

    # The jwinf locale carries the world blocks plus every standard block
    # whose old wording differs from today's — that difference is exactly
    # what a learner sees on jwinf.
    differing = {
        block_id: entry
        for block_id, entry in classic.items()
        if block_id in modern and entry["text"] != modern[block_id]["text"]
    }
    # The start blocks. Each world has one, it is a hat, and its text is what
    # the running editors show: "Roboter-Programm" on the grid tasks,
    # "Schildkröten-Programm" on the turtle tasks, "Programm" on the text
    # console. They are not in the France-IOI libraries, which build them at
    # runtime, so they are declared here from what was observed on jwinf.de.
    starts = {
        "robot_start": {"text": "Roboter-Programm", "shape": "hat", "category": "start"},
        "turtle_start": {"text": "Schildkröten-Programm", "shape": "hat", "category": "start"},
        "program_start": {"text": "Programm", "shape": "hat", "category": "start"},
    }
    world = {}
    for name, entry in {**robot, **turtle}.items():
        # Same rule as for the standard blocks: a label that is nothing but
        # placeholders would match any bare input.
        if re.fullmatch(r"(\s*%\d+\s*)+", entry["text"]):
            continue
        # France-IOI appends a block's parameters after a label that has no
        # placeholder of its own — "setze Farbe" with one parameter shows as
        # "setze Farbe ?" — so the spec has to say so.
        if entry["slots"] and "%" not in entry["text"]:
            entry = dict(entry, text=entry["text"] + "".join(f" %{i + 1}" for i in range(len(entry["slots"]))))
        world[name] = entry
    # Many blocks come in two flavours that read identically — one takes a
    # value block ("moveamount"), one a typed-in field ("moveamountvalue").
    # A worksheet writes the number, so the field flavour is kept; a block
    # plugged in is drawn as a socket regardless of the flavour.
    by_text: dict[str, list[str]] = {}
    for name, entry in world.items():
        by_text.setdefault(entry["text"], []).append(name)
    twins = []
    for names in by_text.values():
        if len(names) < 2:
            continue
        fields_only = [n for n in names if "value" not in world[n]["slots"]]
        keep = fields_only[0] if fields_only else names[0]
        for n in names:
            if n != keep:
                twins.append(n)
    for name in twins:
        del world[name]
    jwinf_specs = {**world, **differing, **starts}
    write_locale(
        OUT / "locales" / "jwinf-de.toml",
        "jwinf world blocks, plus standard blocks whose wording differs.",
        "World blocks from France-IOI bebras-modules (MIT); standard block text from Blockly (Apache-2.0).",
        jwinf_specs,
        inherits="blockly-de",
        markers={"scratchblocks:end": "ende", "control_else": classic_messages.get("CONTROLS_IF_MSG_ELSE", "sonst")},
        aliases=aliases,
    )

    print("Profiles")
    for name, profile in PROFILES.items():
        write_profile(OUT / "profiles" / f"{name}.toml", name, profile)
        print(f"  {name}")

    report = [
        "# Generated block data — what came out",
        "",
        "Regenerate with `python3 scripts/blockly-data/generate.py`.",
        "",
        "## Counts",
        "",
        "| Set | Blocks |",
        "| --- | ---: |",
        f"| Blockly standard (German) | {len(modern)} |",
        f"| jwinf robot world | {len(robot)} |",
        f"| jwinf turtle world | {len(turtle)} |",
        f"| standard blocks worded differently in the old Blockly | {len(differing)} |",
        f"| world blocks dropped as socket-twins of a field variant | {len(twins)} |",
        "",
        "## Shapes",
        "",
        "| Shape | Blockly | jwinf worlds |",
        "| --- | ---: | ---: |",
    ]
    shapes = sorted({e["shape"] for e in list(modern.values()) + list(robot.values()) + list(turtle.values())})
    world = {**robot, **turtle}
    for shape in shapes:
        report.append(
            f"| {shape} | {sum(1 for e in modern.values() if e['shape'] == shape)} "
            f"| {sum(1 for e in world.values() if e['shape'] == shape)} |"
        )

    untranslated = sorted(k for k, v in {**robot, **turtle}.items() if v.get("untranslated"))

    report += ["", "## Gaps", ""]
    if modern_gaps:
        report.append(f"- {len(modern_gaps)} standard blocks without a German string: {', '.join(sorted(modern_gaps))}")
    if robot_gaps or turtle_gaps:
        missing = sorted(set(robot_gaps) | set(turtle_gaps))
        report.append(
            f"- {len(missing)} world blocks without a German label — these need translating by hand: "
            + ", ".join(missing)
        )
    if untranslated:
        report.append(
            f"- {len(untranslated)} world blocks whose German and English labels are identical, "
            "so one of the two upstream tables is wrong — checked by hand, this goes both ways "
            "(`row` is English in the German table, `turnleftamountvalue_options` is German in "
            "the English one): " + ", ".join(untranslated)
        )
    if not (modern_gaps or robot_gaps or turtle_gaps or untranslated):
        report.append("- none")

    report += [
        "",
        "## Wording that differs between old and current Blockly",
        "",
        "The old wording is what a learner sees on jwinf, so the jwinf locale keeps it.",
        "",
        "| Block | jwinf (old) | current |",
        "| --- | --- | --- |",
    ]
    for block_id in sorted(differing):
        report.append(f"| `{block_id}` | {differing[block_id]['text']} | {modern[block_id]['text']} |")

    write_report(OUT / "REPORT.md", report)
    print(f"\nWritten to {OUT.relative_to(Path.cwd()) if OUT.is_relative_to(Path.cwd()) else OUT}")
    print("See generated/REPORT.md for counts and gaps.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
