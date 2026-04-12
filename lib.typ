// =====================================================
// blockst — Container function and language modules
// =====================================================

#import "libs/scratch/mod.typ": scratch-block-options
#import "libs/scratch/rendering/colors.typ": get-colors-from-options, get-font-from-options

// Container function for block environments
// Usage: #blockst[#import scratch.de: * ...]
#let blockst(
  theme: auto,
  scale: auto,
  body,
) = context {
  // Read current options from state
  let current-opts = scratch-block-options.get()

  // Fall back to state values when auto was passed
  let final-theme = if theme == auto {
    current-opts.at("theme", default: "normal")
  } else {
    theme
  }

  let final-scale = if scale == auto {
    current-opts.at("scale", default: 100%)
  } else {
    scale
  }

  // Apply text styling once at the top level to avoid
  // per-block set-text show rules that compound nesting depth.
  let colors = get-colors-from-options(current-opts)
  let font-family = get-font-from-options(current-opts)
  set text(font: font-family, fill: colors.text-color, weight: 500)

  // Render body with scaling
  block(above: 2em, std.scale(final-scale, reflow: true, body))
}

// Global settings for block environments
// Usage: #set-blockst(theme: "print", scale: 80%, stroke-width: 1pt)
#let set-blockst(
  theme: none,
  scale: none,
  stroke-width: none,
  font: none,
) = {
  scratch-block-options.update(old => {
    let new-opts = old
    if theme != none {
      new-opts.insert("theme", theme)
    }
    if scale != none {
      new-opts.insert("scale", scale)
    }
    if stroke-width != none {
      new-opts.insert("stroke-width", stroke-width)
    }
    if font != none {
      new-opts.insert("font", font)
    }
    new-opts
  })
}

// Executable Scratch environment (interpreter, state, settings)
#import "libs/scratch/interpreter.typ": blockst-run-options, set-scratch-run, scratch-run

// Language modules as sub-namespaces
#import "libs/scratch/lang/de.typ" as de
#import "libs/scratch/lang/en.typ" as en
#import "libs/scratch/lang/fr.typ" as fr

// Experimental text parser modules
#import "libs/scratch/text/en.typ" as text-en
#import "libs/scratch/text/de.typ" as text-de
#import "libs/scratch/text/fr.typ" as text-fr

// Experimental SB3 import helpers (Typst plugin bridge)
#import "libs/scratch/sb3.typ" as sb3

// Executable block localisations
#import "libs/scratch/exec/de.typ" as exec-de
#import "libs/scratch/exec/en.typ" as exec-en
#import "libs/scratch/exec/fr.typ" as exec-fr

// Scratch namespace with language sub-modules and executable localisations
#let scratch = (
  de: de,
  en: en,
  fr: fr,
  text: (
    de: text-de,
    en: text-en,
    fr: text-fr,
  ),
  sb3: sb3,
  exec: (
    de: exec-de,
    en: exec-en,
    fr: exec-fr,
  ),
)

