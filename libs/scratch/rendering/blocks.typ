// rendering/blocks.typ — scratch-block renderer and condition (diamond) shape
// Contains the core scratch-block() and condition() functions.

#import "colors.typ": scratch-block-options, get-colors-from-options, get-stroke-from-options, get-font-from-options
#import "geometry.typ": block-height, block-offset-y, content-inset, pill-height, pill-inset-x, pill-inset-y, pill-spacing, corner-radius, block-path, hat-cp1-y
#import "pills.typ": pill-round, pill-reporter

// Prototype toggle: draw simple statement blocks relative to their outer shell
// instead of using measured width/height for the curve path.
#let statement-relative-shell = true
#let statement-relative-shell-debug = false
#let statement-relative-children-debug = false

// ------------------------------------------------
// Internal scratch-block function (takes explicit params)
// ------------------------------------------------
#let scratch-block-internal(colorschema, type: "event", top-notch: true, bottom-notch: true, dx: 0mm, dy: 0mm, body, children-array, colors, stroke-thickness) = block(
  above: 0em + if (type == "event" or type == "define") { 6mm } else { 0mm },
  below: 0mm + if (type == "event" or type == "define") { 6mm } else { 0mm },
)[
  #let styled-body = text(fill: colors.text-color, body)

  #let measured-content-height(content) = {
    let direct = measure(content).height
    let boxed = measure(box(inset: 0mm, content)).height
    let blocked = measure(block(above: 0em, below: 0em, content)).height
    calc.max(direct, calc.max(boxed, blocked))
  }

  #let measured-content-size(content) = {
    let direct = measure(content)
    let boxed = measure(box(inset: 0mm, content))
    (
      width: calc.max(direct.width, boxed.width),
      height: calc.max(direct.height, boxed.height),
    )
  }

  #let content-body = {
    // Ensure a minimum visual block height for text-only statement blocks,
    // while still growing naturally for nested/taller inline content.
    let content-height = measured-content-height(styled-body)
    let min-height = 0.75 * block-height
    if content-height < min-height {
      box(styled-body, height: min-height)
    } else {
      styled-body
    }
  }

  #let content-box = align(horizon, box(
    inset: content-inset,
    height: if type == "define" { 1.5 * block-height } else { auto },
    [#content-body],
  ))

  #if type == "statement" and statement-relative-shell [
    #{
      let shell_size = measured-content-size(content-box)
      box(
        width: shell_size.width,
        height: shell_size.height,
        inset: 0mm,
        stroke: if statement-relative-shell-debug {
          (paint: rgb("ff2a2a"), thickness: 0.15mm)
        } else {
          none
        },
        clip: false,
        [
          #place(top + left, dx: dx, dy: dy)[
            #box(width: 100%, height: 100%)[
              #curve(
                fill: colorschema.primary,
                stroke: (paint: colorschema.tertiary, thickness: stroke-thickness),
                ..block-path(100%, 100%, type, bottom-notch: bottom-notch, top-notch: top-notch),
              )
            ]
          ]
          #content-box
        ],
      )
    }
  ] else [
    #let top-inset = if type == "event" { hat-cp1-y } else { 0mm }
    #box(inset: (top: top-inset), clip: false, [
      #{
        let size = measured-content-size(content-box)
        let width = size.width
        let height = size.height
        place(top + left, dx: dx, dy: dy)[
          #curve(
            fill: colorschema.primary,
            stroke: (paint: colorschema.tertiary, thickness: stroke-thickness),
            ..block-path(height, width, type, bottom-notch: bottom-notch, top-notch: top-notch),
          )
        ]
      }
      #content-box
    ])
  ]

  #v(dy, weak: true)
  #if children-array.len() != none {
    for child in children-array {
      if std.type(child) == content {
        if statement-relative-children-debug {
          box(
            inset: 0mm,
            stroke: (paint: rgb("22aa22"), thickness: 0.15mm),
            child,
          )
        } else {
          child
        }
      }
    }
  }
]

// ------------------------------------------------
// Public scratch-block function (uses state)
// ------------------------------------------------
#let scratch-block(colorschema: auto, type: "event", top-notch: true, bottom-notch: true, dx: 0mm, dy: 0mm, options: auto, body, ..children) = {
  if options == auto {
    context {
      let resolved-options = scratch-block-options.get()
      scratch-block(colorschema: colorschema, type: type, top-notch: top-notch, bottom-notch: bottom-notch, dx: dx, dy: dy, options: resolved-options, body, ..children)
    }
    return
  }

  let colors = get-colors-from-options(options)
  let stroke-thickness = get-stroke-from-options(options)
  let final-colorschema = if colorschema == auto { colors.motion } else { colorschema }

  scratch-block-internal(
    final-colorschema,
    type: type,
    top-notch: top-notch,
    bottom-notch: bottom-notch,
    dx: dx,
    dy: dy,
    body,
    children.pos(),
    colors,
    stroke-thickness,
  )
}

// ------------------------------------------------
// Condition (diamond shape for boolean values)
// ------------------------------------------------
#let condition(colorschema: auto, type: "condition", body, nested: false, options: auto) = {
  if options == auto {
    context {
      let resolved-options = scratch-block-options.get()
      condition(colorschema: colorschema, type: type, body, nested: nested, options: resolved-options)
    }
    return
  }

  let colors = get-colors-from-options(options)
  let stroke-thickness = get-stroke-from-options(options)
  let final-colorschema = if colorschema == auto { colors.control } else { colorschema }

  // Fast path for empty conditions: avoid nested box/place/measure overhead
  if body == [] {
    return box(width: 1.5 * pill-height, height: pill-height, baseline: 50%)[
      #place(bottom + left)[
        #curve(
          fill: final-colorschema.tertiary,
          stroke: none,
          ..block-path(pill-height, pill-height, type),
        )
      ]
    ]
  }

  box([
    // nested can be bool (both sides same) or (left, right) array
    #let nested-type = std.type(nested)
    #let (nested-left, nested-right) = if nested-type == array {
      (nested.at(0), nested.at(1))
    } else {
      (nested, nested)
    }
    #let x-inset-left = if nested-left { -0.5 } else { -0.1 }
    #let x-inset-right = if nested-right { -0.25 } else { -0.05 }
    #let content-box = {
      let body = if std.type(body) != array { (body,) } else { body }
      box(inset: (left: pill-inset-x * x-inset-left, right: pill-inset-x * x-inset-right, y: pill-inset-y), align(horizon, [
        #grid(
          columns: (body.len() * 2 + 1) * (auto,),
          column-gutter: 1fr,
          align: center + horizon,
          h(pill-spacing),
          ..body.map(x => { (x, h(0.25em)) }).flatten(),
          h(pill-spacing),
        )
      ]))
    }
    
    #let styled-content-box = text(fill: colors.text-color, content-box)
    
    #{
      let m = measure(styled-content-box, height: auto)
      let height = if m.height < block-height {
        block-height
      } else {
        m.height
      }
      let width = m.width
      place(bottom + left)[
        #curve(
          fill: final-colorschema.primary,
          stroke: (paint: final-colorschema.tertiary, thickness: stroke-thickness),
          ..block-path(height, width, type),
        )
      ]
      box(width: width + 0.5 * height, height: height, baseline: 50%, align(left + horizon, styled-content-box))
    }
  ])
}
