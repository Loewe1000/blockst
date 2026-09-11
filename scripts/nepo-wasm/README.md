# NEPO plugin for Blockst (prototype)

Renders Open Roberta (NEPO) blocks as SVG, for the Calliope mini and micro:bit
block sets. Separate from `scratchblocks-wasm`: the two dialects disagree about
notches, corner radii and what a block is even made of, so they share a Typst
front end and nothing below it.

Status: static renderer with a growing core set from the Calliope mini and
micro:bit beginner toolboxes, plus the variable and list blocks that go with
them. No execution or simulation. Mutator icons are drawn where Open Roberta
draws them, and the two mutations that change what a program *says* — the
Start block's variable declarations and a list's items — are rendered; the
remaining mutator branches (a second `wenn` condition, `sonst wenn`) are not.

## Blocks

| Group | Catalog ids | German surface syntax |
|---|---|---|
| Start | `robControls_start` | `Start`, with the declaration mouth its plus mutator opens |
| Display & sound | `mbedActions_display_text`, `mbedActions_display_image`, `mbedActions_display_clear`, `mbedActions_play_note` | `Zeige Text …`, `Zeige Bild …`, `Lösche Bildschirm`, `Spiele Viertelnote C4` |
| Calliope RGB LED | `actions_rgbLed_hidden_on_calliope`, `actions_rgbLed_hidden_off_calliope` | `Schalte RGB LED an Farbe …`, `Schalte RGB LED aus` |
| Sensor | `robSensors_key_getSample`, `robSensors_pintouch_getSample`, `robSensors_gesture_getSample`, `robSensors_compass_getSample`, `robSensors_sound_getSample`, `robSensors_timer_getSample`, `robSensors_temperature_getSample`, `robSensors_light_getSample`, `mbedSensors_timer_reset` | `Taste A gedrückt?`, `Pin 0 gedrückt?`, `gib …`, `Setze Zeitgeber 1 zurück` |
| Control | `robControls_loopForever`, `controls_repeat_ext`, `robControls_if`, `robControls_ifElse`, `robControls_wait_time`, `robControls_wait_for` | `Wiederhole … mal`, `wenn …`, `Warte ms …`, `Warte bis …` |
| Logic & math | `logic_boolean`, `logic_compare`, `logic_operation`, `math_number`, `math_arithmetic`, `math_random_int` | `wahr`, `1 + 2`, `Taste A gedrückt? und wahr`, `1 ≤ 2`, `ganzzahliger Zufallswert zwischen 1 bis 10` |
| Text, colour & image reporters | `text`, `text_comment`, `mbedColour_picker`, `mbedImage_image`, `mbedImage_get_image` | quoted text, `Kommentar "Notiz"`, colours, editable 5×5 LED matrices, `Herz`/`Lächeln`/… |
| Variables | `robGlobalVariables_declare`, `variables_set`, `variables_get`, `robMath_change` | `Variable Punkte : Zahl ← 0`, `Schreibe Punkte 0`, `Punkte`, `erhöhe Punkte um 1` |
| Lists | `robLists_create_with`, `robLists_repeat`, `robLists_length`, `robLists_isEmpty`, `robLists_indexOf`, `robLists_getIndex`, `robLists_setIndex` | `Liste : Zahl ← 1 2 3`, `Länge von Werte`, `von der Liste Werte nimm #tes 2` |

The parser continues to accept the prototype's shorter RGB-LED wording
(`Schalte RGB LED an …`) and renders Open Roberta's canonical `Farbe` label.

## Variables

Open Roberta has no variable category to drag from: variables are *declared*,
in the Start block, by its plus mutator, and only then do the getter and the
setter appear in the toolbox. The renderer follows that shape.

```text
Start
  Variable Punkte : Zahl ← 0          <- declaration, inside the Start block
  Variable Name : Zeichenkette ← "Ida"
  Schreibe Punkte 10                  <- the program itself follows below
  erhöhe Punkte um 1
  Zeige Text Punkte
```

Declarations indented under `Start` go into its mouth; everything else stays
in the stack below it, because Blockly gives that mouth the connection check
`declarationGlobal` and the renderer applies the same rule. `←` may be written
as `=` or `<-`.

The declared type is not decoration. It types the declaration's own socket,
the setter's socket and the getter's output plug, and each of those is drawn
in the colour of the type — which is exactly the hint a worksheet with an
empty socket is trying to give. `src/variables.rs` does that pass, standing in
for the `onchange` handlers a live Blockly workspace runs. The same pass
carries a list's element type through `robLists_getIndex` and
`robLists_setIndex`.

Ten types are offered, matching `Blockly.TYPE_DROPDOWN('calliope')`: `Zahl`,
`logischer Wert`, `Zeichenkette`, `Farbe`, `Bild` and the five `Liste …`
variants.

Not covered: local declarations (`robLocalVariables_declare`, which belong to
procedures and counted loops), the zero-item form of `robLists_create_with`
("Erzeuge eine leere Liste"), and `robLists_getSublist`. A list may hold at
most six items, where Open Roberta's plus mutator has no limit.

## Where the numbers come from

Everything visual is transcribed from the official sources rather than matched
by eye, because "the colours and shapes are exactly right" is what the
prototype had to demonstrate:

| What | Source |
|---|---|
| Category colours, data-type colours | `core/constants.js` |
| Notches, tabs, corner radius, spacing | `core/block_render_svg.js` |
| Field box, baseline, dropdown arrow | `core/field.js`, `core/field_dropdown.js` |
| Pixel cell | `core/field_pixelbox.js` |
| Label font | `core/css.js` (`.blocklyText`, 11pt sans-serif) |
| Block structure | `blocks/mbedControls.js`, `blocks/mbedActions.js`, `blocks/mbedImage.js`, `blocks/robControls.js`, `blocks/robSensorDefinitions.js`, `blocks/variables.js`, `blocks/lists.js`, `blocks/math.js` |
| Mutator icons, icon spacing | `core/mutatorPlus.js`, `core/mutatorMinus.js`, `core/icon.js` |
| Variable types | `core/constants.js` (`Blockly.TYPE_DROPDOWN`) |
| German text | `msg/js/de.js` |

All from <https://github.com/OpenRoberta/blockly>.

## Layout

```
src/
  catalog.rs   blocks.toml + locales -> renderable blocks
  parser.rs    indentation-based source text -> blocks
  variables.rs declared types -> every use of the variable
  matrix.rs    the 5x5 LED grid, the one special case
  measure.rs   port of Blockly's renderCompute_
  render.rs    port of Blockly's renderDraw*
  svg.rs       the geometry constants, verbatim
  theme.rs     the palette, verbatim
data/
  blocks.toml          structure: shape, category, field kinds
  locales/de.toml      German text, dropdown options, aliases
  locales/en.toml      English text
```

Adding a block normally means adding one entry to `blocks.toml` and one line
per locale. Inline value fields (`inline_value`) let reporter blocks compose
within one row; the renderer uses a typed rounded socket when they are empty.
`mbedImage_get_image` is deliberately self-contained: it draws the 5×5 pattern
itself instead of depending on Open Roberta's PNG asset bundle. The field still
claims the size the PNG would have — `FieldDropdownImage` asks for 34×34 for a
24×24 picture — so the block, and the row that holds it, are as tall as the
real ones. That height is the one field size in the whole set that is not
Blockly's default 25.

## Build

```bash
rustup target add wasm32-unknown-unknown
cd scripts/nepo-wasm
cargo build --target wasm32-unknown-unknown --release --lib
cd ../..
cp scripts/nepo-wasm/target/wasm32-unknown-unknown/release/nepo_wasm.wasm \
   libs/nepo/plugins/nepo_wasm.wasm
```

## Test

```bash
cd scripts/nepo-wasm && cargo test
```

`tests/test_geometry.rs` checks the layout and the outline against
`examples/nepo/references/blockly-reference.mjs`, an independent JavaScript
transcription of the same Blockly source. Regenerate its output with
`examples/nepo/comparisons/build.sh` after changing any geometry.

## Render without Typst

```bash
cargo run --bin nepo-render -- '{"code":"Start","language":"de"}'
```
