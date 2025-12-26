// core.typ — Core-Renderer mit vollständiger Lokalisierung
// Importiert scratch.typ für Rendering-Funktionen und nutzt Registry aus registry.typ

#import "scratch.typ": *
#import "registry.typ": REGISTRY

// Hilfsfunktion: Hole lokalisierten Text aus Registry
#let get-template(id, lang-code) = {
  if REGISTRY.keys().contains(id) {
    let info = REGISTRY.at(id)
    if lang-code in info {
      info.at(lang-code)
    } else if "de" in info {
      info.at("de")
    } else {
      id
    }
  } else {
    id
  }
}

// Hilfsfunktion: Erstelle Pills für verschiedene Argument-Typen
#let make-pill(key, value, colors, shape: none) = {
  let stroke-thickness = get-stroke-from-options(scratch-block-options.get())
  
  // Dropdown-Felder (typischerweise Strings in Rechteck-Pills)
  let dropdown-keys = ("key", "scene", "element", "message1" , "style", "effect", "property", "layer", "direction", "option", "mode", "timeunit", "operator", "variable", "list")

  let dropdown-keys2 = ("message2", "to", "towards", "costume", "backdrop", "sound", "clone", "object", "key2","component")
  
  // inline: true für Reporter/Bedingungen, inline: false für Stack-Blöcke
  let use-inline = shape in ("reporter", "boolean")

  if key in dropdown-keys and type(value) == str {
    pill-rect(value, fill: colors.primary, stroke: colors.tertiary + stroke-thickness, dropdown: true, inline: use-inline)
  } else if key in dropdown-keys2 and type(value) == str {
    pill-reporter(value, fill: colors.secondary, stroke: colors.tertiary + stroke-thickness, dropdown: true, inline: use-inline)
  } else if key in ("color", "color1", "color2") {
    pill-color("        ", fill: value)
  } else {
    zahl-oder-content(value, colors)
  }
  
}

// Hilfsfunktion: Ersetze Platzhalter in Templates - UNIVERSELLE VERSION
#let fill-template(template, args, colors, shape: none) = {
  // Icon-Definitionen aus scratch.typ
  let flag-icon = box(baseline: 20%,inset:(bottom:2.5pt), image(icons.green-flag, width: 1.3em, height: 1.4em))
  let arrow-right = box(baseline: 20%, image(icons.rotate-right, width: 1.5em, height: 1.5em))
  let arrow-left = box(baseline: 20%, image(icons.rotate-left, width: 1.5em, height: 1.5em))
  let pen = h(.1em)+box(baseline: 20%, image(icons.pen, width: 2.2em, height: 2.2em))+h(.5em)+box(baseline:20%,line(angle:90deg,length: 2em,stroke:.6pt+rgb("#0da57a")))
  
  // Einfache Templates ohne Platzhalter
  if not template.contains("{") {
    // Einfacher Text ohne Platzhalter - füge Inset für Reporter hinzu
    if shape in ("reporter", "boolean") and template.len() > 0 {
      return box(inset: (left: 2mm, right: 2mm), template)
    }
    return template
  }
  
  // Verwende split() statt manueller Iteration
  let parts = ()
  let remaining = template
  let is-first-text = true
  
  while remaining.contains("{") {
    // Finde Position von {
    let before-split = remaining.split("{")
    if before-split.len() < 2 {
      // Kein { gefunden
      if remaining.len() > 0 {
        let text = remaining
        // Letzter Text-Teil - nur right Inset
        if shape in ("reporter", "boolean") and text.len() > 0 {
          parts.push(box(inset: (right: 2mm), text))
        } else {
          parts.push(text)
        }
      }
      break
    }
    
    // Text vor {
    if before-split.at(0).len() > 0 {
      let text = before-split.at(0)
      // Erster Text-Teil - nur left Inset
      if shape in ("reporter", "boolean") and text.len() > 0 {
        if is-first-text {
          parts.push(box(inset: (left: 2mm), text))
          is-first-text = false
        } else {
          parts.push(text)
        }
      } else {
        parts.push(text)
        is-first-text = false
      }
    } else {
      is-first-text = false
    }
    
    // Rest nach {
    let after-open = before-split.slice(1).join("{")
    
    // Finde }
    let inside-split = after-open.split("}")
    if inside-split.len() < 2 {
      // Kein } gefunden - als Text behandeln
      parts.push("{" + after-open)
      break
    }
    
    // Platzhalter-Name
    let placeholder = inside-split.at(0)
    
    // Ersetze bekannte Platzhalter
    if placeholder == "flag" {
      parts.push(flag-icon)
    } else if placeholder == "arrow-right" {
      parts.push(arrow-right)
    } else if placeholder == "arrow-left" {
      parts.push(arrow-left)
    } else if placeholder == "pen" {
      parts.push(pen)
    } else if placeholder in args {
      parts.push(make-pill(placeholder, args.at(placeholder), colors, shape: shape))
    } else {
      // Unbekannter Platzhalter - als Text behalten
      parts.push("{" + placeholder + "}")
    }
    
    // Weiter mit dem Rest
    remaining = inside-split.slice(1).join("}")
  }
  
  // Letzten Rest hinzufügen
  if remaining.len() > 0 and not remaining.contains("{") {
    // Letzter Text-Teil - nur right Inset
    if shape in ("reporter", "boolean") and remaining.len() > 0 {
      parts.push(box(inset: (right: 2mm), remaining))
    } else {
      parts.push(remaining)
    }
  }
  
  // Wenn keine Parts, gib Template zurück
  if parts.len() == 0 {
    return template
  }
  
  // Wenn nur ein Part, gib direkt zurück
  if parts.len() == 1 {
    return parts.at(0)
  }
  
  // Baue Stack aus allen Parts
  return stack(dir: ltr, spacing: 1.5mm, ..parts)
}

// Hilfsfunktion: Hole Block-Info aus Registry
#let get-block-info(id) = {
  if REGISTRY.keys().contains(id) {
    REGISTRY.at(id)
  } else {
    (shape: "stack", category: "control")
  }
}

// Zentrale Block-Rendering-Funktion mit Lokalisierung
// WICHTIG: Diese Funktion sollte NUR für einfache Template-basierte Blöcke verwendet werden
// Komplexe Blöcke (Operatoren, Reporter, C-Blöcke) nutzen die Original-Funktionen aus scratch.typ
#let render-block(id, args: (:), lang-code: str, body: [], else-body: none) = context {
  // Normalisiere Sprache
  let l = if lang-code == "auto" { "de" } else { lang-code }
  
  // Hole Optionen und Farben
  let options = scratch-block-options.get()
  let colors = get-colors-from-options(options)
  let stroke-thickness = get-stroke-from-options(options)
  
  // Hole Block-Info aus Registry
  let info = get-block-info(id)
  let category = info.at("category", default: "control")
  let shape = info.at("shape", default: "stack")
  let template = get-template(id, l)
  
  // Wähle die richtige Farbe basierend auf Kategorie
  let color = if category == "ereignisse" { 
    colors.ereignisse 
  } else if category == "bewegung" { 
    colors.bewegung 
  } else if category == "aussehen" {
    colors.aussehen
  } else if category == "klang" {
    colors.klang
  } else if category == "steuerung" {
    colors.steuerung
  } else if category == "fühlen" {
    colors.fühlen
  } else if category == "operatoren" {
    colors.operatoren
  } else if category == "variablen" {
    colors.variablen
  } else if category == "listen" {
    colors.listen
  } else if category == "eigene" {
    colors.eigene
  } else if category == "malstift" {
    colors.malstift
  } else { 
    colors.steuerung 
  }
  
  // Fülle Template mit Argumenten
  let content = fill-template(template, args, color, shape: shape)

  // Spezialfälle: Nutze Original-Funktionen aus scratch.typ
  
  // Control blocks mit speziellen Formen
  if id == "control.if_else" {
    let condition = if "condition" in args { args.condition } else { bedingung(colorschema: colors.operatoren, []) }
    return falls(condition, dann: body, sonst: else-body,lang-code:l)
  } else if id == "control.repeat" {
    let times = if "times" in args { args.times } else { 10 }
    return wiederhole(anzahl: times, body: body,lang-code:l)
  } else if id == "control.forever" {
    return wiederhole-fortlaufend(body,lang-code:l)
  } else if id == "control.if" {
    let condition = if "condition" in args { args.condition } else { bedingung(colorschema: colors.operatoren, []) }
    return falls(condition, dann: body,lang-code:l)
  } else if id == "control.repeat_until" {
    let condition = if "condition" in args { args.condition } else { bedingung(colorschema: colors.operatoren, []) }
    return wiederhole-bis(condition, body: body,lang-code:l)
  } else if id == "control.start_as_clone" {
    return wenn-ich-als-klon-entstehe(body,lang-code:l)
  }
  
  // Custom block definition
  if id == "custom.define" {
    let label-func = args.at("label", default: none)
    if label-func != none {
      return definiere(label-func, body)
    } else {
      return definiere(content, body)
    }
  }

  // Rendere basierend auf Shape
  if shape == "hat" {
    // Event blocks
    ereignis(content, body)
  } else if shape == "reporter" {
    // Value reporters (rounded pills)
    if category == "bewegung" {
      bewegung-reporter(content)
    } else if category == "aussehen" {
      aussehen-reporter(content)
    } else if category == "klang" {
      klang-reporter(content)
    } else if category == "fühlen" {
      fühlen-reporter(content)
    } else if category == "operatoren" {
      pill-reporter(
        content,
        fill: color.primary,
        stroke: color.tertiary + stroke-thickness,
      )
    } else if category == "variablen" {
      variablen-reporter(content)
    } else if category == "listen" {
      listen-reporter(content)
    } else if category == "malstift" {
      malstift-reporter(content)
    } else if category == "eigene" {
      eigene-reporter(content)
    } else {
      pill-reporter(
        content,
        fill: color.primary,
        stroke: color.tertiary + stroke-thickness,
      )
    }
  } else if shape == "input" {
    // Custom input (variable reporter)
    variablen-reporter(args.at("text", default: ""))

  } else if shape == "boolean" {
    // Boolean reporters (diamond shape)
    bedingung(colorschema: color, content)
  } else {
    // Stack blocks (commands)
    if category == "bewegung" {
      bewegung(content)
    } else if category == "aussehen" {
      aussehen(content)
    } else if category == "klang" {
      klang(content)
    } else if category == "steuerung" {
      steuerung(content)
    } else if category == "fühlen" {
      fühlen(content)
    } else if category == "ereignisse" {
      // Event-Kategorie als Stack-Block (z.B. broadcast)
      scratch-block(
        colorschema: color,
        type: "anweisung",
        dy: block-offset-y,
        content,
      )
    } else if category == "variablen" {
      variablen(content)
    } else if category == "listen" {
      listen(content)
    } else if category == "malstift" {
      malstift(content)
    } else if category == "eigene" {
      eigene(content)
    } else {
      steuerung(content)
    }
  }
}

// Öffentliche API für Alias-Dateien
// Diese Funktion wird von lang/de.typ, lang/en.typ etc. aufgerufen
#let block(id, args: (:), lang-code: "auto", body: [], else-body: none) = {
  render-block(id, args: args, lang-code: lang-code, body: body, else-body: else-body)
}
