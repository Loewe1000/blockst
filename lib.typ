// =====================================================
// Scratch-Blöcke: Container-Funktion und Sprachmodule
// =====================================================

#import "scratch.typ": scratch-block-options

// Container-Funktion für Scratch-Blöcke
// Verwendung: #blockst[#import scratch.de: * ...]
#let blockst(
  theme: auto,
  scale: auto,
  body
) = context {
  // Hole aktuelle Optionen aus dem State
  let current-opts = scratch-block-options.get()
  
  // Verwende State-Werte, wenn auto übergeben wurde
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
  
  // Rendere Body mit Skalierung
  block(above: 2em, std.scale(final-scale, reflow: true, body))
}

// Globale Einstellungen für Scratch-Blöcke
// Verwendung: #set-blockst(theme: "dark", scale: 80%)
#let set-blockst(
  theme: none,
  scale: none,
) = {
  scratch-block-options.update(old => {
    let new-opts = old
    if theme != none {
      new-opts.insert("theme", theme)
    }
    if scale != none {
      new-opts.insert("scale", scale)
    }
    new-opts
  })
}

// Sprachmodule als Sub-Module
#import "lang/de.typ" as de
#import "lang/en.typ" as en

// Scratch-Namespace mit Sprachmodulen
#let scratch = (
  de: de,
  en: en,
)

// Scratch-Blöcke: Legacy-Import (für Abwärtskompatibilität)
#import "scratch.typ": *