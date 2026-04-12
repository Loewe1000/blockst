// rendering/controls.typ — Control structures (loops, conditionals)
// Contains repeat, repeat-until, forever, if-then-else blocks.

#import "colors.typ": scratch-block-options, get-colors-from-options, get-stroke-from-options, get-font-from-options
#import "icons.typ": icons, icon-by-theme
#import "geometry.typ": block-height, block-offset-y, corner-radius, content-inset, notch-spacing, block-path
#import "pills.typ": number-or-content
#import "blocks.typ": condition

// Alias to avoid shadowing by function parameters named `condition`
#let _condition-fn = condition

// ------------------------------------------------
// Common helper for loop and conditional blocks
// ------------------------------------------------
#let conditional-block(
  header-label,
  first-body: none, // First body (loop content or "then" branch)
  middle-notch: false,
  middle-label: none, // "else" label (only for if-block)
  second-body: none, // Second body (only "else" branch for if-block)
  bottom-notch: true,
  first-inset-notch: true,
  second-inset-notch: true,
  block-type: "loop", // "loop" or "if"
  empty-child-min-height: 0.5 * block-height,
  options: auto,
) = {
  if options == auto {
    context {
      let resolved-options = scratch-block-options.get()
      conditional-block(header-label, first-body: first-body, middle-notch: middle-notch, middle-label: middle-label, second-body: second-body, bottom-notch: bottom-notch, first-inset-notch: first-inset-notch, second-inset-notch: second-inset-notch, block-type: block-type, empty-child-min-height: empty-child-min-height, options: resolved-options)
    }
    return
  }

  let colors = get-colors-from-options(options)
  let stroke-thickness = get-stroke-from-options(options)

  block(
    above: 0em,
    below: 0mm,
  )[    #let first-body-is-empty = first-body in (none, [])
    #let second-body-is-empty = second-body in (none, [])

    #let measured-content-height(content) = {
      let direct = measure(content).height
      let boxed = measure(box(inset: 0mm, content)).height
      let blocked = measure(block(above: 0em, below: 0em, content)).height
      calc.max(direct, calc.max(boxed, blocked))
    }

    #let measured-content-width(content) = {
      let direct = measure(content).width
      let boxed = measure(box(inset: 0mm, content)).width
      calc.max(direct, boxed)
    }

    #let first-body = if not first-body-is-empty {
      first-body
    } else { box(height: empty-child-min-height + 2 * corner-radius, width: 0cm) }

    #let second-body = if not second-body-is-empty {
      second-body
    } else { box(height: empty-child-min-height + 2 * corner-radius, width: 0cm) }

    #let header-box = text(fill: colors.text-color, box(inset: content-inset, header-label))
    #let middle-box = if middle-label != none {
      text(fill: colors.text-color, box(inset: content-inset, middle-label))
    } else { none }

    #{
      // Path prefix based on block type
      let path-prefix = if block-type == "if" { "if" } else { "loop" }

      if middle-label == none {
        // === SINGLE BODY: measure header + body for absolute curve dimensions ===
        let header-width = measured-content-width(header-box)
        let header-height = calc.max(measured-content-height(header-box), block-height)

        let empty-body-threshold = 2 * corner-radius + 0.1mm
        let empty-body-box = box(height: empty-child-min-height + 2 * corner-radius, width: 0cm)
        let first-body-candidate = if first-body-is-empty { empty-body-box } else { first-body }
        let first-body-height = measured-content-height(first-body-candidate)
        let first-body-is-empty = first-body-is-empty or first-body-height <= empty-body-threshold
        let first-body-render = if first-body-is-empty { empty-body-box } else { first-body-candidate }
        let first-height-source = if first-body-is-empty {
          empty-child-min-height + 2 * corner-radius
        } else {
          first-body-height
        }
        let first-height = first-height-source - 2 * corner-radius

        // Draw curve with absolute dimensions
        place(top + left, dy: block-offset-y)[
          #curve(
            fill: colors.control.primary,
            stroke: (paint: colors.control.tertiary, thickness: stroke-thickness),
            ..block-path(header-height, header-width, path-prefix + "-header"),
            curve.line((0mm, first-height), relative: true),
            ..block-path(header-height, header-width, path-prefix + "-footer", bottom-notch: bottom-notch, top-notch: second-inset-notch),
          )
        ]
        if block-type == "loop" {
          place(bottom + left, dx: header-width - 0.5 * block-height)[
            #image(icon-by-theme("repeat", theme: options.at("theme", default: "normal")), height: 0.5 * block-height)
          ]
        }

        // Render content
        box(height: header-height, align(horizon, header-box))
        block(
          above: 0em,
          below: 0em,
          inset: (bottom: 3mm + 2 * corner-radius, left: 2 * notch-spacing),
          first-body-render,
        )
      } else {
        // === TWO BODIES (if-else): measure-based approach ===
        let header-width = measured-content-width(header-box)
        let header-height = calc.max(measured-content-height(header-box), block-height)
        let middle-height = calc.max(measured-content-height(middle-box), block-height)

        let empty-body-threshold = 2 * corner-radius + 0.1mm
        let empty-body-box = box(height: empty-child-min-height + 2 * corner-radius, width: 0cm)

        let first-body-candidate = if first-body-is-empty { empty-body-box } else { first-body }
        let second-body-candidate = if second-body-is-empty { empty-body-box } else { second-body }

        let first-body-height = measured-content-height(first-body-candidate)
        let second-body-height = measured-content-height(second-body-candidate)

        let first-body-is-empty = first-body-is-empty or first-body-height <= empty-body-threshold
        let second-body-is-empty = second-body-is-empty or second-body-height <= empty-body-threshold

        let first-body-render = if first-body-is-empty { empty-body-box } else { first-body-candidate }
        let second-body-render = if second-body-is-empty { empty-body-box } else { second-body-candidate }

        let first-height-source = if first-body-is-empty {
          empty-child-min-height + 2 * corner-radius
        } else {
          first-body-height
        }
        let second-height-source = if second-body-is-empty {
          empty-child-min-height + 2 * corner-radius
        } else {
          second-body-height
        }

        let first-height = first-height-source - 2 * corner-radius
        let second-height = second-height-source - 2 * corner-radius

        // Draw header and body
        place(top + left, dy: block-offset-y)[
          #curve(
            fill: colors.control.primary,
            stroke: (paint: colors.control.tertiary, thickness: stroke-thickness),
            ..block-path(header-height, header-width, path-prefix + "-header"),
            curve.line((0mm, first-height), relative: true),
            ..block-path(middle-height, header-width, path-prefix + "-middle", bottom-notch: first-inset-notch),
            curve.line((0mm, second-height), relative: true),
            ..block-path(header-height, header-width, path-prefix + "-footer", bottom-notch: bottom-notch, top-notch: second-inset-notch),
          )
        ]

        // Render content — each element with its own height
        box(height: header-height, align(horizon, header-box))
        block(
          above: 0em,
          below: 0em,
          inset: (bottom: corner-radius, left: 2 * notch-spacing),
          first-body-render,
        )
        box(height: middle-height, align(horizon, middle-box))
        block(
          above: 0em,
          below: 0em,
          inset: (bottom: 3mm + 2 * corner-radius, left: 2 * notch-spacing),
          second-body-render,
        )
      }
    }
  ]
}

// ------------------------------------------------
// Repeat n times
// ------------------------------------------------
#let repeat(count: 10, body: none, labels: (repeat: "repeat", times: "times"), options: auto) = {
  if options == auto {
    context {
      let resolved-options = scratch-block-options.get()
      repeat(count: count, body: body, labels: labels, options: resolved-options)
    }
    return
  }
  let colors = get-colors-from-options(options)
  conditional-block(
    [#stack(dir: ltr, spacing: 1.5mm, labels.repeat, number-or-content(count, colors.control, options: options), labels.times)],
    first-body: body,
    options: options,
  )
}

// ------------------------------------------------
// Repeat until condition
// ------------------------------------------------
#let repeat-until(condition, body: none, labels: (repeat-until: "repeat until"), options: auto) = {
  if options == auto {
    context {
      let resolved-options = scratch-block-options.get()
      repeat-until(condition, body: body, labels: labels, options: resolved-options)
    }
    return
  }
  let colors = get-colors-from-options(options)
  conditional-block(
    [#stack(dir: ltr, spacing: 1.5mm, labels.at("repeat-until"), if condition != [] and condition != "" { condition } else { _condition-fn(colorschema: colors.control, [], options: options) })],
    first-body: body,
    options: options,
  )
}

// ------------------------------------------------
// Repeat forever (infinite loop)
// ------------------------------------------------
#let repeat-forever(body, labels: (forever: "forever"), options: auto) = {
  if options == auto {
    context {
      let resolved-options = scratch-block-options.get()
      repeat-forever(body, labels: labels, options: resolved-options)
    }
    return
  }
  conditional-block(
    [#stack(dir: ltr, spacing: 1.5mm, labels.forever)],
    first-body: body,
    bottom-notch: false,
    options: options,
  )
}

// ------------------------------------------------
// If-then-else block
// ------------------------------------------------
#let if-then-else(
  condition,
  then: none,
  else-body: none,
  then-end: false,
  else-end: false,
  labels: ("if-then": "if", then: "then", "else": "else"),
  options: auto,
) = {
  if options == auto {
    context {
      let resolved-options = scratch-block-options.get()
      if-then-else(condition, then: then, else-body: else-body, then-end: then-end, else-end: else-end, labels: labels, options: resolved-options)
    }
    return
  }

  let colors = get-colors-from-options(options)
  conditional-block(
    [#stack(dir: ltr, spacing: 1.5mm, labels.at("if-then"), if condition != [] and condition != "" { condition } else { _condition-fn(colorschema: colors.control, [], options: options) }, labels.then)],
    first-body: then,
    middle-label: if else-body != none { [#stack(dir: ltr, spacing: 1.5mm, labels.at("else"))] } else { none },
    second-body: else-body,
    block-type: "if",
    first-inset-notch: not else-end,
    second-inset-notch: not then-end,
    options: options,
  )
}

