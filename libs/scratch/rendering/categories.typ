// rendering/categories.typ — Category wrapper blocks, event blocks, reporters,
//                              custom blocks, variable/list monitors

#import "colors.typ": get-colors-from-options, get-stroke-from-options, scratch-block-options
#import "icons.typ": icons
#import "geometry.typ": block-height, block-offset-y, block-path, content-inset, corner-radius, notch-spacing
#import "pills.typ": number-or-content, pill-min-height, pill-reporter, pill-round
#import "blocks.typ": condition, scratch-block

// ------------------------------------------------
// Category wrappers (statement blocks)
// ------------------------------------------------

// Generic base: one function drives all plain category statement blocks.
// `color-key` must match a field name in the colors dictionary (e.g. "motion").
#let category-statement(color-key, body, bottom-notch: true) = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  scratch-block(
    colorschema: colors.at(color-key),
    type: "statement",
    dy: block-offset-y,
    bottom-notch: bottom-notch,
    body,
  )
}

// Thin aliases — preserved for call-site compatibility
#let motion(body) = category-statement("motion", body)
#let looks(body) = category-statement("looks", body)
#let sound(body) = category-statement("sound", body)
#let sensing(body) = category-statement("sensing", body)
#let control(body, bottom-notch: true) = category-statement("control", body, bottom-notch: bottom-notch)
#let variables(body) = category-statement("variables", body)
#let lists(body) = category-statement("lists", body)
#let pen(body) = category-statement("pen", body)

// custom() has its own dark-mode logic — kept separate
#let custom(body, dark: false) = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  scratch-block(
    colorschema: if dark {
      (primary: colors.custom.secondary, tertiary: colors.custom.tertiary)
    } else {
      colors.custom
    },
    type: "statement",
    dy: block-offset-y,
    body,
  )
}

// ------------------------------------------------

// ------------------------------------------------
// Event blocks
// ------------------------------------------------
#let event(body, children) = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  scratch-block(colorschema: colors.events, type: "event", body, children)
}

// When green flag clicked
#let event-green-flag(children) = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  scratch-block(
    colorschema: colors.events,
    type: "event",
    grid(
      columns: 3,
      gutter: 0.5em,
      align: horizon,
      [Wenn], box(image(icon-by-theme("green-flag", theme: options.at("theme", default: "normal")))), [angeklickt wird],
    ),
    children,
  )
}

// When key pressed
#let event-key-pressed(key, children) = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  let stroke-thickness = get-stroke-from-options(options)
  scratch-block(
    colorschema: colors.events,
    type: "event",
    stack(dir: ltr, spacing: 1.5mm, "Wenn", pill-rect(key, fill: colors.events.primary, stroke: colors.events.tertiary + stroke-thickness, dropdown: true), "gedrückt wird"),
    children,
  )
}

// When sprite clicked
#let event-sprite-clicked(children) = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  scratch-block(
    colorschema: colors.events,
    type: "event",
    stack(dir: ltr, spacing: 1.5mm, "Wenn diese Figur angeklickt wird"),
    children,
  )
}

// When backdrop switches to
#let event-backdrop-switches-to(name, children) = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  let stroke-thickness = get-stroke-from-options(options)
  scratch-block(
    colorschema: colors.events,
    type: "event",
    stack(dir: ltr, spacing: 1.5mm, "Wenn das Bühnenbild zu", pill-rect(name, fill: colors.events.primary, stroke: colors.events.tertiary + stroke-thickness, dropdown: true), "wechselt"),
    children,
  )
}

// When greater than threshold
#let event-greater-than(element, value, children) = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  let stroke-thickness = get-stroke-from-options(options)
  scratch-block(
    colorschema: colors.events,
    type: "event",
    stack(dir: ltr, spacing: 1.5mm, "Wenn", pill-rect(element, fill: colors.events.primary, stroke: colors.events.tertiary + stroke-thickness, dropdown: true), ">", number-or-content(
      value,
      colors.events,
    )),
    children,
  )
}

// When I receive message
#let event-message-received(message, children) = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  let stroke-thickness = get-stroke-from-options(options)
  scratch-block(
    colorschema: colors.events,
    type: "event",
    stack(dir: ltr, spacing: 1.5mm, "Wenn ich", pill-rect(message, fill: colors.events.primary, stroke: colors.events.tertiary + stroke-thickness, dropdown: true), "empfange"),
    children,
  )
}

// Event statement block (no hat)
#let event-statement(body) = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  scratch-block(colorschema: colors.events, type: "statement", dy: block-offset-y, body)
}

// Broadcast message
#let broadcast-message(message, wait: false) = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  let stroke-thickness = get-stroke-from-options(options)
  event-statement(
    stack(dir: ltr, spacing: 1.5mm, "sende", pill-rect(message, fill: colors.events.secondary, stroke: colors.events.tertiary + stroke-thickness, dropdown: true, inline: true), if wait {
      "an alle und warte"
    } else { "an alle" }),
  )
}

// When I start as a clone (event shape with control colors)
#let when-i-start-as-clone(children, label: "when I start as a clone") = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  scratch-block(colorschema: colors.control, type: "event", [#label], children)
}

// Create clone of
#let create-clone-of(element: "mir selbst") = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  let stroke-thickness = get-stroke-from-options(options)
  control(
    stack(dir: ltr, spacing: 1.5mm, "erstelle Klon von", pill-reporter(element, fill: colors.control.secondary, stroke: colors.control.tertiary + stroke-thickness, dropdown: true, inline: true)),
  )
}

// ------------------------------------------------

// ------------------------------------------------
// Reporter blocks (value blocks)
// ------------------------------------------------
// Generic reporter function for all categories
#let reporter(colorschema: auto, body, dropdown-content: none, body-min-height: pill-min-height, enforce-min-height: true) = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  let stroke-thickness = get-stroke-from-options(options)
  let final-colorschema = if colorschema == auto { colors.looks } else { colorschema }
  let simple-body = type(body) in (str, int, float)
  let resolved-body-min-height = if enforce-min-height or simple-body { body-min-height } else { auto }

  pill-reporter(
    fill: final-colorschema.primary,
    stroke: final-colorschema.tertiary + stroke-thickness,
    text-color: colors.text-color,
    if dropdown-content != none {
      pill-round(fill: none, stroke: none, text-color: colors.text-color, inset: (x: 0mm, y: 0.5mm), min-height: resolved-body-min-height, stack(
        dir: ltr,
        spacing: pill-spacing,
        box(inset: (left: pill-inset-x), body),
        pill-reporter(
          dropdown-content,
          fill: final-colorschema.secondary,
          stroke: final-colorschema.tertiary + stroke-thickness,
          text-color: colors.text-color,
          dropdown: true,
          inline: true,
        ),
      ))
    } else {
      pill-round(body, fill: none, stroke: none, text-color: colors.text-color, inset: (x: 1.5mm, y: 0.5mm), min-height: resolved-body-min-height)
    },
  )
}

// Category-specific reporters — thin aliases over the generic reporter()
#let motion-reporter(body, dropdown-content: none) = {
  let c = get-colors-from-options(scratch-block-options.get())
  reporter(colorschema: c.motion, body, dropdown-content: dropdown-content)
}
#let looks-reporter(body, dropdown-content: none) = {
  let c = get-colors-from-options(scratch-block-options.get())
  reporter(colorschema: c.looks, body, dropdown-content: dropdown-content)
}
#let sound-reporter(body, dropdown-content: none) = {
  let c = get-colors-from-options(scratch-block-options.get())
  reporter(colorschema: c.sound, body, dropdown-content: dropdown-content)
}
#let sensing-reporter(body, dropdown-content: none) = {
  let c = get-colors-from-options(scratch-block-options.get())
  reporter(colorschema: c.sensing, body, dropdown-content: dropdown-content, enforce-min-height: true)
}
#let variables-reporter(body, dropdown-content: none) = {
  let c = get-colors-from-options(scratch-block-options.get())
  reporter(colorschema: c.variables, body, dropdown-content: dropdown-content)
}
#let lists-reporter(body, dropdown-content: none) = {
  let c = get-colors-from-options(scratch-block-options.get())
  reporter(colorschema: c.lists, body, dropdown-content: dropdown-content)
}
#let custom-reporter(body, dropdown-content: none) = {
  let c = get-colors-from-options(scratch-block-options.get())
  reporter(colorschema: c.custom, body, dropdown-content: dropdown-content)
}
#let pen-reporter(body, dropdown-content: none) = {
  let c = get-colors-from-options(scratch-block-options.get())
  reporter(colorschema: c.pen, body, dropdown-content: dropdown-content)
}


// ------------------------------------------------
// Parameter reporter (pink) for custom block parameters
// ------------------------------------------------
#let parameter(name) = {
  context {
    let options = scratch-block-options.get()
    let colors = get-colors-from-options(options)
    let stroke-thickness = get-stroke-from-options(options)
    pill-round(name, fill: colors.custom.primary, stroke: colors.custom.tertiary + stroke-thickness)
  }
}

// ------------------------------------------------
// Custom blocks
// ------------------------------------------------
// White argument placeholder for custom blocks
#let custom-input(text) = {
  context {
    let options = scratch-block-options.get()
    let colors = get-colors-from-options(options)
    let stroke-thickness = get-stroke-from-options(options)
    pill-round(text, stroke: colors.custom.tertiary + stroke-thickness)
  }
}

// Creates a custom statement block with text and placeholders.
// Usage:
//   #let my-block = custom-block("rotate", custom-input("degrees"))
//   #my-block(45)[ ... ]
#let custom-block(..body) = {
  let items = body.pos()
  return (dark: true, ..values) => {
    context {
      let options = scratch-block-options.get()
      let colors = get-colors-from-options(options)
      let stroke-thickness = get-stroke-from-options(options)

      custom(dark: dark, {
        let values = values.pos()
        stack(
          dir: ltr,
          spacing: 1.5mm,
          ..if values.len() == 0 {
            for item in items {
              if std.type(item) == str {
                (item,)
              } else if std.type(item) == dictionary {
                (pill-round(stroke: colors.custom.tertiary, fill: colors.custom.primary, text-color: colors.text-color, item.name),)
              } else {
                (pill-round(stroke: colors.custom.tertiary, fill: colors.custom.primary, text-color: colors.text-color, str("number or text")),)
              }
            }
          } else {
            let key = 0
            for item in items {
              if std.type(item) == str {
                (item,)
              } else {
                (number-or-content(values.at(calc.rem(key, values.len())), colors.custom),)
                key += 1
              }
            }
          },
        )
      })
    }
  }
}

// Define block header (signature for custom block definitions)
#let define(block-label, verb: "define", ..children) = {
  context {
    let options = scratch-block-options.get()
    let colors = get-colors-from-options(options)
    let rendered-label = if std.type(block-label) == function {
      block-label(dark: true)
    } else {
      block-label
    }
    scratch-block(
      colorschema: colors.custom,
      type: "define",
      dy: 2.5 * corner-radius,
      stack(dir: ltr, spacing: 1.5mm, verb, rendered-label),
      ..children,
    )
  }
}


// ------------------------------------------------
// Variable monitor (visual display like in Scratch)
// ------------------------------------------------
#let variable-monitor(name: "Variable", value: 0) = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)

  box(
    fill: rgb("#E6F0FF"),
    stroke: (paint: rgb("#CAD1D9"), thickness: 1pt),
    radius: 2pt,
    inset: (left: 6pt, right: 6pt, y: 2pt),
  )[
    #set text(size: 7.85pt, font: ("Helvetica Neue", "Helvetica", "Arial"), weight: "bold", fill: rgb("#575E75"))
    #grid(
      columns: (auto, auto),
      column-gutter: 7pt,
      align: left + horizon,
      name,
      rect(
        fill: colors.variables.primary,
        stroke: none,
        radius: 2pt,
        inset: (x: 3pt, y: 1.5pt),
        grid(
          columns: 1,
          align: center + horizon,
          box(width: 20pt, height: 0pt),
          text(fill: white, size: 10pt, weight: "bold", str(value)),
        ),
      ),
    )
  ]
}

// ------------------------------------------------
// List monitor (visual display like in Scratch)
// ------------------------------------------------
#let list-monitor(name: "List", items: (), width: 4cm, height: auto, length-label: "Length") = {
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  let len = items.len()

  let bg-blue = rgb("#E6F0FF")
  let line-color = rgb("#CAD1D9")

  box(
    width: width,
    height: height,
    fill: bg-blue,
    stroke: (paint: line-color, thickness: 1pt),
    radius: 3pt,
    clip: true,
  )[
    #set text(font: ("Helvetica Neue", "Helvetica", "Arial"))
    #grid(
      columns: 1,
      rows: (15pt, auto, 15pt),
      rect(
        width: 100%,
        height: 100%,
        fill: white,
        stroke: (bottom: line-color + 2pt),
        inset: (x: 4pt, y: 0pt),
        align(center + horizon, text(fill: rgb("#575E75"), size: 8pt, weight: "bold", name)),
      ),
      box(
        width: 100%,
        clip: true,
        {
          let needs-scrollbar = false
          let available-h = 0pt
          let content-h = 0pt
          if type(height) == length {
            // Approx 20pt per row. Header and footer take 30pt.
            available-h = height - 30pt
            content-h = len * 20pt
            if content-h > available-h {
              needs-scrollbar = true
            }
          }
          let right-inset = if needs-scrollbar { 14pt } else { 4pt }

          let item-rows = items
            .enumerate()
            .map(((index, item)) => {
              rect(
                width: 100%,
                fill: bg-blue,
                stroke: none,
                inset: (left: -3pt, right: right-inset, top: 1.5pt, bottom: 1.5pt),
                grid(
                  columns: (12pt, 1fr),
                  column-gutter: 4pt,
                  align: (right + horizon, left + horizon),
                  text(fill: rgb("#575E75"), size: 9pt, weight: "bold", str(index + 1)),
                  rect(
                    width: 100%,
                    fill: colors.lists.primary,
                    stroke: colors.lists.tertiary + 1pt,
                    radius: 2pt,
                    inset: (x: 3pt, y: 3.5pt),
                    text(fill: white, size: 8pt, weight: 400, item),
                  ),
                ),
              )
            })

          stack(dir: ttb, ..item-rows)

          if needs-scrollbar {
            let thumb-h = calc.max(10pt, available-h * (available-h / content-h))
            // Track
            place(
              right + top,
              dx: -2pt,
              dy: 2pt,
              rect(
                width: 7pt,
                height: available-h - 4pt,
                fill: rgb("#DBE4F3"),
                radius: 3.5pt,
                stroke: none,
              ),
            )
            // Thumb
            place(
              right + top,
              dx: -2pt,
              dy: 2pt,
              rect(
                width: 7pt,
                height: thumb-h,
                fill: rgb("#6E737B"),
                radius: 3.5pt,
                stroke: none,
              ),
            )
          }
        },
      ),
      rect(
        width: 100%,
        height: 100%,
        fill: white,
        inset: (x: 2.5pt, y: 5pt),
        grid(
          columns: (auto, 1fr, auto),
          align: (left + horizon, center + horizon, right + horizon),
          text(fill: rgb("#575E75"), size: 8pt, weight: "bold", "+"),
          text(fill: rgb("#575E75"), size: 8pt, weight: "bold", length-label + ": " + str(len)),
          text(fill: rgb("#575E75"), size: 8pt, weight: "bold", "="),
        ),
      ),
    )
  ]
}
