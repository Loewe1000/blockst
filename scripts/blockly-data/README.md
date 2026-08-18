# Block data for the Blockly dialect

Generates the block texts, shapes, categories and profiles that the Blockly
side of blockst renders — the counterpart to `data/blocks.toml` and
`data/locales/*.toml` on the Scratch side.

```sh
python3 scripts/blockly-data/generate.py           # download sources, then generate
python3 scripts/blockly-data/generate.py --offline # generate from the cache alone
```

Sources land in `sources/` (ignored by git) and are reused. Output goes to
`generated/`, which **is** committed, so the data can be reviewed in a diff
without anyone having to run the script.

## Where the data comes from

| Source | License | Used for |
| --- | --- | --- |
| [google/blockly](https://github.com/google/blockly) `blocks/*.ts`, `msg/json/de.json` | Apache-2.0 | standard blocks and their current German wording |
| [France-ioi/bebras-modules](https://github.com/France-ioi/bebras-modules) `ext/blockly/de.js` | Apache-2.0 (bundled Blockly) | the German wording jwinf actually shows |
| [France-ioi/bebras-modules](https://github.com/France-ioi/bebras-modules) `pemFioi/blockly*_lib*.js` | MIT | robot and turtle world blocks |

Geometry values are not scraped: the modern set is Blockly's base
`ConstantProvider` (thrasos adds none of its own), the classic set was read
out of the `blockly_compressed.js` that bebras-modules bundles. Both sit in
`PROFILES` in the script, with a comment saying where each number is from.

## Output

- `generated/locales/blockly-de.toml` — standard blocks, current wording
- `generated/locales/jwinf-de.toml` — world blocks, plus the standard blocks
  whose old wording differs from today's
- `generated/profiles/*.toml` — geometry and palette per profile
- `generated/REPORT.md` — counts, gaps, and the wording differences

## Reading the report

`REPORT.md` is the point of this stage: it makes the gaps visible instead of
letting them surface halfway through the renderer work. Two kinds show up.

**Blocks without a German label.** Around fifty world blocks have no German
text upstream at all. They need translating by hand before the affected
worlds can be rendered.

**Labels identical in German and English.** These mean one of the two upstream
tables is wrong, and it goes both ways: `row` is English in the German table,
while `turnleftamountvalue_options` is German in the English one. Each needs a
look rather than an automatic rule.

Neither is a defect of this script. Both are worth reporting upstream once
someone has checked them.
