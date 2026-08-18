// blockst — Handbuch (PDF) und Website (HTML) aus einer Quelle.
//
//     typst compile docs/docs.typ public --format bundle --features bundle,html --root .

#import "@schule/schuldocs:0.2.0": docs

#show: docs.with(
  toml: toml("../typst.toml"),
  authors: ("Loewe1000",),
  abstract: [
    *blockst* renders Scratch-style programming blocks directly in Typst
    documents — for worksheets, tutorials and teaching material. Scratch code is
    written as plain text and rendered by a bundled WASM plugin, in 26 languages
    including right-to-left scripts, with a turtle-graphics execution engine and
    helpers for importing real `.sb3` project files.
  ],
  links: ((name: "GitHub", url: "https://github.com/Loewe1000/blockst"),),
  notices: ([Part of the Schule Typst ecosystem],),
)

#include "content.typ"
