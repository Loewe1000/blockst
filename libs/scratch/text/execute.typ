#import "parser.typ": parse-scratch-text
#import "../executable.typ": move, turn-right, turn-left, set-direction, go-to, set-x, set-y, change-x, change-y, erase-all, stamp, pen-down, pen-up, set-pen-color, change-pen-size, set-pen-size, set-variable, change-variable

#let _parse(text, language) = parse-scratch-text(text, lang-code: language)

#let _num(value, default: 0) = {
  if type(value) == int or type(value) == float {
    value
  } else if type(value) == str {
    float(value)
  } else {
    default
  }
}

#let _inputs(node) = {
  let out = ()
  for part in node.at("parts", default: ()) {
    if part.at("kind", default: "") == "input" {
      out.push(part)
    }
  }
  out
}

#let _input-value(node, index, default: none) = {
  let inputs = _inputs(node)
  if index < inputs.len() {
    inputs.at(index).at("value", default: default)
  } else {
    default
  }
}

#let _flatten(commands) = {
  let out = ()
  for item in commands {
    if type(item) == array {
      out += _flatten(item)
    } else {
      out.push(item)
    }
  }
  out
}

#let _compile-node(node) = {
  let id = node.id
  let body = node.at("body", default: ())
  let compile-body = nodes => {
    let out = ()
    for nested in nodes {
      out += _compile-node(nested)
    }
    _flatten(out)
  }
  if id == "EVENT_WHENFLAGCLICKED" or id == "EVENT_WHENKEYPRESSED" or id == "EVENT_WHENTHISSPRITECLICKED" or id == "EVENT_WHENBACKDROPSWITCHESTO" or id == "EVENT_WHENBROADCASTRECEIVED" or id == "CONTROL_START_AS_CLONE" {
    return compile-body(body)
  }
  if id == "MOTION_MOVESTEPS" {
    return (move(steps: _num(_input-value(node, 0, default: 10), default: 10)),)
  }
  if id == "MOTION_TURNRIGHT" {
    return (turn-right(degrees: _num(_input-value(node, 0, default: 15), default: 15)),)
  }
  if id == "MOTION_TURNLEFT" {
    return (turn-left(degrees: _num(_input-value(node, 0, default: 15), default: 15)),)
  }
  if id == "MOTION_POINTINDIRECTION" {
    return (set-direction(direction: _num(_input-value(node, 0, default: 90), default: 90)),)
  }
  if id == "MOTION_GOTOXY" {
    return (go-to(x: _num(_input-value(node, 0, default: 0)), y: _num(_input-value(node, 1, default: 0))),)
  }
  if id == "MOTION_SETX" {
    return (set-x(x: _num(_input-value(node, 0, default: 0))),)
  }
  if id == "MOTION_SETY" {
    return (set-y(y: _num(_input-value(node, 0, default: 0))),)
  }
  if id == "MOTION_CHANGEXBY" {
    return (change-x(dx: _num(_input-value(node, 0, default: 0))),)
  }
  if id == "MOTION_CHANGEYBY" {
    return (change-y(dy: _num(_input-value(node, 0, default: 0))),)
  }
  if id == "PEN_CLEAR" {
    return (erase-all(),)
  }
  if id == "PEN_STAMP" {
    return (stamp(),)
  }
  if id == "PEN_PENDOWN" {
    return (pen-down(),)
  }
  if id == "PEN_PENUP" {
    return (pen-up(),)
  }
  if id == "PEN_SETPENCOLORTOCOLOR" {
    return (set-pen-color(color: _input-value(node, 0, default: black)),)
  }
  if id == "PEN_CHANGEPENSIZEBY" {
    return (change-pen-size(size: _num(_input-value(node, 0, default: 1), default: 1)),)
  }
  if id == "PEN_SETPENSIZETO" {
    return (set-pen-size(size: _num(_input-value(node, 0, default: 1), default: 1)),)
  }
  if id == "DATA_SETVARIABLETO" {
    return (set-variable(_input-value(node, 0, default: "var"), _input-value(node, 1, default: 0)),)
  }
  if id == "DATA_CHANGEVARIABLEBY" {
    return (change-variable(_input-value(node, 0, default: "var"), _num(_input-value(node, 1, default: 1), default: 1)),)
  }
  if id == "CONTROL_REPEAT" {
    let count = _num(_input-value(node, 0, default: 10), default: 10)
    let out = ()
    for _i in range(int(count)) {
      out += compile-body(body)
    }
    return out
  }
  if id == "CONTROL_FOREVER" {
    // Intentional bounded execution to keep scratch-run finite.
    let out = ()
    for _i in range(20) {
      out += compile-body(body)
    }
    return out
  }
  if id == "CONTROL_IF" or id == "CONTROL_IF_ELSE" or id == "CONTROL_REPEATUNTIL" or id == "CONTROL_WAIT" {
    return compile-body(body)
  }
  ()
}

#let _compile-nodes(nodes) = {
  let out = ()
  for node in nodes {
    out += _compile-node(node)
  }
  _flatten(out)
}

#let execute-scratch-text(text, language: "en") = _compile-nodes(_parse(text, language))
