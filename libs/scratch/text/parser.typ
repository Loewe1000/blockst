// text/parser.typ — Shared scratchblocks-like parser engine for Blockst
// Language modules (en/de/fr) should only provide lightweight wrappers and
// language-specific configuration around these generic functions.

#import "../core.typ": block, get-template
#import "profiles.typ": get-language-profile

#let _trim(value) = value.trim()

#let _collapse-spaces(value) = {
  let text = value.replace("\t", " ")
  while text.contains("  ") {
    text = text.replace("  ", " ")
  }
  text
}

#let _normalise-punctuation(value) = {
  let text = value
  text = text.replace("?", " ? ")
  text = text.replace(",", " , ")
  _collapse-spaces(text)
}

#let _decode-html-entities(value) = {
  let text = value
  text = text.replace("&lt;", "<")
  text = text.replace("&gt;", ">")
  text
}

#let _normalise-line(value) = _normalise-punctuation(_decode-html-entities(_trim(value)))

#let _apply-scratchblocks-aliases(value, lang-code) = {
  let profile = get-language-profile(lang-code)
  profile.at("apply-aliases")(value)
}

#let _starts-with(value, prefix) = value.starts-with(prefix)

#let _ends-with(value, suffix) = value.ends-with(suffix)

#let _is-wrapped(value, left, right) = {
  if value.len() < 2 or not _starts-with(value, left) or not _ends-with(value, right) {
    return false
  }

  let depth = 0
  let index = 0
  let total = value.clusters().len()

  for cluster in value.clusters() {
    if cluster == left {
      depth += 1
    }
    if cluster == right {
      depth -= 1
      if depth < 0 {
        return false
      }
    }

    if depth == 0 and index < total - 1 {
      return false
    }

    index += 1
  }

  depth == 0
}

#let _next-boundary(text, index) = {
  let pos = 0
  for cluster in text.clusters() {
    if pos >= index {
      return pos
    }
    pos += cluster.len()
  }
  pos
}

#let _strip-dropdown-marker(value) = {
  // Remove Scratch-style dropdown marker: " v" before closing bracket
  if value.ends-with(" v") {
    _trim(value.slice(0, value.len() - 2))
  } else {
    value
  }
}

#let _is-mathop-operator(value) = {
  let token = _trim(value)
  let operator = if token.len() >= 2 and token.starts-with("[") and token.ends-with("]") {
    let inner = _trim(token.slice(1, token.len() - 1))
    if inner.ends-with(" v") {
      _trim(inner.slice(0, inner.len() - 2))
    } else {
      inner
    }
  } else {
    token
  }
  operator in (
    "abs",
    "floor",
    "ceiling",
    "sqrt",
    "wurzel",
    "Wurzel",
    "betrag",
    "Betrag",
    "abrunden",
    "Abrunden",
    "aufrunden",
    "Aufrunden",
    "sin",
    "cos",
    "tan",
    "asin",
    "acos",
    "atan",
    "ln",
    "log",
    "e ^",
    "10 ^",
  )
}

#let _is-dropdown-token(value) = {
  let token = _trim(value)
  if not _is-wrapped(token, "[", "]") {
    return false
  }

  let inner = _trim(token.slice(1, token.len() - 1))
  inner.ends-with(" v")
}

#let _strip-wrappers(value) = {
  let text = _trim(value)
  if _is-wrapped(text, "(", ")") or _is-wrapped(text, "[", "]") or _is-wrapped(text, "<", ">") {
    let unwrapped = _trim(text.slice(1, text.len() - 1))
    _strip-dropdown-marker(unwrapped)
  } else {
    text
  }
}

#let _expression-opener-count(value) = {
  let count = 0
  for ch in value.clusters() {
    if ch == "(" or ch == "<" {
      count += 1
    }
  }
  count
}

#let _find-segment(text, needle, start: 0) = {
  if needle.len() == 0 {
    return start
  }

  let depth-paren = 0
  let depth-square = 0
  let depth-angle = 0
  let prev-cluster = none
  let byte-pos = 0

  for cluster in text.clusters() {
    let next-pos = byte-pos + cluster.len()
    let next-cluster = if next-pos < text.len() {
      text.slice(next-pos, _next-boundary(text, next-pos + 1))
    } else {
      none
    }

    if byte-pos < start {
      prev-cluster = cluster
      byte-pos += cluster.len()
      continue
    }

    if byte-pos + needle.len() > text.len() {
      return none
    }

    if depth-paren == 0 and depth-square == 0 and depth-angle == 0 {
      if text.slice(byte-pos).starts-with(needle) {
        return byte-pos
      }
    }

    let ch = cluster
    if ch == "(" {
      depth-paren += 1
    } else if ch == ")" and depth-paren > 0 {
      depth-paren -= 1
    } else if ch == "[" {
      depth-square += 1
    } else if ch == "]" and depth-square > 0 {
      depth-square -= 1
    } else if ch == "<" {
      // Treat "<" as angle-group opener only when it is not a plain
      // comparison operator surrounded by spaces (e.g. "a < b").
      if not (prev-cluster == " " and next-cluster == " ") {
        depth-angle += 1
      }
    } else if ch == ">" and depth-angle > 0 {
      depth-angle -= 1
    }

    prev-cluster = cluster
    byte-pos += cluster.len()
  }

  none
}

#let _parse-template(template) = {
  let template = _normalise-punctuation(template)
  let segments = ()
  let placeholders = ()
  let remaining = template

  while remaining.contains("{") {
    let before = remaining.split("{")
    if before.len() < 2 {
      break
    }
    segments.push(before.at(0))

    let after-open = before.slice(1).join("{")
    let inside = after-open.split("}")
    if inside.len() < 2 {
      segments.push("{" + after-open)
      return (segments: segments, placeholders: placeholders)
    }

    placeholders.push(inside.at(0))
    remaining = inside.slice(1).join("}")
  }

  segments.push(remaining)
  (segments: segments, placeholders: placeholders)
}

#let _template-score(template) = {
  let parsed = _parse-template(template)
  let score = 0
  for segment in parsed.segments {
    score += segment.len()
  }
  score * 100 - parsed.placeholders.len()
}

#let _order-defs(defs, lang-code) = {
  let ordered = ()
  for raw-def in defs {
    let copy = raw-def
    copy.insert("score", _template-score(get-template(copy.id, lang-code)))

    let inserted = false
    let idx = 0
    while idx < ordered.len() {
      if copy.score > ordered.at(idx).score {
        ordered.insert(idx, copy)
        inserted = true
        idx = ordered.len()
      } else {
        idx += 1
      }
    }

    if not inserted {
      ordered.push(copy)
    }
  }
  ordered
}

#let _match-template-plain(line, template) = {
  let parsed = _parse-template(template)
  let segments = parsed.segments
  let placeholders = parsed.placeholders
  let args = (:)
  let line-index = 0

  let i = 0
  while i < placeholders.len() {
    line-index = _next-boundary(line, line-index)
    let prefix = segments.at(i)
    if line-index + prefix.len() > line.len() {
      return none
    }
    if not line.slice(line-index).starts-with(prefix) {
      return none
    }
    line-index += prefix.len()

    let next-segment = segments.at(i + 1)
    if next-segment.len() == 0 {
      let token = _trim(line.slice(line-index))
      args.insert(placeholders.at(i), token)
      line-index = line.len()
    } else {
      let next-pos = _find-segment(line, next-segment, start: line-index)
      if next-pos == none {
        return none
      }
      let token = _trim(line.slice(line-index, next-pos))
      args.insert(placeholders.at(i), token)
      line-index = next-pos
    }

    i += 1
  }

  let trailing = segments.at(segments.len() - 1)
  line-index = _next-boundary(line, line-index)
  if line-index + trailing.len() > line.len() {
    return none
  }
  if not line.slice(line-index).starts-with(trailing) {
    return none
  }
  if line-index + trailing.len() != line.len() {
    return none
  }

  args
}

#let _try-parse-expression(line, lang-code, expression-defs, depth: 0) = {
  if depth > 24 {
    return none
  }

  let source = _apply-scratchblocks-aliases(_normalise-line(line), lang-code)
  if source == "" {
    return none
  }

  for expr-def in expression-defs {
    let template = get-template(expr-def.id, lang-code)
    let parsed-template = _parse-template(template)
    let raw-args = _match-template-plain(source, template)
    if raw-args != none {
      // Disambiguate list reporters/booleans from generic string operators.
      // `length of [list v]` and `[list v] contains [...]` must map to data.*
      // and not to operator.length/operator.contains.
      if expr-def.id == "operator.length" and "string" in raw-args {
        if _is-dropdown-token(raw-args.at("string")) {
          continue
        }
      }

      if expr-def.id == "operator.contains" and "string1" in raw-args {
        if _is-dropdown-token(raw-args.at("string1")) {
          continue
        }
      }

      if expr-def.id == "sensing.of" and "property" in raw-args {
        if _is-mathop-operator(raw-args.at("property")) {
          continue
        }
      }

      let args = (:)
      for key in parsed-template.placeholders {
        let raw-token = _trim(raw-args.at(key))
        if raw-token == "" {
          args.insert(key, "")
        } else if _is-wrapped(raw-token, "[", "]") {
          // Dropdown/string-style slots should stay atomic and must not recurse.
          args.insert(key, _strip-wrappers(raw-token))
        } else {
          let inner-token = if _is-wrapped(raw-token, "(", ")") or _is-wrapped(raw-token, "<", ">") {
            _strip-wrappers(raw-token)
          } else {
            raw-token
          }
          let is-expression-slot = key in ("condition", "operand", "operand1", "operand2")
          let looks-like-expression = raw-token.starts-with("not ") or raw-token.contains(" ?") or raw-token.contains(" and ") or raw-token.contains(" or ") or raw-token.contains(" > ") or raw-token.contains(" < ") or raw-token.contains(" = ")
          let should-try-unwrapped = is-expression-slot and looks-like-expression and depth < 6
          let nested = if inner-token != raw-token or should-try-unwrapped {
            _try-parse-expression(inner-token, lang-code, expression-defs, depth: depth + 1)
          } else {
            none
          }
          if nested != none {
            args.insert(key, nested)
          } else {
            args.insert(key, _strip-wrappers(raw-token))
          }
        }
      }
      return (id: expr-def.id, args: args)
    }
  }

  none
}

#let _decode-token(value, key, lang-code, expression-defs, expression-only: false) = {
  let cleaned = _trim(value)
  if cleaned == "" {
    return ""
  }

  // Guard against pathological deep nesting in large imported scripts.
  // Above this threshold, keep token text instead of recursive expression parsing.
  if _expression-opener-count(cleaned) > 24 {
    return _strip-wrappers(cleaned)
  }

  if key in ("condition", "operand", "operand1", "operand2") {
    let parsed = _try-parse-expression(cleaned, lang-code, expression-defs)
    if parsed != none {
      return parsed
    }

    let inner = if _is-wrapped(cleaned, "(", ")") or _is-wrapped(cleaned, "<", ">") or _is-wrapped(cleaned, "[", "]") {
      _strip-wrappers(cleaned)
    } else {
      cleaned
    }

    let parsed-inner = _try-parse-expression(inner, lang-code, expression-defs)
    return if parsed-inner != none { parsed-inner } else { inner }
  }

  if key in ("value", "x", "y", "index", "string1", "string2") {
    let parsed = _try-parse-expression(cleaned, lang-code, expression-defs)
    if parsed != none {
      return parsed
    }

    let inner = if _is-wrapped(cleaned, "(", ")") or _is-wrapped(cleaned, "<", ">") {
      _strip-wrappers(cleaned)
    } else {
      cleaned
    }
    let parsed-inner = _try-parse-expression(inner, lang-code, expression-defs)
    if parsed-inner != none {
      return parsed-inner
    }
  }

  if key in ("costume", "backdrop", "sound") {
    let parsed = _try-parse-expression(cleaned, lang-code, expression-defs)
    if parsed != none {
      return parsed
    }

    let inner = if _is-wrapped(cleaned, "(", ")") or _is-wrapped(cleaned, "<", ">") or _is-wrapped(cleaned, "[", "]") {
      _strip-wrappers(cleaned)
    } else {
      cleaned
    }
    let parsed-inner = _try-parse-expression(inner, lang-code, expression-defs)
    if parsed-inner != none {
      return parsed-inner
    }
  }

  if _is-wrapped(cleaned, "[", "]") {
    return _strip-wrappers(cleaned)
  }

  if _is-wrapped(cleaned, "<", ">") {
    let parsed = _try-parse-expression(_strip-wrappers(cleaned), lang-code, expression-defs)
    return if parsed != none { parsed } else { _strip-wrappers(cleaned) }
  }

  if _is-wrapped(cleaned, "(", ")") {
    let inner = _strip-wrappers(cleaned)
    let parsed = _try-parse-expression(inner, lang-code, expression-defs)
    return if parsed != none { parsed } else { inner }
  }

  if expression-only {
    let parsed = _try-parse-expression(cleaned, lang-code, expression-defs)
    return if parsed != none { parsed } else { cleaned }
  }

  // Preserve reporter semantics in slots that are otherwise treated as plain
  // text (e.g. list item/value fields containing `var [Name]`).
  if cleaned.starts-with("var ") {
    let parsed = _try-parse-expression(cleaned, lang-code, expression-defs)
    if parsed != none {
      return parsed
    }
  }

  cleaned
}

#let _match-template(line, template, lang-code, expression-defs, expression-only: false) = {
  let parsed = _parse-template(template)
  let segments = parsed.segments
  let placeholders = parsed.placeholders
  let args = (:)
  let line-index = 0

  let i = 0
  while i < placeholders.len() {
    line-index = _next-boundary(line, line-index)
    let prefix = segments.at(i)
    if line-index + prefix.len() > line.len() {
      return none
    }
    if not line.slice(line-index).starts-with(prefix) {
      return none
    }
    line-index += prefix.len()

    let next-segment = segments.at(i + 1)
    if next-segment.len() == 0 {
      let token = line.slice(line-index)
      args.insert(placeholders.at(i), _decode-token(token, placeholders.at(i), lang-code, expression-defs, expression-only: expression-only))
      line-index = line.len()
    } else {
      let next-pos = _find-segment(line, next-segment, start: line-index)
      if next-pos == none {
        return none
      }
      let token = line.slice(line-index, next-pos)
      args.insert(placeholders.at(i), _decode-token(token, placeholders.at(i), lang-code, expression-defs, expression-only: expression-only))
      line-index = next-pos
    }

    i += 1
  }

  let trailing = segments.at(segments.len() - 1)
  line-index = _next-boundary(line, line-index)
  if line-index + trailing.len() > line.len() {
    return none
  }
  if not line.slice(line-index).starts-with(trailing) {
    return none
  }
  if line-index + trailing.len() != line.len() {
    return none
  }

  args
}

#let _parse-block-line(line, lang-code, statement-defs, expression-defs, allow-body: true, expression-only: false) = {
  let source = _apply-scratchblocks-aliases(_normalise-line(line), lang-code)
  if source == "" {
    return none
  }

  let defs = if expression-only { expression-defs } else { statement-defs }
  let candidates = (source,)

  if expression-only {
    if source.starts-with("(") and source.ends-with(")") and source.len() > 2 {
      candidates.push(_normalise-line(source.slice(1, source.len() - 1)))
    }

    if source.starts-with("<") and source.ends-with(">") and source.len() > 2 {
      candidates.push(_normalise-line(source.slice(1, source.len() - 1)))
    }

    if _is-wrapped(source, "[", "]") {
      candidates.push(_normalise-line(_strip-wrappers(source)))
    }
  }

  for candidate in candidates {
    for entry in defs {
      let template = get-template(entry.id, lang-code)

      if expression-only and entry.id == "operator.length" {
        let raw-args = _match-template-plain(candidate, template)
        if raw-args != none and "string" in raw-args {
          if _is-dropdown-token(raw-args.at("string")) {
            continue
          }
        }
      }

      if expression-only and entry.id == "operator.contains" {
        let raw-args = _match-template-plain(candidate, template)
        if raw-args != none and "string1" in raw-args {
          if _is-dropdown-token(raw-args.at("string1")) {
            continue
          }
        }
      }

      if expression-only and entry.id == "sensing.of" {
        let raw-args = _match-template-plain(candidate, template)
        if raw-args != none and "property" in raw-args {
          if _is-mathop-operator(raw-args.at("property")) {
            continue
          }
        }
      }

      let args = _match-template(candidate, template, lang-code, expression-defs, expression-only: expression-only)
      if args != none {
        return (
          id: entry.id,
          args: args,
          opens-body: allow-body and entry.opens-body,
          control-kind: entry.control-kind,
        )
      }
    }
  }

  none
}

#let _prepare-lines(text, line-comment-prefix) = {
  if type(text) != str {
    panic("Text parser: render-scratch-text expects a string input.")
  }

  let lines = ()
  for raw-line in text.replace("\r", "").split("\n") {
    let stripped = if raw-line.contains(line-comment-prefix) {
      raw-line.split(line-comment-prefix).at(0)
    } else {
      raw-line
    }

    let line = _normalise-line(stripped)
    if line == "" or _starts-with(line, line-comment-prefix) {
      continue
    }
    lines.push(line)
  }
  lines
}

#let _render-arg-value(value, lang-code) = {
  if type(value) == dictionary and "id" in value and "args" in value {
    let nested-args = (:)
    for key in value.args.keys() {
      nested-args.insert(key, _render-arg-value(value.args.at(key), lang-code))
    }
    block(value.id, args: nested-args, lang-code: lang-code)
  } else {
    value
  }
}

#let _render-node-args(args, lang-code) = {
  let rendered = (:)
  for key in args.keys() {
    rendered.insert(key, _render-arg-value(args.at(key), lang-code))
  }
  rendered
}

#let _render-node(node, lang-code) = {
  let rendered-args = _render-node-args(node.args, lang-code)
  let body = node.at("body", default: none)
  let else-body = node.at("else-body", default: none)

  let body-content = if body != none {
    [
      #for child in body {
        _render-node(child, lang-code)
      }
    ]
  } else {
    none
  }

  let else-content = if else-body != none {
    [
      #for child in else-body {
        _render-node(child, lang-code)
      }
    ]
  } else {
    none
  }

  if body == none and else-body == none {
    return block(node.id, args: rendered-args, lang-code: lang-code)
  }

  if else-body != none {
    return block(
      "control.if_else",
      args: rendered-args,
      lang-code: lang-code,
      body: body-content,
      else-body: else-content,
    )
  }

  block(node.id, args: rendered-args, lang-code: lang-code, body: body-content)
}

#let _render-nodes(nodes, lang-code) = [
  #for node in nodes {
    _render-node(node, lang-code)
  }
]

#let _parse-nodes(lines, index, lang-code, end-marker, else-marker, statement-defs, expression-defs, stop-on-hat: false) = {
  let nodes = ()
  let i = index

  while i < lines.len() {
    let line = lines.at(i)

    if stop-on-hat {
      let boundary = _parse-block-line(line, lang-code, statement-defs, expression-defs)
      if boundary != none and boundary.control-kind == "hat" {
        return (nodes: nodes, next: i, marker: "hat-boundary")
      }
    }

    if line == end-marker {
      return (nodes: nodes, next: i + 1, marker: end-marker)
    }
    if line == else-marker {
      return (nodes: nodes, next: i + 1, marker: else-marker)
    }

    let parsed = _parse-block-line(line, lang-code, statement-defs, expression-defs)
    if parsed == none {
      parsed = _parse-block-line(line, lang-code, statement-defs, expression-defs, allow-body: false, expression-only: true)
    }
    if parsed == none {
      panic("Text parser: unknown or unsupported line `" + line + "`.")
    }

    if parsed.opens-body {
      if parsed.control-kind == "hat" {
        let body-result = _parse-nodes(
          lines,
          i + 1,
          lang-code,
          end-marker,
          else-marker,
          statement-defs,
          expression-defs,
          stop-on-hat: true,
        )

        if body-result.marker != none and body-result.marker != "hat-boundary" {
          panic("Text parser: unexpected marker `" + body-result.marker + "` in hat body.")
        }

        let node = parsed
        node.insert("body", body-result.nodes)
        nodes.push(node)
        i = body-result.next
        continue
      }

      let body-result = _parse-nodes(lines, i + 1, lang-code, end-marker, else-marker, statement-defs, expression-defs)
      if body-result.marker == none and parsed.control-kind != "hat" {
        panic("Text parser: missing `" + end-marker + "` for `" + line + "`.")
      }

      let node = parsed
      if body-result.marker == else-marker {
        if parsed.control-kind in ("if", "if_else") {
          let else-result = _parse-nodes(lines, body-result.next, lang-code, end-marker, else-marker, statement-defs, expression-defs)
          if else-result.marker != end-marker {
            panic("Text parser: expected `" + end-marker + "` after `" + else-marker + "`.")
          }
          node.insert("id", "control.if_else")
          node.insert("body", body-result.nodes)
          node.insert("else-body", else-result.nodes)
          nodes.push(node)
          i = else-result.next
          continue
        }
        panic("Text parser: unexpected `" + else-marker + "` after `" + line + "`.")
      }

      node.insert("body", body-result.nodes)
      nodes.push(node)
      i = body-result.next
      continue
    }

    nodes.push(parsed)
    i += 1
  }

  (nodes: nodes, next: i, marker: none)
}

#let make-def(id, opens-body: false, control-kind: none) = (
  id: id,
  opens-body: opens-body,
  control-kind: control-kind,
)

#let default-statement-defs-raw = (
  make-def("event.when_flag_clicked", opens-body: true, control-kind: "hat"),
  make-def("event.when_key_pressed", opens-body: true, control-kind: "hat"),
  make-def("event.when_sprite_clicked", opens-body: true, control-kind: "hat"),
  make-def("event.when_scene_starts", opens-body: true, control-kind: "hat"),
  make-def("event.when_value_exceeds", opens-body: true, control-kind: "hat"),
  make-def("event.when_message_received", opens-body: true, control-kind: "hat"),
  make-def("event.broadcast"),
  make-def("event.broadcast_and_wait"),

  make-def("motion.move_steps"),
  make-def("motion.turn_right"),
  make-def("motion.turn_left"),
  make-def("motion.goto"),
  make-def("motion.goto_xy"),
  make-def("motion.glide"),
  make-def("motion.glide_to_xy"),
  make-def("motion.point_in_direction"),
  make-def("motion.point_towards"),
  make-def("motion.change_x"),
  make-def("motion.set_x"),
  make-def("motion.change_y"),
  make-def("motion.set_y"),
  make-def("motion.if_on_edge_bounce"),
  make-def("motion.set_rotation_style"),

  make-def("looks.say_for_secs"),
  make-def("looks.say"),
  make-def("looks.think_for_secs"),
  make-def("looks.think"),
  make-def("looks.switch_costume_to"),
  make-def("looks.next_costume"),
  make-def("looks.switch_backdrop_to"),
  make-def("looks.switch_backdrop_to_and_wait"),
  make-def("looks.next_backdrop"),
  make-def("looks.change_size_by"),
  make-def("looks.set_size_to"),
  make-def("looks.change_effect_by"),
  make-def("looks.set_effect_to"),
  make-def("looks.clear_graphic_effects"),
  make-def("looks.show"),
  make-def("looks.hide"),
  make-def("looks.goto_front_back"),
  make-def("looks.go_forward_backward_layers"),

  make-def("sound.play_until_done"),
  make-def("sound.start_sound"),
  make-def("sound.stop_all_sounds"),
  make-def("sound.change_effect_by"),
  make-def("sound.set_effect_to"),
  make-def("sound.clear_effects"),
  make-def("sound.change_volume_by"),
  make-def("sound.set_volume_to"),

  make-def("music.play_note_for_beats"),
  make-def("music.set_instrument_to"),
  make-def("music.play_drum_for_beats"),
  make-def("music.rest_for_beats"),
  make-def("music.change_tempo_by"),
  make-def("music.set_tempo_to"),

  make-def("pen.clear"),
  make-def("pen.stamp"),
  make-def("pen.pen_down"),
  make-def("pen.pen_up"),
  make-def("pen.set_pen_color_to_color"),
  make-def("pen.change_pen_param_by"),
  make-def("pen.set_pen_param_to"),
  make-def("pen.change_pen_size_by"),
  make-def("pen.set_pen_size_to"),

  make-def("control.wait"),
  make-def("control.repeat", opens-body: true, control-kind: "repeat"),
  make-def("control.forever", opens-body: true, control-kind: "forever"),
  make-def("control.if", opens-body: true, control-kind: "if"),
  make-def("control.if_else", opens-body: true, control-kind: "if_else"),
  make-def("control.wait_until"),
  make-def("control.repeat_until", opens-body: true, control-kind: "repeat_until"),
  make-def("control.stop"),
  make-def("control.start_as_clone", opens-body: true, control-kind: "hat"),
  make-def("control.create_clone_of"),
  make-def("control.delete_this_clone"),

  make-def("sensing.ask_and_wait"),
  make-def("sensing.set_drag_mode"),
  make-def("sensing.reset_timer"),
  make-def("sensing.turn_video"),
  make-def("sensing.set_video_transparency"),

  make-def("picoboard.when_button_pressed", opens-body: true, control-kind: "hat"),
  make-def("picoboard.when_slider", opens-body: true, control-kind: "hat"),

  make-def("wedo.motor_on_for"),
  make-def("wedo.motor_on"),
  make-def("wedo.motor_off"),
  make-def("wedo.set_motor_power"),
  make-def("wedo.set_motor_direction"),
  make-def("wedo.when_distance"),
  make-def("wedo.when_tilt", opens-body: true, control-kind: "hat"),

  make-def("wedo2.motor_on_for"),
  make-def("wedo2.motor_on"),
  make-def("wedo2.motor_off"),
  make-def("wedo2.set_motor_power"),
  make-def("wedo2.set_motor_direction"),
  make-def("wedo2.set_light_color"),
  make-def("wedo2.play_note_for_seconds"),
  make-def("wedo2.when_distance", opens-body: true, control-kind: "hat"),
  make-def("wedo2.when_tilted", opens-body: true, control-kind: "hat"),

  make-def("control.forever_if", opens-body: true, control-kind: "if"),

  make-def("grey.ellipsis"),

  make-def("data.set_variable_to"),
  make-def("data.change_variable_by"),
  make-def("data.show_variable"),
  make-def("data.hide_variable"),
  make-def("data.add_to_list"),
  make-def("data.delete_of_list"),
  make-def("data.delete_all_of_list"),
  make-def("data.insert_at_list"),
  make-def("data.replace_item_of_list"),
  make-def("data.show_list"),
  make-def("data.hide_list"),

  make-def("custom.define", opens-body: true, control-kind: "hat"),
  make-def("custom.call"),
)

#let default-expression-defs-raw = (
  make-def("motion.x_position"),
  make-def("motion.y_position"),
  make-def("motion.direction"),

  make-def("looks.costume_number_name"),
  make-def("looks.backdrop_number_name"),
  make-def("looks.size"),
  make-def("looks.costume_number"),
  make-def("looks.backdrop_number"),
  make-def("looks.backdrop_name"),

  make-def("sound.volume"),
  make-def("music.tempo"),

  make-def("sensing.touching_object"),
  make-def("sensing.touching_color"),
  make-def("sensing.color_is_touching_color"),
  make-def("sensing.distance_to"),
  make-def("sensing.answer"),
  make-def("sensing.key_pressed"),
  make-def("sensing.mouse_down"),
  make-def("sensing.mouse_x"),
  make-def("sensing.mouse_y"),
  make-def("sensing.loudness"),
  make-def("sensing.timer"),
  make-def("sensing.of"),
  make-def("sensing.current"),
  make-def("sensing.days_since_2000"),
  make-def("sensing.username"),
  make-def("sensing.user_id"),
  make-def("sensing.video_on"),
  make-def("picoboard.sensor_pressed"),
  make-def("picoboard.sensor_value"),
  make-def("wedo.distance"),
  make-def("wedo.tilt"),
  make-def("wedo2.distance"),
  make-def("wedo2.tilt"),

  make-def("operator.add"),
  make-def("operator.subtract"),
  make-def("operator.multiply"),
  make-def("operator.divide"),
  make-def("operator.random"),
  make-def("operator.gt"),
  make-def("operator.lt"),
  make-def("operator.equals"),
  make-def("operator.and"),
  make-def("operator.or"),
  make-def("operator.not"),
  make-def("operator.join"),
  make-def("operator.letter_of"),
  make-def("operator.length"),
  make-def("operator.contains"),
  make-def("operator.mod"),
  make-def("operator.round"),
  make-def("operator.mathop"),

  make-def("data.variable"),
  make-def("data.item_of_list"),
  make-def("data.item_number_of_list"),
  make-def("data.length_of_list"),
  make-def("data.list_contains_item"),
)

#let parse-scratch-text(
  text,
  lang-code: "en",
  end-marker: "end",
  else-marker: "else",
  line-comment-prefix: "//",
  statement-defs-raw: default-statement-defs-raw,
  expression-defs-raw: default-expression-defs-raw,
) = {
  let end-marker = _normalise-line(end-marker)
  let else-marker = _normalise-line(else-marker)

  let statement-defs = _order-defs(statement-defs-raw, lang-code)
  let expression-defs = _order-defs(expression-defs-raw, lang-code)

  let lines = _prepare-lines(text, line-comment-prefix)
  let parsed = _parse-nodes(lines, 0, lang-code, end-marker, else-marker, statement-defs, expression-defs)
  if parsed.marker != none {
    panic("Text parser: stray `" + parsed.marker + "` without matching start block.")
  }
  parsed.nodes
}

#let render-scratch-text(
  text,
  lang-code: "en",
  end-marker: "end",
  else-marker: "else",
  line-comment-prefix: "//",
  statement-defs-raw: default-statement-defs-raw,
  expression-defs-raw: default-expression-defs-raw,
) = {
  let nodes = parse-scratch-text(
    text,
    lang-code: lang-code,
    end-marker: end-marker,
    else-marker: else-marker,
    line-comment-prefix: line-comment-prefix,
    statement-defs-raw: statement-defs-raw,
    expression-defs-raw: expression-defs-raw,
  )
  _render-nodes(nodes, lang-code)
}
