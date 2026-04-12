// rendering/pills.typ — Pill primitives and value/content helpers
// Contains all pill shapes (round, reporter, rect, color) and the
// number-or-content helper.

#import "colors.typ": *
#import "icons.typ": icons, icon-by-theme
#import "geometry.typ": pill-height, pill-inset-x, pill-inset-y, pill-spacing, block-height

// ------------------------------------------------
// Internal base pill function
// ------------------------------------------------
// Accepts explicit colors and stroke-thickness parameters
#let _pill-base-internal(
  fill: white,
  stroke: auto,
  text-color: auto,
  radius: 50%,
  inset: 0mm,
  height: auto,
  min-height: none,
  dropdown: false,
  body,
  colors: colors-normal,
  theme: "normal",
  stroke-thickness: 0.5pt,
) = {
  // Default stroke when auto
  let final-stroke = if stroke == auto {
    (paint: black, thickness: stroke-thickness)
  } else {
    stroke
  }

  // Automatic text color:
  // - Explicit value: use as given
  // - White background: use dark gray (normal) or black (high-contrast/print)
  // - Colored background: use theme text color
  let final-text-color = if text-color != auto {
    text-color
  } else if fill == white or fill == rgb("#FFFFFF") {
    if colors == colors-high-contrast or colors == colors-print {
      black
    } else {
      rgb("#575E75") // dark gray for normal theme
    }
  } else {
    colors.text-color
  }

  let content-box = if dropdown {
    stack(
      dir: ltr,
      spacing: pill-spacing,
      box(body),
      image(icon-by-theme("dropdown-arrow", theme: theme), height: 2mm),
    )
  } else {
    box(body)
  }

  let styled-box = text(fill: final-text-color, content-box)
  let auto-pill = box(
    fill: fill,
    stroke: final-stroke,
    radius: radius,
    height: auto,
    inset: inset,
    align(horizon, styled-box),
  )

  if height != auto {
    box(
      fill: fill,
      stroke: final-stroke,
      radius: radius,
      height: height,
      inset: inset,
      align(horizon, styled-box),
    )
  } else if min-height != none and min-height != auto {
    let auto-height = measure(auto-pill).height
    if auto-height < min-height {
      box(
        fill: fill,
        stroke: final-stroke,
        radius: radius,
        height: min-height,
        inset: inset,
        align(horizon, styled-box),
      )
    } else {
      auto-pill
    }
  } else {
    auto-pill
  }
}

// ------------------------------------------------
// Public base pill function (uses state)
// ------------------------------------------------
#let _pill-base(
  fill: white,
  stroke: auto,
  text-color: auto,
  radius: 50%,
  inset: 0mm,
  height: auto,
  min-height: none,
  dropdown: false,
  body,
  options: auto,
) = {
  if options == auto {
    context {
      let resolved-options = scratch-block-options.get()
      _pill-base(fill: fill, stroke: stroke, text-color: text-color, radius: radius, inset: inset, height: height, min-height: min-height, dropdown: dropdown, body, options: resolved-options)
    }
    return
  }

  let colors = get-colors-from-options(options)
  let theme = options.at("theme", default: "normal")
  let stroke-thickness = get-stroke-from-options(options)

  _pill-base-internal(
    fill: fill,
    stroke: stroke,
    text-color: text-color,
    radius: radius,
    inset: inset,
    height: height,
    min-height: min-height,
    dropdown: dropdown,
    body,
    colors: colors,
    theme: theme,
    stroke-thickness: stroke-thickness,
  )
}

// ------------------------------------------------
// Public pill variants
// ------------------------------------------------

#let pill-min-height = 0.75 * block-height
#let pill-string-min-height = pill-min-height
#let pill-dropdown-min-height = pill-min-height
#let pill-inline-operand-min-height = pill-min-height

// White input pills (fixed height, no insets)
#let pill-round(body, stroke: auto, inset: (x: 1.3 * pill-inset-x, y: 1mm), fill: white, text-color: auto, min-height: pill-min-height, options: auto) = _pill-base(
  fill: fill,
  stroke: stroke,
  text-color: text-color,
  radius: 50%,
  inset: inset,
  height: auto,
  min-height: min-height,
  dropdown: false,
  body,
  options: options,
)

// Colored reporter pills (auto height, reduced insets, min-height 0.8 * block-height)
#let pill-reporter(body, fill: white, stroke: auto, text-color: auto, dropdown: false, inline: false, options: auto) = {
  if options == auto {
    context {
      let resolved-options = scratch-block-options.get()
      pill-reporter(body, fill: fill, stroke: stroke, text-color: text-color, dropdown: dropdown, inline: inline, options: resolved-options)
    }
    return
  }

  let colors = get-colors-from-options(options)
  let stroke-thickness = get-stroke-from-options(options)

  _pill-base-internal(
    fill: fill,
    stroke: stroke,
    text-color: text-color,
    radius: 50%,
    inset: if inline {
      (x: pill-inset-x, y: 0.6 * pill-inset-y)
    } else {
      (x: 0.4 * pill-inset-x, y: 1.0 * pill-inset-y)
    },
    height: auto,
    dropdown: dropdown,
    body,
    colors: colors,
    stroke-thickness: stroke-thickness,
  )
}

// Rectangular dropdown pills (auto height, reduced insets)
#let pill-rect(body, fill: white, stroke: auto, text-color: auto, dropdown: false, inline: false, options: auto) = _pill-base(
  fill: fill,
  stroke: stroke,
  text-color: text-color,
  radius: 10%,
  inset: (x: 0.75 * pill-inset-x, y: if inline { 0mm } else { 0.75 * pill-inset-y }),
  height: auto,
  min-height: pill-dropdown-min-height,
  dropdown: dropdown,
  body,
  options: options,
)

// Color pills (for color picker)
#let pill-color(body, fill: white, options: auto) = {
  if options == auto {
    context {
      let resolved-options = scratch-block-options.get()
      pill-color(body, fill: fill, options: resolved-options)
    }
    return
  }

  let stroke-thickness = get-stroke-from-options(options)

  _pill-base(
    fill: fill,
    stroke: white + stroke-thickness,
    text-color: auto,
    radius: 50%,
    inset: 0mm,
    height: 1.2 * pill-height,
    dropdown: false,
    body,
    options: options,
  )
}

// Legacy pill() function wrapper for compatibility
#let pill(..args, type: "round", stroke: auto, text-color: auto, body, dropdown: false, inset: auto, height: auto, fill: white, options: auto) = {
  if type == "round" {
    pill-round(body, stroke: stroke, fill: fill, text-color: text-color, options: options)
  } else if type == "single" or type == "reporter" {
    pill-reporter(body, fill: fill, stroke: stroke, text-color: text-color, dropdown: dropdown, options: options)
  } else if type == "rect" {
    pill-rect(body, fill: fill, stroke: stroke, text-color: text-color, dropdown: dropdown, options: options)
  } else if type == "color" {
    pill-color(body, fill: fill, options: options)
  }
}

// ------------------------------------------------
// Value/content helper
// ------------------------------------------------
// Converts simple values (string, int, float) into pills,
// leaves content (blocks, reporters, etc.) unchanged.
#let number-or-content(value, colorschema, compact: false, tall: false, options: auto) = {
  if options == auto {
    context {
      let resolved-options = scratch-block-options.get()
      number-or-content(value, colorschema, compact: compact, tall: tall, options: resolved-options)
    }
    return
  }

  let value-type = type(value)
  if value-type == str or value-type == int or value-type == float {
    let stroke-thickness = get-stroke-from-options(options)
    let min-height = if tall {
      pill-inline-operand-min-height
    } else if value-type == str {
      if compact { pill-min-height } else { pill-string-min-height }
    } else {
      pill-min-height
    }

    pill-round(
      str(value),
      stroke: colorschema.tertiary + stroke-thickness,
      inset: (x: 1.3 * pill-inset-x, y: 0.5mm),
      min-height: min-height,
      options: options,
    )
  } else {
    value
  }
}


