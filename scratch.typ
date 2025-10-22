#set page(width: auto, height: auto, margin: 5mm)

#let high-contrast = false

// Standard Scratch-Farben
#let colors-normal = (
  bewegung: (fill: rgb("#4C97FF"), stroke: rgb("#3373CC"), dark-fill: rgb("#4181D7")),
  aussehen: (fill: rgb("#9A66FF"), stroke: rgb("#774DCB"), dark-fill: rgb("#855CD6")),
  klang: (fill: rgb("#CF63CF"), stroke: rgb("#BD42BD"), dark-fill: rgb("#C94FC9")),
  ereignisse: (fill: rgb("#FFBE00"), stroke: rgb("#CC9900")),
  steuerung: (fill: rgb("#FFAB1A"), stroke: rgb("#CF8B16")),
  fühlen: (fill: rgb("#5CB1D6"), stroke: rgb("#3594BD"), dark-fill: rgb("#47A8D1")),
  operatoren: (fill: rgb("#58C059"), stroke: rgb("#379438"), dark-fill: rgb("#379438")),
  variablen: (fill: rgb("#FF8C1A"), stroke: rgb("#D97400"), dark-fill: rgb("#E67E22")),
  listen: (fill: rgb("#FF6619"), stroke: rgb("#E64D00"), dark-fill: rgb("#E64D00")),
  eigene: (fill: rgb("#FF6680"), stroke: rgb("#FF3355"), dark-fill: rgb("#FF4D6A")),
)

// Hoher Kontrast Variante
#let colors-high-contrast = (
  bewegung: (fill: rgb("#7FB5FF"), stroke: rgb("#1A4D99"), dark-fill: rgb("#2666CC")),
  aussehen: (fill: rgb("#CCB3FF"), stroke: rgb("#5A2FA8"), dark-fill: rgb("#6F3FCC")),
  klang: (fill: rgb("#E19DE1"), stroke: rgb("#A32BA3"), dark-fill: rgb("#C13FC1")),
  ereignisse: (fill: rgb("#FFD966"), stroke: rgb("#B39300")),
  steuerung: (fill: rgb("#FFBE4C"), stroke: rgb("#B39300")),
  fühlen: (fill: rgb("#85C4E0"), stroke: rgb("#1E6E8F"), dark-fill: rgb("#2B8BB5")),
  operatoren: (fill: rgb("#7FCE7E"), stroke: rgb("#267326"), dark-fill: rgb("#2F8F2F")),
  variablen: (fill: rgb("#FFB380"), stroke: rgb("#B35900"), dark-fill: rgb("#CC6E28")),
  listen: (fill: rgb("#FF6619"), stroke: rgb("#E64D00"), dark-fill: rgb("#E64D00")),
  eigene: (fill: rgb("#FFA6B5"), stroke: rgb("#B33A4D"), dark-fill: rgb("#B33A4D")),
)

#let colors = if high-contrast {
  colors-high-contrast
} else {
  colors-normal
}

#let stroke-thickness = if high-contrast {
  1.5pt
} else {
  0.5pt
}

//#move(dy: 15.4mm, dx: -stroke-thickness, image("ereignis.svg", width: 4cm))
//#image("ereignis.svg", width: 4cm)

// Notch (Auskerbung) Dimensionen
#let notch-depth = 1.5mm
#let notch-width = 2.2mm
#let notch-cp-x = 0.75mm  // Bézierkurven-Kontrollpunkt

// Block-Dimensionen
#let block-height = 10mm
#let corner-radius = 0.75mm
#let block-offset-y = 1.5mm  // Vertikaler Offset für bewegung
#let notch-margin = 1.3mm    // Horizontaler Abstand vor/nach Notch
#let notch-total = 3mm       // Gesamtbreite der Notch-Region

// Hat (Kappe) Dimensionen für ereignis-Block
#let hat-cp1-x = 4mm
#let hat-cp1-y = 3.1mm
#let hat-cp2-x = 5.2mm

// Pill Dimensionen
#let pill-height = 6mm
#let pill-inset-x = 2.5mm
#let pill-inset-y = 1.25mm
#let pill-spacing = pill-inset-x * 0.66

// Layout
#let content-inset = 5pt
#let notch-path = (
  curve.cubic((-notch-cp-x, 0mm), (-notch-depth, notch-depth), (-notch-depth - notch-cp-x, notch-depth), relative: true),
  curve.line((-notch-width, 0mm), relative: true),
  curve.cubic((-notch-cp-x, 0mm), (-notch-depth, -notch-depth), (-notch-depth - notch-cp-x, -notch-depth), relative: true),
)

#let inverted-notch-path = (
  curve.cubic((notch-cp-x, 0mm), (notch-depth, notch-depth), (notch-depth + notch-cp-x, notch-depth), relative: true),
  curve.line((notch-width, 0mm), relative: true),
  curve.cubic((notch-cp-x, 0mm), (notch-depth, -notch-depth), (notch-depth + notch-cp-x, -notch-depth), relative: true),
)

// Basis-Funktion für alle Pills (intern)
#let _pill-base(
  fill: white,
  stroke: (paint: black, thickness: stroke-thickness),
  text-color: rgb("#575E75"),
  radius: 50%,
  inset: 0mm,
  height: auto,
  dropdown: false,
  body,
) = box(
  fill: fill,
  stroke: stroke,
  radius: radius,
  height: auto,
  inset: inset,
  align(horizon, if dropdown {
    context {
      let height = measure(body).height
      let height = if height < pill-height {
        pill-height
      } else {
        height
      }
      let width = pill-inset-x
      stack(dir: ltr, spacing: pill-spacing, box(height: height, text(text-color, body)), curve(
        fill: white,
        stroke: (paint: stroke.paint, thickness: stroke-thickness, cap: "round", join: "round"),
        curve.line((width, 0mm), relative: true),
        curve.line((-0.5 * width, 0.5 * width), relative: true),
        curve.close(),
      ))
    }
  } else {
    context [
      #let height = measure(body).height
      #let height = if height < pill-height {
        pill-height
      } else {
        height
      }
      #box(height: height, text(text-color, body))
    ]
  }),
)

// Weiße Input-Pills (feste Höhe 8.4mm, keine Insets)
#let pill-round(body, stroke: (paint: black, thickness: stroke-thickness), inset: (x: 1.3 * pill-inset-x, y: 1mm), fill: white, text-color: rgb("#575E75")) = _pill-base(
  fill: fill,
  stroke: stroke,
  text-color: text-color,
  radius: 50%,
  inset: inset,
  height: auto,
  dropdown: false,
  body,
)

// Farbige Reporter-Pills (auto-höhe, reduzierte Insets)
#let pill-reporter(body, fill: white, stroke: (paint: black, thickness: stroke-thickness), text-color: white, dropdown: false, inline: false) = _pill-base(
  fill: fill,
  stroke: stroke,
  text-color: text-color,
  radius: 50%,
  inset: if inline {
    (x: pill-inset-x, y: 0.7 * pill-inset-y)
  } else {
    (x: 0.4 * pill-inset-x, y: 0.7 * pill-inset-y)
  },
  height: if inline { 100% } else { auto },
  dropdown: dropdown,
  body,
)

// Rechteckige Dropdown-Pills (auto-höhe, reduzierte Insets)
#let pill-rect(body, fill: white, stroke: (paint: black, thickness: stroke-thickness), text-color: white, dropdown: false) = _pill-base(
  fill: fill,
  stroke: stroke,
  text-color: text-color,
  radius: 10%,
  inset: (x: 0.75 * pill-inset-x, y: 0.75 * pill-inset-y),
  height: pill-height,
  dropdown: dropdown,
  body,
)

// Farb-Pills (für Farbauswahl)
#let pill-color(body, fill: white) = _pill-base(
  fill: fill,
  stroke: white + stroke-thickness,
  text-color: white,
  radius: 50%,
  inset: 0mm,
  height: 1.2 * pill-height,
  dropdown: false,
  body,
)

// Alte pill() Funktion als Wrapper für Kompatibilität
#let pill(..args, type: "round", stroke: (paint: black, thickness: stroke-thickness), text-color: rgb("#575E75"), body, dropdown: false, inset: auto, height: auto, fill: white) = {
  if type == "round" {
    pill-round(body, stroke: stroke, fill: fill, text-color: text-color)
  } else if type == "single" or type == "reporter" {
    pill-reporter(body, fill: fill, stroke: stroke, text-color: text-color, dropdown: dropdown)
  } else if type == "rect" {
    pill-rect(body, fill: fill, stroke: stroke, text-color: text-color, dropdown: dropdown)
  } else if type == "color" {
    pill-color(body, fill: fill)
  }
}

// Helper-Funktion: Wert oder Content
// Wandelt einfache Werte (String, Int, Float) in Pills um,
// lässt Content (Blöcke, Reporter, etc.) unverändert
#let zahl-oder-content(value, colorschema) = {
  let value-type = type(value)
  if value-type == str {
    pill-round(value, stroke: colorschema.stroke + stroke-thickness)
  } else if value-type == int or value-type == float {
    pill-round(str(value), stroke: colorschema.stroke + stroke-thickness)
  } else {
    // Es ist bereits Content (bedingung, reporter, etc.)
    value
  }
}

#let block-path(height, width, type) = {
  return (
    ereignis: (
      curve.line((0mm, 0mm), relative: true),
      curve.quad((hat-cp1-x, -hat-cp1-y), (block-height, -hat-cp1-y), relative: true),
      curve.quad((hat-cp2-x, 0mm), (block-height, hat-cp1-y), relative: true),
      curve.line((width - 2 * block-height - corner-radius, 0mm), relative: true),
      curve.quad((corner-radius, 0mm), (corner-radius, corner-radius), relative: true),
      curve.line((0mm, height - 2 * corner-radius), relative: true),
      curve.quad((0mm, corner-radius), (-corner-radius, corner-radius), relative: true),
      curve.line((-width + 3.7mm + notch-margin + notch-total + corner-radius, 0mm), relative: true),
      ..notch-path,
      curve.line((-notch-margin + corner-radius, 0mm), relative: true),
      curve.quad((-corner-radius, 0mm), (-corner-radius, -corner-radius), relative: true),
      curve.close(),
    ),
    definiere: (
      curve.quad((0mm, -5 * corner-radius), (5 * corner-radius, -5 * corner-radius), relative: true),
      curve.line((width - 10 * corner-radius, 0mm), relative: true),
      curve.quad((5 * corner-radius, 0mm), (5 * corner-radius, 5 * corner-radius), relative: true),
      curve.line((0mm, height - corner-radius), relative: true),
      curve.quad((0mm, corner-radius), (-corner-radius, corner-radius), relative: true),
      curve.line((-(width - corner-radius - notch-total - notch-margin - 3.7mm), 0mm), relative: true),
      ..notch-path,
      curve.line((-notch-margin + corner-radius, 0mm), relative: true),
      curve.quad((-corner-radius, 0mm), (-corner-radius, -corner-radius), relative: true),
      curve.close(),
    ),
    anweisung: (
      curve.line((0mm, -block-offset-y + corner-radius), relative: true),
      curve.quad((0mm, -corner-radius), (corner-radius, -corner-radius), relative: true),
      curve.line((notch-margin - corner-radius, 0mm), relative: true),
      ..inverted-notch-path,
      curve.line((width - 3.7mm - notch-margin - notch-total, 0mm), relative: true),
      curve.quad((corner-radius, 0mm), (corner-radius, corner-radius), relative: true),
      curve.line((0mm, +block-offset-y - corner-radius), relative: true),
      curve.line((0mm, height - block-offset-y - corner-radius), relative: true),
      curve.quad((0mm, corner-radius), (-corner-radius, corner-radius), relative: true),
      curve.line((-width + 3.7mm + notch-margin + notch-total, 0mm), relative: true),
      ..notch-path,
      curve.line((-notch-margin + corner-radius, 0mm), relative: true),
      curve.quad((-corner-radius, 0mm), (-corner-radius, -corner-radius), relative: true),
      curve.close(),
    ),
    bedingung: (
      curve.move((0.5 * height, 0mm)),
      curve.line((width - 0.5 * height, 0mm), relative: true),
      curve.line((0.5 * height, -0.5 * height), relative: true),
      curve.line((-0.5 * height, -0.5 * height), relative: true),
      curve.line((-width + 0.5 * height, 0mm), relative: true),
      curve.line((-0.5 * height, 0.5 * height), relative: true),
      curve.line((0.5 * height, 0.5 * height), relative: true),
    ),
    loop-header: (
      curve.line((0mm, -block-offset-y + corner-radius), relative: true),
      curve.quad((0mm, -corner-radius), (corner-radius, -corner-radius), relative: true),
      curve.line((notch-margin - corner-radius, 0mm), relative: true),
      ..inverted-notch-path,
      curve.line((width - 3.7mm - notch-margin - notch-total, 0mm), relative: true),
      curve.quad((corner-radius, 0mm), (corner-radius, corner-radius), relative: true),
      curve.line((0mm, +block-offset-y - corner-radius), relative: true),
      curve.line((0mm, height - block-offset-y - corner-radius), relative: true),
      curve.quad((0mm, corner-radius), (-corner-radius, corner-radius), relative: true),
      curve.line((-width + 3.7mm + 3 * notch-margin + notch-total, 0mm), relative: true),
      ..notch-path,
      curve.line((-notch-margin + corner-radius, 0mm), relative: true),
      curve.quad((-corner-radius, 0mm), (-corner-radius, corner-radius), relative: true),
    ),
    loop-footer: (
      curve.quad((0mm, corner-radius), (corner-radius, corner-radius), relative: true),
      curve.line((notch-margin - corner-radius, 0mm), relative: true),
      ..inverted-notch-path,
      curve.line((width - 3.7mm - 3 * notch-margin - notch-total, 0mm), relative: true),
      curve.quad((corner-radius, 0mm), (corner-radius, corner-radius), relative: true),
      curve.line((0mm, 3mm), relative: true),
      curve.quad((0mm, corner-radius), (-corner-radius, corner-radius), relative: true),
      curve.line((-width + 3.7mm + 1 * notch-margin + notch-total, 0mm), relative: true),
      ..notch-path,
      curve.line((-notch-margin + corner-radius, 0mm), relative: true),
      curve.quad((-corner-radius, 0mm), (-corner-radius, -corner-radius), relative: true),
      curve.close(),
    ),
    falls-header: (
      curve.line((0mm, -block-offset-y + corner-radius), relative: true),
      curve.quad((0mm, -corner-radius), (corner-radius, -corner-radius), relative: true),
      curve.line((notch-margin - corner-radius, 0mm), relative: true),
      ..inverted-notch-path,
      curve.line((width - 3.7mm - notch-margin - notch-total, 0mm), relative: true),
      curve.quad((corner-radius, 0mm), (corner-radius, corner-radius), relative: true),
      curve.line((0mm, +block-offset-y - corner-radius), relative: true),
      curve.line((0mm, height - block-offset-y - corner-radius), relative: true),
      curve.quad((0mm, corner-radius), (-corner-radius, corner-radius), relative: true),
      curve.line((-width + 3.7mm + 3 * notch-margin + notch-total, 0mm), relative: true),
      ..notch-path,
      curve.line((-notch-margin + corner-radius, 0mm), relative: true),
      curve.quad((-corner-radius, 0mm), (-corner-radius, corner-radius), relative: true),
    ),
    falls-middle: (
      curve.quad((0mm, corner-radius), (corner-radius, corner-radius), relative: true),
      curve.line((notch-margin - corner-radius, 0mm), relative: true),
      ..inverted-notch-path,
      curve.line((width - 3.7mm - 3 * notch-margin - notch-total, 0mm), relative: true),
      curve.quad((corner-radius, 0mm), (corner-radius, corner-radius), relative: true),
      curve.line((0mm, 0.75 * block-height - corner-radius), relative: true),
      curve.quad((0mm, corner-radius), (-corner-radius, corner-radius), relative: true),
      curve.line((-width + 3.7mm + 3 * notch-margin + notch-total, 0mm), relative: true),
      ..notch-path,
      curve.line((-notch-margin + corner-radius, 0mm), relative: true),
      curve.quad((-corner-radius, 0mm), (-corner-radius, corner-radius), relative: true),
    ),
    falls-footer: (
      curve.quad((0mm, corner-radius), (corner-radius, corner-radius), relative: true),
      curve.line((notch-margin - corner-radius, 0mm), relative: true),
      ..inverted-notch-path,
      curve.line((width - 3.7mm - 3 * notch-margin - notch-total, 0mm), relative: true),
      curve.quad((corner-radius, 0mm), (corner-radius, corner-radius), relative: true),
      curve.line((0mm, 3mm), relative: true),
      curve.quad((0mm, corner-radius), (-corner-radius, corner-radius), relative: true),
      curve.line((-width + 3.7mm + 1 * notch-margin + notch-total, 0mm), relative: true),
      ..notch-path,
      curve.line((-notch-margin + corner-radius, 0mm), relative: true),
      curve.quad((-corner-radius, 0mm), (-corner-radius, -corner-radius), relative: true),
      curve.close(),
    ),
  ).at(type, default: "anweisung")
}


#let scratch(colorschema: colors.bewegung, type: "ereignis", dx: 0mm, dy: 0mm, body, ..children) = block(
  above: 0em + if (type == "ereignis" or type == "definiere") { 6mm } else { 0mm },
  below: 0mm + if (type == "ereignis" or type == "definiere") { 6mm } else { 0mm },
)[
  #set text(font: "Helvetica Neue", if high-contrast { black } else { white }, weight: 500)
  #let content-box = align(horizon, box(
    inset: content-inset,
    height: if type == "definiere" { 1.5 * block-height } else { auto },
    [
      #context [
        #let height = measure(body).height
        #stack(dir: ltr, box(body, height: if height < block-height { 0.75 * block-height } else { auto }))
      ]
    ],
  ))
  #context [
    #let (width, height) = measure(content-box)
    #place(top + left, dx: dx, dy: dy)[
      #curve(
        fill: colorschema.fill,
        stroke: (paint: colorschema.stroke, thickness: stroke-thickness),
        ..block-path(height, width, type),
      )
    ]
  ]
  #content-box
  #v(dy, weak: true)
  #let children = children.pos()
  #if children.len() != none {
    for child in children {
      if std.type(child) == content {
        child
      }
    }
  }
]

#let ereignis(body, children) = scratch(
  colorschema: colors.ereignisse,
  type: "ereignis",
  body,
  children,
)

#let bewegung(body) = scratch(
  colorschema: colors.bewegung,
  type: "anweisung",
  dy: block-offset-y,
  body,
)

#let aussehen(body) = scratch(
  colorschema: colors.aussehen,
  type: "anweisung",
  dy: block-offset-y,
  body,
)

#let klang(body) = scratch(
  colorschema: colors.klang,
  type: "anweisung",
  dy: block-offset-y,
  body,
)

#let fühlen(body) = scratch(
  colorschema: colors.fühlen,
  type: "anweisung",
  dy: block-offset-y,
  body,
)

#let variablen(body) = scratch(
  colorschema: colors.variablen,
  type: "anweisung",
  dy: block-offset-y,
  body,
)

#let listen(body) = scratch(
  colorschema: colors.listen,
  type: "anweisung",
  dy: block-offset-y,
  body,
)

#let eigene(body, dark: false) = scratch(
  colorschema: if dark {
    (fill: colors.eigene.dark-fill, stroke: colors.eigene.stroke)
  } else {
    colors.eigene
  },
  type: "anweisung",
  dy: block-offset-y,
  body,
)

// Schleifen-Block (loop mit Körper)
#let wiederhole(anzahl: 10, loop-body: none) = block(
  above: 0em,
  below: 0mm,
)[
  #set text(font: "Helvetica Neue", if high-contrast { black } else { white }, weight: 500)
  // Oberer Teil (Kopf der Schleife)
  #let header-box = align(horizon, box(inset: content-inset, height: auto, [
    #stack(dir: ltr, spacing: 1.5mm, "wiederhole", zahl-oder-content(anzahl, colors.steuerung), "mal")
  ]))
  #context [
    #let header-box-sizes = measure(header-box)
    #let loop-body-sizes = measure(loop-body)

    // Vorerst nur Header zeichnen
    #place(top + left, dy: block-offset-y)[
      #curve(
        fill: colors.steuerung.fill,
        stroke: (paint: colors.steuerung.stroke, thickness: stroke-thickness),
        ..block-path(header-box-sizes.height, header-box-sizes.width, "loop-header"),
        curve.line((0mm, if loop-body != none { loop-body-sizes.height - corner-radius - corner-radius } else { 0mm }), relative: true),
        ..block-path(header-box-sizes.height, header-box-sizes.width, "loop-footer"),
      )
    ]
    #header-box
    #if loop-body != none {
      block(above: 0em, inset: (bottom: 3mm + 2 * corner-radius), move(dx: 2 * notch-margin, loop-body))
    }
  ]
]

// Falls-Sonst-Block (if-else mit zwei Armen)
#let falls(bedingung, dann-body: none, sonst-body: none) = block(
  above: 0em,
  below: 0mm,
)[
  #set text(font: "Helvetica Neue", if high-contrast { black } else { white }, weight: 500)
  // Header mit Bedingung
  #let header-box = align(horizon, box(inset: content-inset, [
    #stack(dir: ltr, spacing: 1.5mm, "falls", bedingung, ", dann")
  ]))

  // Sonst-Label
  #let middle-box = align(horizon, box(inset: content-inset, height: 0.75 * block-height + corner-radius, [
    #stack(dir: ltr, spacing: 1.5mm, "sonst")
  ]))

  #context [
    #let header-box-sizes = measure(header-box)
    #let middle-box-sizes = measure(middle-box)
    #let dann-body-sizes = measure(dann-body)
    #let sonst-body-sizes = measure(sonst-body)

    #let dann-height = if dann-body != none { dann-body-sizes.height } else { 0mm }
    #let sonst-height = if sonst-body != none { sonst-body-sizes.height } else { 0mm }

    // Kompletter Pfad: Header → Dann-Arm → Middle → Sonst-Arm → Footer
    #place(top + left, dy: block-offset-y)[
      #curve(
        fill: colors.steuerung.fill,
        stroke: (paint: colors.steuerung.stroke, thickness: stroke-thickness),
        ..block-path(header-box-sizes.height, header-box-sizes.width, "falls-header"),
        curve.line((0mm, dann-height - corner-radius - corner-radius), relative: true),
        ..if sonst-body != none {
          (..block-path(header-box-sizes.height, header-box-sizes.width, "falls-middle"), curve.line((0mm, sonst-height - corner-radius - corner-radius), relative: true))
        },
        ..block-path(header-box-sizes.height, header-box-sizes.width, "falls-footer"),
      )
    ]

    // Content rendern
    #header-box
    #if dann-body != none {
      block(above: 0em, below: 0em, pad(x: 2 * notch-margin, dann-body))
    }
    #if sonst-body != none {
      middle-box
    }
    #block(above: 0em, inset: (bottom: 3mm + 2 * corner-radius), pad(x: 2 * notch-margin, sonst-body))
  ]
]

#let gehe-zu(x: 0, y: 0) = bewegung(
  stack(dir: ltr, spacing: 1.5mm, "gehe zu x:", zahl-oder-content(x, colors.bewegung), "y:", zahl-oder-content(y, colors.bewegung)),
)
#let gleite-in-zu(sek: 1, x: 0, y: 0) = bewegung(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "gleite in",
    zahl-oder-content(sek, colors.bewegung),
    "Sek. zu x:",
    zahl-oder-content(x, colors.bewegung),
    "y:",
    zahl-oder-content(y, colors.bewegung),
  ),
)
#let setze-Richtung-auf(grad: 90) = bewegung(
  stack(dir: ltr, spacing: 1.5mm, "setze Richtung auf", zahl-oder-content(grad, colors.bewegung), "Grad"),
)
#let gehe(zu: "Zufallsposition") = bewegung(
  stack(dir: ltr, spacing: 1.5mm, "gehe zu", pill-reporter(
    inline: true,
    zu,
    fill: colors.bewegung.dark-fill,
    stroke: colors.bewegung.stroke + stroke-thickness,
    text-color: white,
    dropdown: true,
  )),
)
#let drehe-dich(zu: "Mauszeiger") = bewegung(
  stack(dir: ltr, spacing: 1.5mm, "drehe dich zu", pill-reporter(
    inline: true,
    zu,
    fill: colors.bewegung.dark-fill,
    stroke: colors.bewegung.stroke + stroke-thickness,
    text-color: white,
    dropdown: true,
  )),
)
#let gehe-schritt(schritt: 0) = bewegung(
  stack(dir: ltr, spacing: 1.5mm, "gehe", zahl-oder-content(schritt, colors.bewegung), "er Schritt"),
)
#let drehe-dich-um(richtung: "rechts", grad: 15) = bewegung(
  stack(dir: ltr, spacing: 1.5mm, "drehe dich", if richtung == "rechts" { sym.arrow.cw } else { sym.arrow.ccw }, "um", zahl-oder-content(grad, colors.bewegung), "Grad"),
)
#let ändere-um(richtung: "x", auf: false, schritt: 0) = bewegung(
  stack(dir: ltr, spacing: 1.5mm, if auf { "setze " } else { "ändere " } + str(richtung) + if auf { " auf" } else { " um" }, zahl-oder-content(schritt, colors.bewegung)),
)
#let ändere-x-um(schritt: 10) = ändere-um(richtung: "x", schritt: schritt)
#let setze-x-auf(x: 10) = ändere-um(richtung: "x", auf: true, schritt: x)
#let ändere-y-um(schritt: 10) = ändere-um(richtung: "y", schritt: schritt)
#let setze-y-auf(y: 10) = ändere-um(richtung: "y", auf: true, schritt: y)
#let pralle-vom-rand-ab() = bewegung(
  stack(dir: ltr, spacing: 1.5mm, "pralle vom Rand ab"),
)

#let gleite-in(sek: 1, zu: "Zufallsposition") = bewegung(
  stack(dir: ltr, spacing: 1.5mm, "gleite in", zahl-oder-content(sek, colors.bewegung), "Sek. zu", pill-reporter(
    inline: true,
    zu,
    fill: colors.bewegung.dark-fill,
    stroke: colors.bewegung.stroke + stroke-thickness,
    text-color: white,
    dropdown: true,
  )),
)

// Aussehen-Blöcke
#let sage(text: "Hallo!", sekunden: none) = aussehen(
  if sekunden == none {
    stack(dir: ltr, spacing: 1.5mm, "sage", zahl-oder-content(text, colors.aussehen))
  } else {
    stack(dir: ltr, spacing: 1.5mm, "sage", zahl-oder-content(text, colors.aussehen), "für", zahl-oder-content(sekunden, colors.aussehen), "Sekunden")
  },
)

#let denke(text: "Hmm...", sekunden: none) = aussehen(
  if sekunden == none {
    stack(dir: ltr, spacing: 1.5mm, "denke", zahl-oder-content(text, colors.aussehen))
  } else {
    stack(dir: ltr, spacing: 1.5mm, "denke", zahl-oder-content(text, colors.aussehen), "für", zahl-oder-content(sekunden, colors.aussehen), "Sekunden")
  },
)

#let wechsle-zu-kostüm(kostüm: "Kostüm2") = aussehen(
  stack(dir: ltr, spacing: 1.5mm, "wechsle zu Kostüm", pill-reporter(
    inline: true,
    kostüm,
    fill: colors.aussehen.dark-fill,
    stroke: colors.aussehen.stroke + stroke-thickness,
    text-color: white,
    dropdown: true,
  )),
)

#let wechsle-zum-nächsten-kostüm() = aussehen(
  stack(dir: ltr, spacing: 1.5mm, "wechsle zum nächsten Kostüm"),
)

#let wechsle-zu-bühnenbild(bild: "Hintergrund1") = aussehen(
  stack(dir: ltr, spacing: 1.5mm, "wechsle zu Bühnenbild", pill-reporter(
    inline: true,
    fill: colors.aussehen.dark-fill,
    stroke: colors.aussehen.stroke + stroke-thickness,
    text-color: white,
    bild,
    dropdown: true,
  )),
)

#let wechsle-zum-nächsten-bühnenbild() = aussehen(
  stack(dir: ltr, spacing: 1.5mm, "wechsle zum nächsten Bühnenbild"),
)

#let ändere-größe-um(wert: 10) = aussehen(
  stack(dir: ltr, spacing: 1.5mm, "ändere Größe um", zahl-oder-content(wert, colors.aussehen)),
)

#let setze-größe-auf(wert: 100) = aussehen(
  stack(dir: ltr, spacing: 1.5mm, "setze Größe auf", zahl-oder-content(wert, colors.aussehen)),
)

#let ändere-effekt(effekt: "Farbe", um: 25) = aussehen(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "ändere Effekt",
    pill-rect(
      effekt,
      fill: colors.aussehen.fill,
      stroke: colors.aussehen.stroke + stroke-thickness,
      text-color: white,
      dropdown: true,
    ),
    "um",
    zahl-oder-content(um, colors.aussehen),
  ),
)

#let setze-effekt(effekt: "Farbe", auf: 0) = aussehen(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "setze Effekt",
    pill-rect(
      effekt,
      fill: colors.aussehen.fill,
      stroke: colors.aussehen.stroke + stroke-thickness,
      text-color: white,
      dropdown: true,
    ),
    "auf",
    zahl-oder-content(auf, colors.aussehen),
  ),
)

#let schalte-grafikeffekte-aus() = aussehen(
  stack(dir: ltr, spacing: 1.5mm, "schalte Grafikeffekte aus"),
)

#let zeige-dich() = aussehen(
  stack(dir: ltr, spacing: 1.5mm, "zeige dich"),
)

#let verstecke-dich() = aussehen(
  stack(dir: ltr, spacing: 1.5mm, "verstecke dich"),
)

// Klang-Blöcke
#let spiele-klang(sound: "Meow", ganz: true) = klang(
  if ganz {
    stack(
      dir: ltr,
      spacing: 1.5mm,
      "spiele Klang",
      pill-reporter(
        inline: true,
        sound,
        fill: colors.klang.dark-fill,
        stroke: colors.klang.stroke + stroke-thickness,
        text-color: white,
        dropdown: true,
      ),
      "ganz",
    )
  } else {
    stack(dir: ltr, spacing: 1.5mm, "spiele Klang", pill-reporter(
      inline: true,
      sound,
      fill: colors.klang.dark-fill,
      stroke: colors.klang.stroke + stroke-thickness,
      text-color: white,
      dropdown: true,
    ))
  },
)

#let stoppe-alle-klänge() = klang(
  stack(dir: ltr, spacing: 1.5mm, "stoppe alle Klänge"),
)

#let ändere-klang-effekt(effekt: "Höhe", um: 10) = klang(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "ändere Effekt",
    pill-rect(
      effekt,
      fill: colors.klang.fill,
      stroke: colors.klang.stroke + stroke-thickness,
      text-color: white,
      dropdown: true,
    ),
    "um",
    zahl-oder-content(um, colors.klang),
  ),
)

#let setze-klang-effekt(effekt: "Höhe", auf: 100) = klang(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "setze Effekt",
    pill-rect(
      effekt,
      fill: colors.klang.fill,
      stroke: colors.klang.stroke + stroke-thickness,
      text-color: white,
      dropdown: true,
    ),
    "auf",
    zahl-oder-content(auf, colors.klang),
  ),
)

#let schalte-klangeffekte-aus() = klang(
  stack(dir: ltr, spacing: 1.5mm, "schalte Klangeffekte aus"),
)

#let ändere-lautstärke-um(wert: -10) = klang(
  stack(dir: ltr, spacing: 1.5mm, "ändere Lautstärke um", zahl-oder-content(wert, colors.klang)),
)

#let setze-lautstärke-auf(wert: 100) = klang(
  stack(dir: ltr, spacing: 1.5mm, "setze Lautstärke auf", zahl-oder-content(wert, colors.klang), "%"),
)

// Reporter-Blöcke (Werte)
// Allgemeine Reporter-Funktion für alle Kategorien
#let reporter(colorschema: colors.aussehen, body, dropdown-content: none) = pill-reporter(
  fill: colorschema.fill,
  stroke: colorschema.stroke + stroke-thickness,
  text-color: white,
  if dropdown-content != none {
    pill-round(fill: none, stroke: none, text-color: white, inset: (x: 0mm), stack(dir: ltr, spacing: pill-spacing, box(inset: (left: pill-inset-x), body), pill-reporter(
      dropdown-content,
      fill: colorschema.dark-fill,
      stroke: colorschema.stroke + stroke-thickness,
      text-color: white,
      dropdown: true,
      inline: true,
    )))
  } else {
    pill-round(body, fill: none, stroke: none, text-color: white)
  },
)

// Bewegungs-Reporter
#let bewegung-reporter(body, dropdown-content: none) = reporter(
  colorschema: colors.bewegung,
  body,
  dropdown-content: dropdown-content,
)

// Aussehen-Reporter
#let aussehen-reporter(body, dropdown-content: none) = reporter(
  colorschema: colors.aussehen,
  body,
  dropdown-content: dropdown-content,
)

// Klang-Reporter
#let klang-reporter(body, dropdown-content: none) = reporter(
  colorschema: colors.klang,
  body,
  dropdown-content: dropdown-content,
)

// Fühlen-Reporter
#let fühlen-reporter(body, dropdown-content: none) = reporter(
  colorschema: colors.fühlen,
  body,
  dropdown-content: dropdown-content,
)

// Variablen-Reporter
#let variablen-reporter(body, dropdown-content: none) = reporter(
  colorschema: colors.variablen,
  body,
  dropdown-content: dropdown-content,
)

// Listen-Reporter
#let listen-reporter(body, dropdown-content: none) = reporter(
  colorschema: colors.listen,
  body,
  dropdown-content: dropdown-content,
)

// Eigene-Reporter (für Platzhalter/Reporter in eigenen Blöcken)
#let eigene-reporter(body, dropdown-content: none) = reporter(
  colorschema: colors.eigene,
  body,
  dropdown-content: dropdown-content,
)

// Bedingung (Diamant-Form für boolesche Werte)
#let bedingung(colorschema: colors.bewegung, type: "bedingung", body, nested: false) = box([
  // nested kann bool (beide Seiten gleich) oder (left, right) array sein
  #let nested-type = std.type(nested)
  #let (nested-left, nested-right) = if nested-type == array {
    (nested.at(0), nested.at(1))
  } else {
    (nested, nested)
  }
  #let x-inset-left = if nested-left { -0.3 } else { 1.0 }
  #let x-inset-right = if nested-right { -0.3 } else { 1.0 }
  #let content-box = box(inset: (left: pill-inset-x * x-inset-left, right: pill-inset-x * x-inset-right, y: pill-inset-y), align(horizon, [
    #grid(
      columns: (body.len() * 2 + 1) * (auto,),
      column-gutter: 1fr,
      align: center + horizon,
      h(pill-spacing),
      ..body.map(x => { (x, h(0.25em)) }).flatten(),
      h(pill-spacing),
    )
  ]))

  #context [
    #let (width, height) = measure(content-box, height: auto)
    #place(bottom + left)[
      #curve(
        fill: colorschema.fill,
        stroke: (paint: colorschema.stroke, thickness: stroke-thickness),
        ..block-path(height, width, type),
      )
    ]
    #box(width: width + 0.5 * height)[#content-box]
  ]
])

// Spezifische Aussehen-Reporter
//#let kostüm(eigenschaft: "Nummer") = aussehen-reporter("Kostüm", dropdown-content: eigenschaft)
#let kostüm(eigenschaft: "Nummer") = aussehen-reporter(stack(
  dir: ltr,
  spacing: pill-spacing,
  "Kostüm",
  pill-rect(
    eigenschaft,
    dropdown: true,
    fill: colors.aussehen.fill,
    text-color: white,
    stroke: colors.aussehen.stroke + stroke-thickness,
  ),
))

#let bühnenbild(eigenschaft: "Nummer") = aussehen-reporter(stack(
  dir: ltr,
  spacing: pill-spacing,
  "Bühnenbild",
  pill-rect(
    eigenschaft,
    dropdown: true,
    fill: colors.aussehen.fill,
    text-color: white,
    stroke: colors.aussehen.stroke + stroke-thickness,
  ),
))

#let größe() = aussehen-reporter("Größe")

// Spezifische Klang-Reporter
#let lautstärke() = klang-reporter("Lautstärke")

// Fühlen-Blöcke (Befehlsblöcke)
#let frage(text: "Wie heißt du?") = fühlen(
  stack(dir: ltr, spacing: 1.5mm, "frage", zahl-oder-content(text, colors.fühlen), "und warte"),
)

#let setze-ziehbarkeit-auf(modus: "ziehbar") = fühlen(
  stack(dir: ltr, spacing: 1.5mm, "setze Ziehbarkeit auf", pill-rect(
    modus,
    fill: colors.fühlen.dark-fill,
    stroke: colors.fühlen.stroke + stroke-thickness,
    text-color: white,
    dropdown: true,
  )),
)

#let setze-stoppuhr-zurück() = fühlen(
  stack(dir: ltr, spacing: 1.5mm, "setze Stoppuhr zurück"),
)

// Spezifische Fühlen-Reporter
#let entfernung-von(objekt: "Mauszeiger") = fühlen-reporter("Entfernung von", dropdown-content: objekt)

#let antwort() = fühlen-reporter("Antwort")

#let taste-gedrückt(taste: "Leertaste", nested: false) = bedingung(
  colorschema: colors.fühlen,
  type: "bedingung",
  (
    "Taste",
    pill-round(fill: none, stroke: none, inset: (x: 0mm), pill-reporter(
      taste,
      inline: true,
      dropdown: true,
      fill: colors.fühlen.dark-fill,
      text-color: white,
      stroke: colors.fühlen.stroke + stroke-thickness,
    )),
    "gedrückt",
  ),
  nested: nested,
)

#let maustaste-gedrückt(nested: false) = bedingung(
  colorschema: colors.fühlen,
  type: "bedingung",
  (pill-round("Maustaste gedrückt?", fill: none, stroke: none, inset: 0mm, text-color: white),),
  nested: nested,
)

#let maus-x-position() = fühlen-reporter("Maus x-Position")

#let maus-y-position() = fühlen-reporter("Maus y-Position")

#let stoppuhr() = fühlen-reporter("Stoppuhr")

#let von-bühne(eigenschaft: "Bühnenbildnummer", objekt: "Bühne") = fühlen-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing,
    pill-rect(
      eigenschaft,
      dropdown: true,
      fill: colors.fühlen.fill,
      text-color: white,
      stroke: colors.fühlen.stroke + stroke-thickness,
    ),
    "von",
    pill-rect(
      objekt,
      dropdown: true,
      fill: colors.fühlen.fill,
      text-color: white,
      stroke: colors.fühlen.stroke + stroke-thickness,
    ),
  ),
)

#let zeit(einheit: "Jahr") = fühlen-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing,
    pill-rect(
      einheit,
      dropdown: true,
      fill: colors.fühlen.fill,
      text-color: white,
      stroke: colors.fühlen.stroke + stroke-thickness,
    ),
    "im Moment",
  ),
)

#let tage-seit-2000() = fühlen-reporter("Tage seit 2000")

#let benutzername() = fühlen-reporter("Benutzername")

#let wird-mauszeiger-berührt(nested: false) = bedingung(
  colorschema: colors.fühlen,
  type: "bedingung",
  (
    pill-round("wird", text-color: white, fill: none, stroke: none, inset: (right: 0mm)),
    pill-round(fill: none, stroke: none, inset: (x: 0mm), pill-reporter(
      "Mauszeiger",
      inline: true,
      dropdown: true,
      fill: colors.fühlen.dark-fill,
      text-color: white,
      stroke: colors.fühlen.stroke + stroke-thickness,
    )),
    pill-round("berührt", text-color: white, fill: none, stroke: none, inset: (left: 0mm)),
  ),
  nested: nested,
)

#let wird-farbe-berührt(color: rgb("#36B7CE"), nested: false) = bedingung(
  colorschema: colors.fühlen,
  type: "bedingung",
  ("wird Farbe", pill-color("         ", fill: color), "berührt?"),
  nested: nested,
)

#let farbe-berührt(color: (rgb("#83FEF3"), rgb("#CB6622")), nested: false) = bedingung(
  colorschema: colors.fühlen,
  type: "bedingung",
  (
    "Farbe",
    pill-color("         ", fill: color.at(0)),
    "berührt",
    pill-color("         ", fill: color.at(1)),
    "?",
  ),
  nested: nested,
)

// Variablen-Blöcke
#let variable(name) = variablen-reporter(name)

#let setze-variable-auf(name: "my variable", wert: 0) = variablen(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "setze",
    pill-rect(
      name,
      fill: colors.variablen.fill,
      stroke: colors.variablen.stroke + stroke-thickness,
      text-color: white,
      dropdown: true,
    ),
    "auf",
    zahl-oder-content(wert, colors.variablen),
  ),
)

#let ändere-variable-um(name: "my variable", wert: 1) = variablen(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "ändere",
    pill-rect(
      name,
      fill: colors.variablen.fill,
      stroke: colors.variablen.stroke + stroke-thickness,
      text-color: white,
      dropdown: true,
    ),
    "um",
    zahl-oder-content(wert, colors.variablen),
  ),
)

#let zeige-variable(name: "my variable") = variablen(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "zeige Variable",
    pill-rect(
      name,
      fill: colors.variablen.fill,
      stroke: colors.variablen.stroke + stroke-thickness,
      text-color: white,
      dropdown: true,
    ),
  ),
)

#let verstecke-variable(name: "my variable") = variablen(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "verstecke Variable",
    pill-rect(
      name,
      fill: colors.variablen.fill,
      stroke: colors.variablen.stroke + stroke-thickness,
      text-color: white,
      dropdown: true,
    ),
  ),
)

// Listen-Blöcke (eigene Farbe)
#let füge-zu-hinzu(wert: "Ding", liste: "Test") = listen(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "füge",
    zahl-oder-content(wert, colors.listen),
    "zu",
    pill-rect(
      liste,
      dropdown: true,
      fill: colors.listen.fill,
      text-color: white,
      stroke: colors.listen.stroke + stroke-thickness,
    ),
    "hinzu",
  ),
)

#let lösche-aus(index: 1, liste: "Test") = listen(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "lösche",
    zahl-oder-content(index, colors.listen),
    "aus",
    pill-rect(
      liste,
      dropdown: true,
      fill: colors.listen.fill,
      text-color: white,
      stroke: colors.listen.stroke + stroke-thickness,
    ),
  ),
)

#let lösche-alles-aus(liste: "Test") = listen(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "lösche alles aus",
    pill-rect(
      liste,
      dropdown: true,
      fill: colors.listen.fill,
      text-color: white,
      stroke: colors.listen.stroke + stroke-thickness,
    ),
  ),
)

#let füge-bei-in-ein(wert: "Ding", index: 1, liste: "Test") = listen(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "füge",
    zahl-oder-content(wert, colors.listen),
    "bei",
    zahl-oder-content(index, colors.listen),
    "in",
    pill-rect(
      liste,
      dropdown: true,
      fill: colors.listen.fill,
      text-color: white,
      stroke: colors.listen.stroke + stroke-thickness,
    ),
    "ein",
  ),
)

#let ersetze-element-von-durch(index: 1, liste: "Test", wert: "Ding") = listen(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "ersetze Element",
    zahl-oder-content(index, colors.listen),
    "von",
    pill-rect(
      liste,
      dropdown: true,
      fill: colors.listen.fill,
      text-color: white,
      stroke: colors.listen.stroke + stroke-thickness,
    ),
    "durch",
    zahl-oder-content(wert, colors.listen),
  ),
)

#let element-von(index: 1, liste: "Test") = listen-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing,
    "Element",
    zahl-oder-content(index, colors.listen),
    "von",
    pill-rect(
      liste,
      dropdown: true,
      fill: colors.listen.fill,
      text-color: white,
      stroke: colors.listen.stroke + stroke-thickness,
    ),
  ),
)

#let nummer-von-in(wert: "Ding", liste: "Test") = listen-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing,
    "Nummer von",
    zahl-oder-content(wert, colors.listen),
    "in",
    pill-rect(
      liste,
      dropdown: true,
      fill: colors.listen.fill,
      text-color: white,
      stroke: colors.listen.stroke + stroke-thickness,
    ),
  ),
)

#let länge-von-liste(liste) = listen-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing,
    "Länge von",
    pill-rect(
      liste,
      dropdown: true,
      fill: colors.listen.fill,
      text-color: white,
      stroke: colors.listen.stroke + stroke-thickness,
    ),
  ),
)

#let liste-enthält(liste: "Test", wert: "Ding", nested: false) = bedingung(
  colorschema: colors.listen,
  type: "bedingung",
  (
    pill-rect(
      liste,
      dropdown: true,
      fill: colors.listen.fill,
      text-color: white,
      stroke: colors.listen.stroke + stroke-thickness,
    ),
    "enthält",
    zahl-oder-content(wert, colors.listen),
    "?",
  ),
  nested: nested,
)

#let zeige-liste(liste: "Test") = listen(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "zeige Liste",
    pill-rect(
      liste,
      dropdown: true,
      fill: colors.listen.fill,
      text-color: white,
      stroke: colors.listen.stroke + stroke-thickness,
    ),
  ),
)

#let verstecke-liste(liste: "Test") = listen(
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "verstecke Liste",
    pill-rect(
      liste,
      dropdown: true,
      fill: colors.listen.fill,
      text-color: white,
      stroke: colors.listen.stroke + stroke-thickness,
    ),
  ),
)

// Eigene Blöcke
// Weißer Argument-Platzhalter für eigene Blöcke
#let eigene-eingabe(text) = pill-round(text, stroke: colors.eigene.stroke + stroke-thickness)

// Eigener Anweisungsblock: Übergib gemischte Inhalte (Text, eigene-eingabe(...), Reporter ...)
#let eigener-block(..body) = {
  let items = body.pos()
  return (dark: true, ..values) => eigene(dark: dark, {
    let values = values.pos()
    stack(
      dir: ltr,
      spacing: 1.5mm,
      ..if values.len() == 0 {
        for item in items {
          if std.type(item) == str {
            (item,)
          } else {
            (pill-round(stroke: colors.eigene.stroke, fill: colors.eigene.fill, text-color: white, str("number or text")),)
          }
        }
      } else {
        let key = 0
        for item in items {
          if std.type(item) == str {
            (item,)
          } else {
            (zahl-oder-content(values.at(calc.rem(key, values.len())), colors.eigene),)
            key += 1
          }
        }
      },
    )
  })
}

// "Definiere"-Block: Kopf der Definition mit innerem Label (Block-Signatur)
#let definiere(label, ..children) = scratch(
  colorschema: colors.eigene,
  type: "definiere",
  dy: 2.5 * corner-radius,
  stack(
    dir: ltr,
    spacing: 1.5mm,
    "Definiere",
    label(dark: true),
  ),
  ..children,
)

// Operatoren-Blöcke
#let plus(arg1, arg2) = pill-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing * 0.5,
    zahl-oder-content(arg1, colors.operatoren),
    "+",
    zahl-oder-content(arg2, colors.operatoren),
  ),
  fill: colors.operatoren.fill,
  text-color: white,
  stroke: colors.operatoren.stroke + stroke-thickness,
)

#let minus(arg1, arg2) = pill-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing * 0.5,
    zahl-oder-content(arg1, colors.operatoren),
    "−",
    zahl-oder-content(arg2, colors.operatoren),
  ),
  fill: colors.operatoren.fill,
  text-color: white,
  stroke: colors.operatoren.stroke + stroke-thickness,
)

#let mal(arg1, arg2) = pill-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing * 0.5,
    zahl-oder-content(arg1, colors.operatoren),
    "∗",
    zahl-oder-content(arg2, colors.operatoren),
  ),
  fill: colors.operatoren.fill,
  text-color: white,
  stroke: colors.operatoren.stroke + stroke-thickness,
)

#let geteilt(arg1, arg2) = pill-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing * 0.5,
    zahl-oder-content(arg1, colors.operatoren),
    "/",
    zahl-oder-content(arg2, colors.operatoren),
  ),
  fill: colors.operatoren.fill,
  text-color: white,
  stroke: colors.operatoren.stroke + stroke-thickness,
)

#let zufallszahl(von: 1, bis: 10) = pill-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing * 0.5,
    "Zufallszahl von",
    zahl-oder-content(von, colors.operatoren),
    "bis",
    zahl-oder-content(bis, colors.operatoren),
  ),
  fill: colors.operatoren.fill,
  text-color: white,
  stroke: colors.operatoren.stroke + stroke-thickness,
)

#let größer-als(arg1, arg2, nested: false) = bedingung(
  colorschema: colors.operatoren,
  type: "bedingung",
  (zahl-oder-content(arg1, colors.operatoren), ">", zahl-oder-content(arg2, colors.operatoren)),
  nested: nested,
)

#let kleiner-als(arg1, arg2, nested: false) = bedingung(
  colorschema: colors.operatoren,
  type: "bedingung",
  (zahl-oder-content(arg1, colors.operatoren), "<", zahl-oder-content(arg2, colors.operatoren)),
  nested: nested,
)

#let gleich(arg1, arg2, nested: false) = bedingung(
  colorschema: colors.operatoren,
  type: "bedingung",
  (zahl-oder-content(arg1, colors.operatoren), "=", zahl-oder-content(arg2, colors.operatoren)),
  nested: nested,
)

#let und(arg1, arg2, nested: (false, false)) = bedingung(
  colorschema: colors.operatoren,
  type: "bedingung",
  (arg1, "und", arg2),
  nested: nested,
)

#let oder(arg1, arg2, nested: (false, false)) = bedingung(
  colorschema: colors.operatoren,
  type: "bedingung",
  (arg1, "oder", arg2),
  nested: nested,
)

#let nicht(arg1, nested: false) = bedingung(
  colorschema: colors.operatoren,
  type: "bedingung",
  ("nicht", arg1),
  nested: (false, nested), // Links immer false (nur "nicht"), rechts variabel
)

#let verbinde(text1, text2) = pill-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing * 0.5,
    "",
    "verbinde",
    zahl-oder-content(text1, colors.operatoren),
    "und",
    zahl-oder-content(text2, colors.operatoren),
  ),
  fill: colors.operatoren.fill,
  text-color: white,
  stroke: colors.operatoren.stroke + stroke-thickness,
)

#let zeichen(position: 1, von: "Apfel") = pill-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing * 0.5,
    "Zeichen",
    zahl-oder-content(position, colors.operatoren),
    "von",
    zahl-oder-content(von, colors.operatoren),
  ),
  fill: colors.operatoren.fill,
  text-color: white,
  stroke: colors.operatoren.stroke + stroke-thickness,
)

#let länge-von(text: "Apfel") = pill-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing * 0.5,
    "Länge von",
    zahl-oder-content(text, colors.operatoren),
  ),
  fill: colors.operatoren.fill,
  text-color: white,
  stroke: colors.operatoren.stroke + stroke-thickness,
)

#let enthält(text: "Apfel", zeichen: "a", nested: false) = bedingung(
  colorschema: colors.operatoren,
  type: "bedingung",
  (zahl-oder-content(text, colors.operatoren), "enthält", zahl-oder-content(zeichen, colors.operatoren), "?"),
  nested: nested,
)

#let modulo(arg1, arg2) = pill-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing * 0.5,
    zahl-oder-content(arg1, colors.operatoren),
    "mod",
    zahl-oder-content(arg2, colors.operatoren),
  ),
  fill: colors.operatoren.fill,
  text-color: white,
  stroke: colors.operatoren.stroke + stroke-thickness,
)

#let gerundet(zahl) = pill-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing * 0.5,
    zahl-oder-content(zahl, colors.operatoren),
    "gerundet",
  ),
  fill: colors.operatoren.fill,
  text-color: white,
  stroke: colors.operatoren.stroke + stroke-thickness,
)

#let betrag-von(operation: "Betrag", zahl) = pill-reporter(
  stack(
    dir: ltr,
    spacing: pill-spacing * 0.5,
    pill-rect(
      operation,
      dropdown: true,
      fill: colors.operatoren.fill,
      text-color: white,
      stroke: colors.operatoren.stroke + stroke-thickness,
    ),
    "von",
    zahl-oder-content(zahl, colors.operatoren),
  ),
  fill: colors.operatoren.fill,
  text-color: white,
  stroke: colors.operatoren.stroke + stroke-thickness,
)
