// =====================================================
// blockst — Render block-based programming languages
// =====================================================
//
// Primary API (no wrapper needed):
//   #scratch("when green flag clicked\nmove (10) steps")
//   #scratch("Wenn ⚑ angeklickt\ngehe (30)er-Schritt", language: "de")
//
// Optional grouping with theme override:
//   #blockst(theme: "print", scale: 80%)[
//     #scratch("...")
//   ]
//
// Global settings:
//   #set-blockst(theme: "high-contrast", scale: 70%)
//
// SB3 import:
//   #sb3.render-scripts(project, language: "de")
//
// Execution:
//   #scratch-run(..scratch-execute("pen down\nrepeat (4)\nmove (70) steps\nturn cw (90) degrees\nend"))

#import "libs/scratch/options.typ": get-options, scratch-block-options

// Generic parser/renderer — supports all 26 WASM locales
#import "libs/scratch/text/parser.typ": parse-scratch-text as _generic-parse, render-scratch-text as _generic-render

#import "libs/scratch/text/execute.typ": execute-scratch-text

#import "libs/scratch/sb3.typ" as sb3

// ─── Container ────────────────────────────────────────
/// Optional container for grouped blocks with theme/scale override.
/// Only needed when a specific group should differ from global settings.
#let blockst(
  theme: auto,
  scale: auto,
  spacing: 1.5em,
  body,
) = context {
  let current-opts = get-options()
  let final-theme = if theme == auto { current-opts.at("theme", default: "normal") } else { theme }
  let final-scale = if scale == auto { current-opts.at("scale", default: 100%) } else { scale }
  scratch-block-options.update(old => {
    let new-opts = old
    new-opts.theme = final-theme
    new-opts.scale = final-scale
    new-opts
  })
  stack(spacing: spacing, body)
}

/// Global settings applied to all scratch() and sb3 calls.
#let set-blockst(theme: none, scale: none, stroke-width: none, font: none) = {
  scratch-block-options.update(old => {
    let new-opts = old
    if theme != none { new-opts.insert("theme", theme) }
    if scale != none { new-opts.insert("scale", scale) }
    if stroke-width != none { new-opts.insert("stroke-width", stroke-width) }
    if font != none { new-opts.insert("font", font) }
    new-opts
  })
}

// ─── Main API ─────────────────────────────────────────

#let _normalize-source(elements) = {
  if type(elements) == content and elements.func() == raw {
    elements = elements.text
  }
  if type(elements) == str { elements } else { str(elements) }
}

/// Render scratch blocks from text. Works standalone or inside #blockst[].
/// Supports 26 languages: en, de, fr, es, it, pt, nl, pl, ru, ja, ...
#let scratch(text, language: "en") = {
  let text = _normalize-source(text)
  _generic-render(text, lang-code: language)
}

/// Render a single scratch block (no script grouping).
#let scratch-block(text, language: "en") = scratch(text, language: language)

/// Parse scratch text to AST (for programmatic use).
#let scratch-parse(text, language: "en") = {
  let text = _normalize-source(text)
  _generic-parse(text, lang-code: language)
}

/// Execute scratch text, producing scratch-run commands.
#let scratch-execute(text, language: "en") = execute-scratch-text(_normalize-source(text), language: language)

// ─── Interpreter ──────────────────────────────────────
#import "libs/scratch/interpreter.typ": blockst-run-options, scratch-run, set-scratch-run
#import "libs/scratch/exec/de.typ" as exec-de
#import "libs/scratch/exec/en.typ" as exec-en
#import "libs/scratch/exec/fr.typ" as exec-fr

// ─── Markdown code block support ─────────────────────
/// Enable scratch code blocks in raw text:
///   #show: raw-scratch()
///   ```scratch
///   move (10) steps
///   ```
/// With language:
///   #show: raw-scratch(language: "de")
#let raw-scratch(..args) = (
  body => {
    show raw.where(block: true, lang: "scratch"): scratch.with(..args)
    body
  }
)
