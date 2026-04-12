// core.typ — Scratch block renderer with full localisation
// Imports mod.typ for rendering functions, uses the nested registry
// and translation files from lang/translations/.

#import "mod.typ": *
#import "registry.typ": REGISTRY
#import "category-map.typ": CATEGORY_MAP
#let _TRANS_DE = toml("lang/translations/de.toml")
#let _TRANS_EN = toml("lang/translations/en.toml")
#let _TRANS_FR = toml("lang/translations/fr.toml")

// Internal helper: split block ID into group key and block key
#let _split-id(id) = {
  let parts = id.split(".")
  let group = parts.first()
  let key = parts.slice(1).join(".")
  (group, key)
}

// Helper: look up the localised text string from translation files
#let get-template(id, lang-code) = {
  let (group, key) = _split-id(id)
  let trans = if lang-code == "en" { _TRANS_EN } else if lang-code == "fr" { _TRANS_FR } else { _TRANS_DE }
  let value = trans.at(group, default: (:)).at(key, default: none)
  if value == none {
    // Fallback: German translation, then raw ID
    _TRANS_DE.at(group, default: (:)).at(key, default: id)
  } else {
    value
  }
}

#let _is-numeric-string(value) = {
  if type(value) != str {
    return false
  }

  let text = value.trim()
  if text == "" {
    return false
  }

  let digits = ("0", "1", "2", "3", "4", "5", "6", "7", "8", "9")
  let has-digit = false
  let dot-count = 0
  let index = 0

  for ch in text.clusters() {
    if ch in digits {
      has-digit = true
    } else if ch == "-" and index == 0 {
      // Leading minus is allowed.
    } else if ch == "." {
      dot-count += 1
      if dot-count > 1 {
        return false
      }
    } else {
      return false
    }

    index += 1
  }

  has-digit
}

#let _is-hex-digit(ch) = ch in (
  "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
  "a", "b", "c", "d", "e", "f",
  "A", "B", "C", "D", "E", "F",
)

#let _is-hex-color-literal(value) = {
  if type(value) != str {
    return false
  }

  let text = value.trim()
  if not text.starts-with("#") {
    return false
  }

  let len = text.len()
  if not (len == 4 or len == 5 or len == 7 or len == 9) {
    return false
  }

  let i = 1
  while i < len {
    let ch = text.slice(i, i + 1)
    if not _is-hex-digit(ch) {
      return false
    }
    i += 1
  }

  true
}

#let _coerce-color(value) = {
  if _is-hex-color-literal(value) {
    return rgb(value.trim())
  }

  if type(value) == color {
    return value
  }

  // Fallback avoids type errors for malformed color tokens in text input.
  white
}

#let _custom-label-parts(label) = {
  let parts = ()
  let buffer = ""
  let in-slot = false

  for ch in label.clusters() {
    if ch == "[" and not in-slot {
      if buffer != "" {
        parts.push((kind: "text", value: buffer))
      }
      buffer = ""
      in-slot = true
    } else if ch == "]" and in-slot {
      parts.push((kind: "slot", value: buffer.trim()))
      buffer = ""
      in-slot = false
    } else {
      buffer += ch
    }
  }

  if buffer != "" {
    if in-slot {
      parts.push((kind: "text", value: "[" + buffer))
    } else {
      parts.push((kind: "text", value: buffer))
    }
  }

  parts
}

#let _custom-variable-name(slot-value) = {
  let value = slot-value.trim()
  if value.starts-with("var (") and value.ends-with(")") {
    return value.slice(5, value.len() - 1).trim()
  }
  if value.starts-with("var [") and value.ends-with("]") {
    return value.slice(5, value.len() - 1).trim()
  }
  if value.starts-with("var ") {
    return value.slice(4).trim()
  }
  none
}

#let _custom-known-reporter(slot-value, langcode) = {
  let value = slot-value.trim()
  if value == "mouse x" {
    sensing-reporter("mouse x")
  } else if value == "mouse y" {
    sensing-reporter("mouse y")
  } else {
    none
  }
}

#let _render-custom-define-label(label) = {
  let parts = _custom-label-parts(label)
  let items = ()

  for part in parts {
    if part.kind == "slot" {
      items.push((name: part.value))
    } else if part.value != "" {
      items.push(part.value)
    }
  }

  if items.len() == 0 {
    custom-block(label)
  } else {
    custom-block(..items)
  }
}

#let _render-custom-call-label(label, custom-colors, langcode) = {
  let parts = _custom-label-parts(label)
  let rendered = ()

  for part in parts {
    if part.kind == "slot" {
      let known-reporter = _custom-known-reporter(part.value, langcode)
      let variable-name = _custom-variable-name(part.value)
      if known-reporter != none {
        rendered.push(known-reporter)
      } else if variable-name != none {
        rendered.push(variables-reporter(variable-name))
      } else {
        rendered.push(number-or-content(part.value, custom-colors))
      }
    } else if part.value != "" {
      rendered.push(part.value)
    }
  }

  if rendered.len() == 0 {
    label
  } else if rendered.len() == 1 {
    rendered.at(0)
  } else {
    stack(dir: ltr, spacing: 1.5mm, ..rendered)
  }
}

// Helper: render a pill for a block argument slot
#let make-pill(key, value, colors, shape: none, block-id: none, options: auto) = {
  let final-options = if options == auto { scratch-block-options.get() } else { options }
  let stroke-thickness = get-stroke-from-options(final-options)
  
  // Dropdown-Felder (typischerweise Strings in Rechteck-Pills)
  let dropdown-keys = ("to", "scene", "costume", "backdrop", "effect", "sound", "key", "object", "property", "timeunit", "layer", "direction", "variable", "list", "clone", "option", "mode", "style", "element", "operator", "towards", "param")
  let message-dropdown-blocks = ("event.when_message_received", "event.broadcast", "event.broadcast_and_wait")
  
  // Condition fields (for boolean operators: and, or, not, and direct condition slots)
  let condition-keys = ("operand", "operand1", "operand2", "condition")
  
  // inline: true for reporters/booleans, inline: false for stack blocks
  let use-inline = shape in ("reporter", "boolean")
  let non-compact-inline-keys = ("num", "num1", "num2", "index", "x", "y", "dx", "dy", "steps", "letter")
  let compact-inline = use-inline and not (key in non-compact-inline-keys)
  let tall-inline = use-inline and (key in non-compact-inline-keys)
  let is-numeric-random-to = key == "to" and _is-numeric-string(value)
  let is-message-dropdown = key == "message" and block-id in message-dropdown-blocks

  if (key in dropdown-keys or is-message-dropdown) and type(value) == str and not is-numeric-random-to {
    pill-rect(value, fill: colors.primary, stroke: colors.tertiary + stroke-thickness, dropdown: true, inline: use-inline, options: final-options)
  } else if key in ("color", "color1", "color2") {
    pill-color("        ", fill: _coerce-color(value), options: final-options)
  } else if key in condition-keys and (value == none or value == []) {
    // Empty condition → dark placeholder with nested: true for smaller insets
    condition(colorschema: colors, type: "condition", [], nested: true, options: final-options)
  } else {
    number-or-content(value, colors, compact: compact-inline, tall: tall-inline, options: final-options)
  }
}

// Hilfsfunktion: Ersetze Platzhalter in Templates - UNIVERSELLE VERSION
#let fill-template(template, args, colors, shape: none, theme: "normal", block-id: none, options: auto) = {
  // Icon-Definitionen aus scratch.typ
  let flag-icon = box(baseline: 20%, image(icon-by-theme("green-flag", theme: theme), width: 1em, height: 1em))
  let arrow-right = box(baseline: 20%, image(icon-by-theme("rotate-right", theme: theme), width: 1.5em, height: 1.5em))
  let arrow-left = box(baseline: 20%, image(icon-by-theme("rotate-left", theme: theme), width: 1.5em, height: 1.5em))
  let pen-icon = box(baseline: 20%, image(icon-by-theme("pen", theme: theme), width: 1.5em, height: 1.5em))
  
  // Einfache Templates ohne Platzhalter
  if not template.contains("{") {
    // Plain text without placeholder — add inset for reporter context
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
    let remaining-after-placeholder = inside-split.slice(1).join("}")

    // Klammern um Platzhalter sind in Parser-Templates oft nur Syntax-Markierung.
    // Beispiel: "move ({steps}) steps" soll visuell ohne sichtbare Klammern rendern.
    let has-leading-slot-paren = before-split.at(0).ends-with("(")
    let has-trailing-slot-paren = remaining-after-placeholder.starts-with(")")
    let strip-slot-parens = placeholder in args and has-leading-slot-paren and has-trailing-slot-paren
    let remaining-next = if strip-slot-parens {
      remaining-after-placeholder.slice(1)
    } else {
      remaining-after-placeholder
    }
    let placeholder-ends-template = not remaining-next.contains("{") and remaining-next.trim() == ""

    // Text vor {
    if before-split.at(0).len() > 0 {
      let text = if strip-slot-parens {
        before-split.at(0).slice(0, before-split.at(0).len() - 1)
      } else {
        before-split.at(0)
      }
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
      // Keep first-position state when template starts with a placeholder,
      // so the first rendered part still gets the left reporter inset.
    }
    
    // Ersetze bekannte Platzhalter
    let placeholder-content = none
    if placeholder == "flag" {
      placeholder-content = flag-icon
    } else if placeholder == "arrow-right" {
      placeholder-content = arrow-right
    } else if placeholder == "arrow-left" {
      placeholder-content = arrow-left
    } else if placeholder == "pen" {
      placeholder-content = pen-icon
    } else if placeholder in args {
      placeholder-content = make-pill(placeholder, args.at(placeholder), colors, shape: shape, block-id: block-id, options: options)
    } else {
      // Unbekannter Platzhalter - als Text behalten
      placeholder-content = "{" + placeholder + "}"
    }

    if placeholder-content != none {
      if shape in ("reporter", "boolean") and is-first-text {
        let first-part = box(inset: (left: 2mm), placeholder-content)
        if placeholder-ends-template {
          parts.push(box(inset: (right: 2mm), first-part))
        } else {
          parts.push(first-part)
        }
      } else if shape in ("reporter", "boolean") and placeholder-ends-template {
        parts.push(box(inset: (right: 2mm), placeholder-content))
      } else {
        parts.push(placeholder-content)
      }
      is-first-text = false
    }
    
    // Weiter mit dem Rest
    remaining = remaining-next
  }
  
  // Append remaining text segment
  if remaining.len() > 0 and not remaining.contains("{") {
    // Letzter Text-Teil - nur right Inset
    if shape in ("reporter", "boolean") and remaining.len() > 0 {
      parts.push(box(inset: (right: 2mm), remaining))
    } else {
      parts.push(remaining)
    }
  }
  
  // No parts: return raw template string
  if parts.len() == 0 {
    return template
  }
  
  // Single part: return directly
  if parts.len() == 1 {
    return parts.at(0)
  }
  
  // Build stack of all parts with adjusted spacing for reporter context
  let final-spacing = if shape in ("reporter", "boolean") { 0.75mm } else { 1.5mm }
  return stack(dir: ltr, spacing: final-spacing, ..parts)
}

// Hilfsfunktion: Hole Block-Info aus verschachtelter Registry
#let get-block-info(id) = {
  let (group, key) = _split-id(id)
  let entry = REGISTRY.at(group, default: (:)).at(key, default: none)
  if entry == none {
    (shape: "stack", category: "control")
  } else {
    // Kategorie aus explizitem Feld (nur data-Gruppe) oder aus CATEGORY_MAP ableiten
    let category = entry.at("category", default: CATEGORY_MAP.at(group, default: "control"))
    (shape: entry.at("shape", default: "stack"), category: category)
  }
}

// Central block rendering function with localisation
// NOTE: This function should only be used for simple template-based blocks.
// Complex blocks (operators, reporters, c-blocks) use the original functions from mod.typ.
#let render-block(id, args: (:), lang-code: str, body: [], else-body: none, options: auto) = {
  if options == auto {
    context {
      let resolved-options = scratch-block-options.get()
      render-block(id, args: args, lang-code: lang-code, body: body, else-body: else-body, options: resolved-options)
    }
    return
  }

  // Normalise language code
  let l = if lang-code == "auto" { "de" } else { lang-code }
  
  // Read options and colours
  let colors = get-colors-from-options(options)
  let stroke-thickness = get-stroke-from-options(options)
  
  // Hole Block-Info aus Registry
  let info = get-block-info(id)
  let category = info.at("category", default: "control")
  let shape = info.at("shape", default: "stack")
  let template = get-template(id, l)
  
  // Kategorie → Farb-Schema: colors hat gleichnamige Felder für alle Kategorien
  let color = colors.at(category, default: colors.control)

  // Custom block definition carries a label-render function, not a normal template value.
  // Handle it before generic template filling to avoid treating functions as inline content.
  if id == "custom.define" {
    let label-value = args.at("label", default: none)
    let define-verb = get-template("custom.define_label", l)
    if label-value != none {
      if type(label-value) == function {
        return define(label-value, verb: define-verb, body)
      }
      if type(label-value) == str {
        return define(_render-custom-define-label(label-value), verb: define-verb, body)
      }
      return define(label-value, verb: define-verb, body)
    }
  }

  if id == "custom.call" {
    let label-value = args.at("label", default: "")
    if type(label-value) == str {
      return custom(_render-custom-call-label(label-value, colors.custom, l))
    }
    return custom(label-value)
  }

  // Dedicated variable reporter: parser uses a synthetic template ("var {...}")
  // for unambiguous matching, but visual output should be just the reporter itself.
  if id == "data.variable" {
    return variables-reporter(args.at("variable", default: ""), options: options)
  }
  
  // Fill template with arguments
  let content = fill-template(template, args, color, shape: shape, theme: options.at("theme", default: "normal"), block-id: id, options: options)

  // Special cases: Control blocks with special shapes.
  // Labels are sourced from the translation system so controls.typ stays language-neutral.
  if id == "control.if_else" or id == "control.if" {
    let cond = if "condition" in args { args.condition } else { condition(colorschema: colors.operators, [], options: options) }
    let tmpl-if   = get-template("control.if_label", l)
    let tmpl-then = get-template("control.then", l)
    let tmpl-else = get-template("control.else", l)
    let lbl = ("if-then": tmpl-if, then: tmpl-then, "else": tmpl-else)
    return if-then-else(cond, then: body, else-body: if id == "control.if_else" { else-body } else { none }, labels: lbl, options: options)
  } else if id == "control.repeat" {
    let times = if "times" in args { args.times } else { 10 }
    let lbl = (repeat: get-template("control.repeat_label", l), times: get-template("control.times_label", l))
    return repeat(count: times, body: body, labels: lbl, options: options)
  } else if id == "control.forever" {
    let lbl = (forever: get-template("control.forever_label", l))
    return repeat-forever(body, labels: lbl, options: options)
  } else if id == "control.repeat_until" {
    let cond = if "condition" in args { args.condition } else { condition(colorschema: colors.operators, [], options: options) }
    let lbl = ("repeat-until": get-template("control.repeat_until_label", l))
    return repeat-until(cond, body: body, labels: lbl, options: options)
  } else if id == "control.start_as_clone" {
    let lbl = get-template("control.start_as_clone", l)
    return when-i-start-as-clone(body, label: lbl, options: options)
  }
  
  // Custom block definition without label function falls back to filled content.
  if id == "custom.define" {
    let define-verb = get-template("custom.define_label", l)
    return define(content, verb: define-verb, body)
  }

  // Monitor widgets (not regular scratch blocks)
  if shape == "monitor-variable" {
    let name  = args.at("name", default: "Variable")
    let value = args.at("value", default: 0)
    return variable-monitor(name: name, value: value)
  } else if shape == "monitor-list" {
    let name         = args.at("name", default: "List")
    let items        = args.at("items", default: ())
    let width        = args.at("width", default: 4cm)
    let height       = args.at("height", default: auto)
    let length-label = get-template("data.length_label", l)
    return list-monitor(name: name, items: items, width: width, height: height, length-label: length-label)
  }

  // Render based on shape
  if shape == "hat" {
    // Event blocks
    event(content, body)
  } else if shape == "reporter" {
    // Value reporters — dispatch via dictionary to avoid long if/else chains
    let reporter-dispatch = (
      motion:    motion-reporter,
      looks:     looks-reporter,
      sound:     sound-reporter,
      pen:       pen-reporter,
      sensing:   sensing-reporter,
      variables: variables-reporter,
      lists:     lists-reporter,
      custom:    custom-reporter,
    )
    if category in reporter-dispatch {
      (reporter-dispatch.at(category))(content)
    } else {
      // operators and any unknown category → plain pill
      pill-reporter(content, fill: color.primary, stroke: color.tertiary + stroke-thickness)
    }
  } else if shape == "input" {
    // Custom input (variable reporter)
    variables-reporter(args.at("text", default: ""))

  } else if shape == "boolean" {
    // Boolean reporters (diamond shape)
    condition(colorschema: color, content)
  } else {
    // Stack blocks — dispatch via dictionary
    let stack-dispatch = (
      motion:    motion,
      looks:     looks,
      sound:     sound,
      pen:       pen,
      control:   content => control(content, bottom-notch: shape != "cap"),
      sensing:   sensing,
      variables: variables,
      lists:     lists,
      custom:    custom,
    )
    if category in stack-dispatch {
      (stack-dispatch.at(category))(content)
    } else if category == "events" {
      // Event category as stack block (e.g. broadcast)
      scratch-block(colorschema: color, type: "statement", dy: block-offset-y, content)
    } else {
      control(content)
    }
  }
}

// Public API for language alias files
// This function is called by lang/de.typ, lang/en.typ etc.
#let block(id, args: (:), lang-code: "auto", body: [], else-body: none) = {
  render-block(id, args: args, lang-code: lang-code, body: body, else-body: else-body)
}
