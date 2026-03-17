# Blockst – Scratch Blocks in Typst

**Blockst** renders Scratch-style programming blocks directly in documents. It is aimed at worksheets, tutorials, classroom material, and visual explanations of programming concepts.

> **⚠️ Work in Progress**  
> This project is under active development. The blocks are functional, but there is still room for improvement in matching the original Scratch block geometry pixel-perfectly. **Contributions are very welcome!** Especially for:
> - 🎨 Fine-tuning block geometry and colours
> - 🌍 Localisation into additional languages (currently: German, English, and French)
> - 📚 Adding missing blocks or features
> - 🐛 Bug reports and feedback

## Features

- ✅ **All Scratch categories:** Motion, Looks, Sound, Events, Control, Sensing, Operators, Variables, Lists, and custom blocks
- ✅ **Accurate colours:** Normal and high-contrast themes
- ✅ **Nested structures:** Loops, conditionals (if-then-else), custom block definitions
- ✅ **Reporters & operators:** Oval and round pills, diamond conditions
- ✅ **Multilingual:** German, English, and French blocks available
- ✅ **Variable & list monitors:** Visual display widgets like in the Scratch stage
- ✅ **Scratch-Run:** Executable drawing programs on a canvas (turtle graphics)

## Installation

Import the package from Typst Universe:

```typst
#import "@preview/blockst:0.0.2": blockst, scratch, set-blockst
```

## Quick Start

Wrap your blocks in `#blockst[...]` and import the language module of your choice inside:

```typst
#import "@preview/blockst:0.0.2": blockst, scratch

#blockst[
  #import scratch.de: *

  #wenn-gruene-flagge-geklickt[
    #wiederhole(anzahl: 10)[
      #gehe(schritte: 10)
      #drehe-rechts(grad: 36)
    ]
  ]
]
```

---

## Examples

### Example 1: German Blocks

```typst
#import "@preview/blockst:0.0.2": blockst, scratch

#blockst[
  #import scratch.de: *

  #wenn-gruene-flagge-geklickt[
    #setze-variable("Punkte", 0)
    #wiederhole(anzahl: 5)[
      #gehe(schritte: 10)
      #falls-sonst(
        wird-beruehrt("Rand"),
        [#drehe-rechts(grad: 180)],
        [#aendere-variable("Punkte", 1)],
      )
    ]
    #sage-fuer-sekunden(eigene-eingabe("Punkte"), sekunden: 2)
  ]
]
```

![German blocks example](examples/example-de.png)

---

### Example 2: English Blocks

```typst
#import "@preview/blockst:0.0.2": blockst, scratch

#blockst[
  #import scratch.en: *

  #when-flag-clicked[
    #set-variable-to("Score", 0)
    #repeat(times: 5)[
      #move(steps: 10)
      #if-then-else(
        touching-object("edge"),
        [#turn-right(degrees: 180)],
        [#change-variable-by("Score", 1)],
      )
    ]
    #say-for-secs(custom-input("Score"), secs: 2)
  ]
]
```

![English blocks example](examples/example-en.png)

---

### Example 3: Variable & List Monitors

Monitor widgets display variables and lists exactly as they appear on the Scratch stage.

```typst
#import "@preview/blockst:0.0.2": blockst, scratch

#blockst[
  #import scratch.de: *

  // Program blocks
  #wenn-gruene-flagge-geklickt[
    #setze-variable("Highscore", 0)
    #fuege-zu-liste-hinzu("Anna", "Spieler")
    #fuege-zu-liste-hinzu("Ben", "Spieler")
    #fuege-zu-liste-hinzu("Clara", "Spieler")
  ]

  // Visual monitors (like on the Scratch stage)
  #variable(name: "Highscore", wert: 100)

  #liste(
    name: "Spieler",
    items: ("Anna", "Ben", "Clara"),
  )
]
```

![Variable and list monitor example](examples/example-monitors.png)

---

### Example 4: Custom Blocks

```typst
#import "@preview/blockst:0.0.2": blockst, scratch

#blockst[
  #import scratch.de: *

  #let zeichne-n-eck = eigener-block(
    "zeichne ",
    (name: "n"),
    "-Eck mit Seite ",
    (name: "s"),
  )

  #definiere(zeichne-n-eck)[
    #wiederhole(anzahl: parameter("n"))[
      #gehe(schritte: parameter("s"))
      #drehe-rechts(grad: dividiere(360, parameter("n")))
    ]
  ]

  #wenn-gruene-flagge-geklickt[
    #zeichne-n-eck(6, 40)
    #zeichne-n-eck(4, 60)
  ]
]
```

![Custom block definition example](examples/example-custom.png)

---

### Example 5: Scratch-Run (Turtle Graphics)

`scratch-run` draws turtle-graphics commands on a canvas. The result is a fixed-size box, so it never overflows the page.

```typst
#import "@preview/blockst:0.0.2": blockst, scratch, scratch-run, set-scratch-run

#import scratch.exec.de: *

// Simple square
#scratch-run(
  stift-ein(),
  quadrat(groesse: 70),
)

// Coloured triangle with coordinate axes
#set-scratch-run(zeige-gitter: true, zeige-achsen: true)

#scratch-run(
  setze-farbe(farbe: rgb("#4C97FF")),
  setze-dicke(dicke: 2),
  stift-ein(),
  dreieck(groesse: 70),
)
```

![Scratch-Run drawing example](examples/example-run.png)

---

## Project Structure

```
lib.typ                    ← Public API (blockst, scratch, set-blockst, scratch-run …)
libs/scratch/
  core.typ                 ← Central dispatcher (schema-based rendering)
  mod.typ                  ← Barrel re-export of all rendering modules
  registry.typ             ← Hierarchical block registry (ID → shape + category)
  category-map.typ         ← Maps registry groups to colour categories
  interpreter.typ          ← Turtle-graphics interpreter (scratch-run)
  rendering/
    colors.typ             ← Colour palettes and theme state
    geometry.typ           ← Layout constants, notch and block paths
    pills.typ              ← Pill primitives (reporter shapes)
    blocks.typ             ← scratch-block(), condition()
    categories.typ         ← Category wrappers, monitor widgets, custom blocks
    controls.typ           ← Control structures (loops, conditionals)
    icons.typ              ← SVG icons (green flag, arrows, …)
  lang/
    de.typ                 ← German alias functions
    en.typ                 ← English alias functions
    fr.typ                 ← French alias functions
    translations/
      de.toml              ← German translation strings
      en.toml              ← English translation strings
      fr.toml              ← French translation strings
  exec/
    de.typ                 ← German drawing commands (scratch.exec.de)
    en.typ                 ← English drawing commands (scratch.exec.en)
    fr.typ                 ← French drawing commands (scratch.exec.fr)
```

### Extensibility

The package uses a **schema-based rendering system**: every block is described as a data record in `registry.typ` (shape, category). The actual display text lives in TOML files. The dispatch logic in `core.typ` looks up colour, shape, and translation string by block ID (e.g. `"motion.move_steps"`) and renders the block uniformly.

Adding a new block typically requires only three steps:

1. **`registry.typ`** — add an entry in the appropriate group (`shape`, `category`)
2. **`de.toml` / `en.toml` / `fr.toml`** — add a translation string with `{placeholder}` slots
3. **`lang/de.typ` / `lang/en.typ` / `lang/fr.typ`** — add an alias function via `block("group.key", args: …)`

For blocks with special geometry (e.g. new loop shapes), extend `rendering/controls.typ` as well.

### Adding a New Language

To add a new language:

1. **`libs/scratch/lang/translations/xx.toml`** — create a TOML file with all translation strings (modelled after `de.toml`)
2. **`libs/scratch/lang/xx.typ`** — create an alias file with language-specific function names (modelled after `en.typ`)
3. **`core.typ`** — extend `_TRANS_DE`/`_TRANS_EN` and `get-template()` to include the new language code
4. **`lib.typ`** — export `scratch.xx` as a new dictionary entry

The rendering engine itself is fully language-neutral — all visible text in blocks comes from the TOML files.

---

## API Reference

### Container Function

```typst
#blockst[
  #import scratch.de: *  // or scratch.en / scratch.fr
  // your blocks here
]

// With explicit options
#blockst(theme: "high-contrast", scale: 80%)[
  #import scratch.de: *
  // …
]
```

### Global Settings

```typst
#set-blockst(
  theme: "normal",       // "normal" or "high-contrast"
  scale: 100%,           // scale factor for all blocks
  stroke-width: 1pt,     // outline thickness
)
```

### Language Selection

```typst
#import scratch.de: *   // German blocks
#import scratch.en: *   // English blocks
#import scratch.fr: *   // French blocks
```

### Scratch-Run (Turtle Graphics)

```typst
#import "@preview/blockst:0.0.2": scratch-run, set-scratch-run

#import scratch.exec.de: *   // or scratch.exec.en / scratch.exec.fr

#set-scratch-run(
  breite: 200,           // canvas width in units
  hoehe: 150,            // canvas height in units
  zeige-gitter: false,
  zeige-achsen: false,
)

#scratch-run(
  stift-ein(),
  gehe(schritte: 50),
  drehe-rechts(grad: 90),
  // …
)
```

---

## Available Categories

### 🔵 Motion
- `gehe(schritte: 10)` / `move(steps: 10)`
- `drehe-rechts(grad: 15)`, `drehe-links(grad: 15)` / `turn-right(degrees: 15)`, `turn-left(degrees: 15)`
- `gehe-zu(x: 0, y: 0)` / `go-to-xy(x: 0, y: 0)`
- `gleite-zu(sekunden: 1, x: 0, y: 0)` / `glide-to-xy(secs: 1, x: 0, y: 0)`
- `aendere-x(dx: 10)`, `setze-x(x: 0)`, `aendere-y(dy: 10)`, `setze-y(y: 0)`

### 🟣 Looks
- `sage(nachricht)`, `sage-fuer-sekunden(nachricht, sekunden: 2)` / `say(message)`, `say-for-secs(message, secs: 2)`
- `denke(nachricht)` / `think(message)`
- `wechsle-zu-kostuem(kostuem)`, `naechstes-kostuem()` / `switch-costume(costume)`, `next-costume()`
- `zeige-dich()`, `verstecke-dich()` / `show()`, `hide()`

### 🟡 Events
- `wenn-gruene-flagge-geklickt[body]` / `when-flag-clicked[body]`
- `wenn-taste-gedrueckt(taste)[body]` / `when-key-pressed(key)[body]`
- `wenn-diese-figur-angeklickt[body]` / `when-sprite-clicked[body]`
- `wenn-nachricht-empfangen(nachricht)[body]` / `when-message-received(message)[body]`
- `sende-nachricht(nachricht)` / `broadcast(message)`

### 🟠 Control
- `wiederhole(anzahl: 10)[body]` / `repeat(times: 10)[body]`
- `wiederhole-fortlaufend[body]` / `forever[body]`
- `falls(bedingung)[body]` / `if-then(condition)[body]`
- `falls-sonst(bedingung, dann, sonst)` / `if-then-else(condition, then-body, else-body)`
- `wiederhole-bis(bedingung)[body]` / `repeat-until(condition)[body]`
- `warte(dauer: 1)`, `warte-bis(bedingung)` / `wait(secs: 1)`, `wait-until(condition)`

### 🔷 Sensing
- `frage(frage)` / `ask(question)`
- `antwort()` / `answer()`
- `taste-gedrueckt(taste)` / `key-pressed(key)`
- `maus-x()`, `maus-y()` / `mouse-x()`, `mouse-y()`
- `wird-beruehrt(objekt)` / `touching-object(object)`
- `wird-farbe-beruehrt(farbe)` / `touching-color(color)`

### 🟢 Operators
- Arithmetic: `addiere`, `subtrahiere`, `multipliziere`, `dividiere` / `add`, `subtract`, `multiply`, `divide`
- Comparison: `groesser-als`, `kleiner-als`, `gleich` / `greater-than`, `less-than`, `equal-to`
- Logic: `und`, `oder`, `nicht` / `and-op`, `or-op`, `not-op`
- Text: `verbinde`, `zeichen-von`, `laenge-von` / `join`, `letter-of`, `length-of`
- Math: `zufallszahl(von: 1, bis: 10)`, `runde`, `modulo` / `random(from: 1, to: 10)`, `round`, `modulo`

### 🟠 Variables
- `setze-variable(variable, wert)` / `set-variable-to(variable, value)`
- `aendere-variable(variable, wert)` / `change-variable-by(variable, value)`
- `zeige-variable(variable)` / `show-variable(variable)`
- Monitor widget: `variable(name: "x", wert: 0)` / `variable-display(name: "x", value: 0)`

### 🟠 Lists
- `fuege-zu-liste-hinzu(element, liste)` / `add-to-list(item, list)`
- `entferne-aus-liste(index, liste)` / `delete-of-list(index, list)`
- `entferne-alles-aus-liste(liste)` / `delete-all-of-list(list)`
- Reporters: `element-von-liste`, `nummer-von-element`, `laenge-von-liste`
- Condition: `liste-enthaelt` / `list-contains-item`
- Monitor widget: `liste(name: "…", items: (…))` / `list(name: "…", items: (…))`

### 🩷 Custom Blocks
- `eigener-block(…)` / `custom-block(…)` — define a custom block template with parameters
- `definiere(label)[body]` / `define(label)[body]` — definition header block
- `parameter(name)` — parameter reporter pill (use inside define body)
- `eigene-eingabe(text)` / `custom-input(text)` — white input field reporter

---

## Configuration

```typst
#set-blockst(theme: "high-contrast")   // switch theme
#set-blockst(scale: 80%)               // shrink all blocks to 80 %
```

---

## Contributing

**Contributions are very welcome!** 🎉

This is a work in progress and we appreciate any help:

- 🐛 **Bug reports:** If something doesn't render correctly
- 🎨 **Design improvements:** Help match the original Scratch block appearance more closely
- 🌍 **Localisation:** Add support for more languages (see "Adding a New Language" above)
- 📚 **Documentation:** Extend examples or write tutorials
- ✨ **Features:** Add missing blocks or propose new functionality

Simply open an issue or a pull request on GitHub!

---

**Made with ❤️ for the Scratch and Typst community**
