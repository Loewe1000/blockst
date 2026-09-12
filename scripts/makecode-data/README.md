# Block data for the MakeCode dialect

Generates the block texts, shapes, categories and profiles that the
Microsoft MakeCode side of blockst renders (micro:bit and Calliope mini) —
the counterpart to `scripts/blockly-data/` for Blockly and
`data/blocks.toml`/`data/locales/*.toml` for Scratch.

```sh
python3 scripts/makecode-data/generate.py           # download sources, then generate
python3 scripts/makecode-data/generate.py --offline # generate from the cache alone
```

Sources land in `sources/` (ignored by git, except `sources/live/`) and are
reused. Output goes to
`scripts/scratchblocks-wasm/data/dialects/`, where the plugin embeds locale
and profile TOML at compile time; it **is** committed, so the data can be
reviewed in a diff without anyone having to run the script.

## Where the data comes from

| Source | License | Used for |
| --- | --- | --- |
| [microsoft/pxt-microbit](https://github.com/microsoft/pxt-microbit) `libs/core/*.ts`, `libs/core/shims.d.ts`, `libs/radio/*` | MIT | micro:bit's own blocks (basic, input, led, control, pins, serial, loops, game, radio) and their English wording |
| [microsoft/pxt-calliope](https://github.com/microsoft/pxt-calliope) `libs/core-mini-codal/*` | MIT | Calliope mini's extra blocks (motors, the RGB LED) |
| [microsoft/pxt](https://github.com/microsoft/pxt) `pxtlib/blocks.ts` | MIT | the loop/logic/math/variables/array/text/function blocks every MakeCode target shares — pxt itself builds these, not any target, so they are transcribed by hand into `BUILTIN_BLOCKS` the same way `scripts/blockly-data/generate.py` transcribes Blockly's five procedure blocks (the source is imperative TypeScript, not JSON) |
| both targets' own `pxtarget.json` `appTheme.blockColors` plus each namespace's `//% color=...` annotation | MIT | the profile colour tables |

Commits are pinned in `generate.py` (`MICROBIT_SHA`, `CALLIOPE_SHA`); bump
them by hand to pick up upstream changes.

## Where the translations come from

MakeCode publishes no translated string files: only the English source
strings live in the repos (`_locales/*-strings.json`), nothing is in the npm
packages, and Crowdin has no keyless export. The running editor fetches its
translations from `cdn.makecode.com/api/translations?lang=<lang>&filename=<file>&approved=true`
— pxt's `downloadLiveTranslationsAsync` — and that is what the generator
does too, for every language in the targets' `appTheme.availableLocales`
(36, the same list for both) and for the files pxt merges: `strings.json`
(the pxt builtins: loops, logic, variables, …), `<target>/target-strings.json`
and `<target>/<lib>-strings.json` per bundled lib (`microbit`, `calliopemini`).
Only approved strings come back; a block without one keeps its English
text, exactly as the editor shows it.

The raw downloads go to `sources/translations-raw/` (ignored, ~11 MB);
the strings the catalog actually uses are kept in
`sources/translations/<target>/<lang>.json` (committed, ~600 KB), so
`--offline` rebuilds every locale without the network.

Keys are matched to catalog blocks through the German text: a block's
`api|block` key where the live crawl gives one, else the German value
(whitespace- and hyphen-insensitive, trailing placeholders ignored), else
the English text as key. pxt builds a few blocks from single words —
`{id:logic}if`/`{id:logic}then`, `{id:op}and`, `join` — listed in
`COMPOSED_KEYS`; the operator blocks are the same glyph in every language
(`NEUTRAL_PREFIXES`). Mouth labels resolve via `{id:repeat}do` and friends;
`{id:empty}` (Japanese) means the editor draws no label. Blocks whose key
could not be found are listed in the report and stay English.

German and English themselves are read off the running editors (below),
which is also what tells the generator the slot kinds and colours; the
translations only replace the text.

## Output

Relative to `scripts/scratchblocks-wasm/data/dialects/`:

- `locales/makecode-en.toml`, `locales/makecode-de.toml` — micro:bit's
  blocks, each a complete locale (German does not silently fall back to
  English at the engine level; an untranslated string is copied into the
  German file as-is and listed in the report instead)
- `locales/makecode-<lang>.toml` for the 34 other editor languages — texts,
  markers and mouth labels only; shapes, categories and slots come from
  `makecode-en`, which they `inherit`
- `locales/makecode-calliope-<lang>.toml` — Calliope mini's extra/overridden
  blocks only; each `inherits` the matching micro:bit locale for everything
  else
- `../../src/generated/makecode_locales.rs` — the `include_str!` list the
  engine embeds
- `profiles/makecode.toml`, `profiles/makecode-calliope.toml` — palette per
  target; the Calliope profile `inherits` the micro:bit one and overrides
  only the categories its own `pxtarget.json` gives a different colour
- `REPORT.md` — counts, categories, shapes, the German-coverage gap list
  and colour table, appended under a "MakeCode" heading (re-running the
  generator replaces that section rather than growing the file)

## What was left out

Scope was kept close in size to the Blockly generator's ~44 standard blocks
rather than exhaustively covering MakeCode's block reference (which spans
hundreds of blocks across dozens of extensions). Deliberately not fetched
or parsed:

- the melody/sound-effect editor blocks (`music.ts`, `melodies.ts`,
  `soundexpressions.ts`, `playable.ts`) — large, and mostly rich custom
  field editors (piano roll, waveform picker) that do not reduce to a
  `%n`-placeholder text/slot pair the way the rest of the catalog does;
- sprite/`LedSprite` methods in `game.ts` — these are TypeScript class
  methods (`public move(...)`), not `//%`-annotated top-level functions,
  and the generator's extractor only recognises the latter (a deliberate
  scope line, not a bug: a sprite method's first parameter is the sprite
  instance itself, which needs different modelling than an ordinary slot);
- `libs/core/pinscompat.ts` (legacy pin-compatibility aliases) and
  `libs/core/icons.ts`'s `IconNames`/`UnitConversion` enum members (each
  carries a `//% block="..."` of its own, but as a dropdown *option* label,
  not a block) — `basic.showIcon` itself is still pulled from `icons.ts`;
- any block whose function name starts with `_` (pxt's own convention for
  an internal, shadow-only helper never shown in a toolbox);
- `deprecated=1`/`blockHidden=1`/`hidden=1` blocks.

See `REPORT.md` for the running list of what *was* pulled in but has no
curated German text yet.

## The running editors as the source of the block texts

The texts the editors actually show in German and in English — with the
slot kinds, mouth labels and colour of every toolbox block, which no string
file carries — are read off the running editors:

- `sources/live/crawl.js`: paste into the browser console of
  makecode.microbit.org or makecode.calliope.cc (opened with
  `?lang=de#editor` / `?lang=en#editor`); it walks every toolbox category
  through Blockly's API and prints one JSON line per block.
- `sources/live/{microbit,calliope}-{de,en}.jsonl`: that output, committed
  (crawled 2026-09-12: micro:bit target 9.0.12 on pxt 13.0.9, Calliope
  8.1.15 on pxt 13.1.8).
- `sources/live/extract.py`: turns a saved console dump into the JSONL.

`generate.py` lays these records over the catalog it extracts from the
`//%` annotations in the sources (`apply_live`); a live record wins. Blocks
the toolbox hides keep the source-derived text.
