# SB3 Import Plugin for Typst

This folder contains a Typst plugin (WASM) that converts Scratch `.sb3` data
into Blockst parser text.

It is built for the direct Typst workflow:
- read `.sb3` bytes inside Typst,
- pass bytes to Blockst's `scratch.sb3` helpers,
- get renderable script text back,
- render with `scratch.text.en`.

If a project contains multiple top-level scripts, output includes numbered
headers by target, e.g. `// [2] Sprite: Cat - Script 1`.

Empty scripts are skipped automatically and are not selectable by number.

## Build the plugin

Requirements:
- Rust toolchain
- `wasm32-unknown-unknown` target

From project root:

```bash
rustup target add wasm32-unknown-unknown
cd scripts/sb3-wasm
cargo build --target wasm32-unknown-unknown --release
cd ../..
cp scripts/sb3-wasm/target/wasm32-unknown-unknown/release/sb3_wasm.wasm libs/scratch/plugins/sb3_wasm.wasm
```

Plugin output:

```text
scripts/sb3-wasm/target/wasm32-unknown-unknown/release/sb3_wasm.wasm
```

## Use directly in Typst

```typst
#import "@preview/blockst:0.2.0": blockst, scratch

#let sb3-plugin = "scripts/sb3-wasm/target/wasm32-unknown-unknown/release/sb3_wasm.wasm"
#let sb3-bytes = read("project.sb3", encoding: none)
#let sb3-text = str(plugin(sb3-plugin).sb3_to_scratch_text(sb3-bytes))

#blockst[
	#import scratch.text.en: *
	#render-scratch-text(sb3-text)
]
```

Or via convenience wrappers from Blockst:

```typst
#import "@preview/blockst:0.2.0": blockst, scratch

#let sb3-bytes = read("project.sb3", encoding: none)

#blockst[
	#import scratch.sb3: render-sb3-scripts
	#render-sb3-scripts(sb3-bytes)
]
```

Render only one script by global number:

```typst
#render-sb3-scripts(sb3-bytes, script-number: 2)
```

Render only scripts from one target:

```typst
// only stage scripts
#render-sb3-scripts(sb3-bytes, target: "stage")

// all scripts from one sprite (exact target name)
#render-sb3-scripts(sb3-bytes, target: "Player")
```

Note: `target` and `script-number` are mutually exclusive.

Render lists and variables from the state catalog:

```typst
#import scratch.sb3: render-sb3-lists, render-sb3-variables

#render-sb3-lists(sb3-bytes, target: "stage")
#render-sb3-variables(sb3-bytes, target: "stage")

// select one list/variable by name within target (recommended)
#render-sb3-lists(sb3-bytes, target: "stage", target-list-name: "Players")
#render-sb3-variables(sb3-bytes, target: "stage", target-variable-name: "Score")

// alternatively by local number within target
#render-sb3-lists(sb3-bytes, target: "stage", target-list-number: 1)
#render-sb3-variables(sb3-bytes, target: "stage", target-variable-number: 1)
```

Inspect script numbering metadata:

```typst
#import scratch.sb3: sb3-scripts-catalog
#let catalog = sb3-scripts-catalog(sb3-bytes)

// parsed_text is included by default
#let first-text = catalog.at(0).scripts.at(0).parsed_text

// metadata only for one target
#let cat-catalog = sb3-scripts-catalog(sb3-bytes, target: "Cat")

// optional: skip parsed_text for faster catalog lookups
#let fast-catalog = sb3-scripts-catalog(sb3-bytes, include-parser-text: false)
```

Get compact per-target state snapshots (variables, lists, stage/sprite props):

```typst
#import scratch.sb3: sb3-state-catalog

// all targets
#let state = sb3-state-catalog(sb3-bytes)

// only one target
#let cat-state = sb3-state-catalog(sb3-bytes, target: "Cat")
```

Optional override with a custom plugin module:

```typst
#let custom-plugin = plugin("my-custom-sb3-plugin.wasm")
#render-sb3-scripts(sb3-bytes, sb3-plugin: custom-plugin)
```

## Current scope

Implemented:
- extraction of `project.json` from `.sb3`
- script discovery for top-level stacks
- opcode mapping for common events, motion, looks, control, operators, and data
- `// unsupported opcode: ...` comments for missing mappings

Limitations:
- not all Scratch opcodes are mapped yet
- custom block/procedure reconstruction is incomplete
- plugin output is English parser text; `scratch.sb3.render-sb3-scripts(...)` can still render it in `en`, `de`, or `fr`

## Coverage check

You can check opcode coverage for any `.sb3` file with:

```bash
./scripts/sb3-wasm/check-sb3-import-coverage.sh examples/Listen.sb3
```

Use strict mode for CI:

```bash
./scripts/sb3-wasm/check-sb3-import-coverage.sh examples/Listen.sb3 --strict
```

This reports:
- total/unique opcodes in the project
- unsupported opcodes (unique)
- per-opcode occurrence counts
