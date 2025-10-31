// registry.typ — Datengetriebene Block-Registry (Vollständig)
// Enthält alle Scratch-Blöcke mit offiziellen Bezeichnungen

#let REGISTRY = (
  // =====================
  // EREIGNISSE (Events)
  // =====================
  "event.when_flag_clicked": (
    de: "Wenn {flag} angeklickt wird",
    en: "when {flag} clicked",
    shape: "hat",
    category: "ereignisse",
  ),
  "event.when_key_pressed": (
    de: "Wenn Taste {key} gedrückt wird",
    en: "when {key} key pressed",
    shape: "hat",
    category: "ereignisse",
  ),
  "event.when_sprite_clicked": (
    de: "Wenn diese Figur angeklickt wird",
    en: "when this sprite clicked",
    shape: "hat",
    category: "ereignisse",
  ),
  "event.when_scene_starts": (
    de: "Wenn Bühnenbild zu {scene} wechselt",
    en: "when backdrop switches to {scene}",
    shape: "hat",
    category: "ereignisse",
  ),
  "event.when_value_exceeds": (
    de: "Wenn {element} > {value}",
    en: "when {element} > {value}",
    shape: "hat",
    category: "ereignisse",
  ),
  "event.when_message_received": (
    de: "Wenn ich {message} empfange",
    en: "when I receive {message}",
    shape: "hat",
    category: "ereignisse",
  ),
  "event.broadcast": (
    de: "sende {message} an alle",
    en: "broadcast {message}",
    shape: "stack",
    category: "ereignisse",
  ),
  "event.broadcast_and_wait": (
    de: "sende {message} an alle und warte",
    en: "broadcast {message} and wait",
    shape: "stack",
    category: "ereignisse",
  ),

  // =====================
  // BEWEGUNG (Motion)
  // =====================
  "motion.move_steps": (
    de: "gehe {steps} er Schritt",
    en: "move {steps} steps",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.turn_right": (
    de: "drehe dich {arrow-right} um {degrees} Grad",
    en: "turn {arrow-right} {degrees} degrees",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.turn_left": (
    de: "drehe dich {arrow-left} um {degrees} Grad",
    en: "turn {arrow-left} {degrees} degrees",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.goto": (
    de: "gehe zu {to}",
    en: "go to {to}",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.goto_xy": (
    de: "gehe zu x: {x} y: {y}",
    en: "go to x: {x} y: {y}",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.glide": (
    de: "gleite {secs} Sek. zu {to}",
    en: "glide {secs} secs to {to}",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.glide_to_xy": (
    de: "gleite {secs} Sek. zu x: {x} y: {y}",
    en: "glide {secs} secs to x: {x} y: {y}",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.point_in_direction": (
    de: "setze Richtung auf {direction} Grad",
    en: "point in direction {direction}",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.point_towards": (
    de: "drehe dich zu {towards}",
    en: "point towards {towards}",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.change_x": (
    de: "ändere x um {dx}",
    en: "change x by {dx}",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.set_x": (
    de: "setze x auf {x}",
    en: "set x to {x}",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.change_y": (
    de: "ändere y um {dy}",
    en: "change y by {dy}",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.set_y": (
    de: "setze y auf {y}",
    en: "set y to {y}",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.if_on_edge_bounce": (
    de: "pralle vom Rand ab",
    en: "if on edge, bounce",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.set_rotation_style": (
    de: "setze Drehtyp auf {style}",
    en: "set rotation style {style}",
    shape: "stack",
    category: "bewegung",
  ),
  "motion.x_position": (
    de: "x-Position",
    en: "x position",
    shape: "reporter",
    category: "bewegung",
  ),
  "motion.y_position": (
    de: "y-Position",
    en: "y position",
    shape: "reporter",
    category: "bewegung",
  ),
  "motion.direction": (
    de: "Richtung",
    en: "direction",
    shape: "reporter",
    category: "bewegung",
  ),

  // =====================
  // AUSSEHEN (Looks)
  // =====================
  "looks.say_for_secs": (
    de: "sage {message} für {secs} Sekunden",
    en: "say {message} for {secs} seconds",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.say": (
    de: "sage {message}",
    en: "say {message}",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.think_for_secs": (
    de: "denke {message} für {secs} Sekunden",
    en: "think {message} for {secs} seconds",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.think": (
    de: "denke {message}",
    en: "think {message}",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.switch_costume_to": (
    de: "wechsle zu Kostüm {costume}",
    en: "switch costume to {costume}",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.next_costume": (
    de: "nächstes Kostüm",
    en: "next costume",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.switch_backdrop_to": (
    de: "wechsle zu Bühnenbild {backdrop}",
    en: "switch backdrop to {backdrop}",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.next_backdrop": (
    de: "nächstes Bühnenbild",
    en: "next backdrop",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.change_size_by": (
    de: "ändere Größe um {change}",
    en: "change size by {change}",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.set_size_to": (
    de: "setze Größe auf {size} %",
    en: "set size to {size} %",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.change_effect_by": (
    de: "ändere Effekt {effect} um {change}",
    en: "change {effect} effect by {change}",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.set_effect_to": (
    de: "setze Effekt {effect} auf {value}",
    en: "set {effect} effect to {value}",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.clear_graphic_effects": (
    de: "schalte Grafikeffekte aus",
    en: "clear graphic effects",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.show": (
    de: "zeige dich",
    en: "show",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.hide": (
    de: "verstecke dich",
    en: "hide",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.goto_front_back": (
    de: "gehe zu {layer} Ebene",
    en: "go to {layer} layer",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.go_forward_backward_layers": (
    de: "gehe {num} Ebenen nach {direction}",
    en: "go {direction} {num} layers",
    shape: "stack",
    category: "aussehen",
  ),
  "looks.costume_number_name": (
    de: "Kostüm {property}",
    en: "costume {property}",
    shape: "reporter",
    category: "aussehen",
  ),
  "looks.backdrop_number_name": (
    de: "Bühnenbild {property}",
    en: "backdrop {property}",
    shape: "reporter",
    category: "aussehen",
  ),
  "looks.size": (
    de: "Größe",
    en: "size",
    shape: "reporter",
    category: "aussehen",
  ),

  // =====================
  // KLANG (Sound)
  // =====================
  "sound.play_until_done": (
    de: "spiele Klang {sound} ganz",
    en: "play sound {sound} until done",
    shape: "stack",
    category: "klang",
  ),
  "sound.start_sound": (
    de: "spiele Klang {sound}",
    en: "start sound {sound}",
    shape: "stack",
    category: "klang",
  ),
  "sound.stop_all_sounds": (
    de: "stoppe alle Klänge",
    en: "stop all sounds",
    shape: "stack",
    category: "klang",
  ),
  "sound.change_effect_by": (
    de: "ändere {effect}-Effekt um {value}",
    en: "change {effect} effect by {value}",
    shape: "stack",
    category: "klang",
  ),
  "sound.set_effect_to": (
    de: "setze {effect}-Effekt auf {value}",
    en: "set {effect} effect to {value}",
    shape: "stack",
    category: "klang",
  ),
  "sound.clear_effects": (
    de: "schalte Klangeffekte aus",
    en: "clear sound effects",
    shape: "stack",
    category: "klang",
  ),
  "sound.change_volume_by": (
    de: "ändere Lautstärke um {volume}",
    en: "change volume by {volume}",
    shape: "stack",
    category: "klang",
  ),
  "sound.set_volume_to": (
    de: "setze Lautstärke auf {volume} %",
    en: "set volume to {volume} %",
    shape: "stack",
    category: "klang",
  ),
  "sound.volume": (
    de: "Lautstärke",
    en: "volume",
    shape: "reporter",
    category: "klang",
  ),

  // =====================
  // STEUERUNG (Control)
  // =====================
  "control.wait": (
    de: "warte {duration} Sekunden",
    en: "wait {duration} seconds",
    shape: "stack",
    category: "steuerung",
  ),
  "control.repeat": (
    de: "wiederhole {times} mal",
    en: "repeat {times}",
    shape: "c-block",
    category: "steuerung",
  ),
  "control.forever": (
    de: "wiederhole fortlaufend",
    en: "forever",
    shape: "cap",
    category: "steuerung",
  ),
  "control.if": (
    de: "falls {condition}",
    en: "if {condition} then",
    shape: "c-block",
    category: "steuerung",
  ),
  "control.if_else": (
    de: "falls {condition}, sonst",
    en: "if {condition} then, else",
    shape: "c-block",
    category: "steuerung",
  ),
  "control.wait_until": (
    de: "warte bis {condition}",
    en: "wait until {condition}",
    shape: "stack",
    category: "steuerung",
  ),
  "control.repeat_until": (
    de: "wiederhole bis {condition}",
    en: "repeat until {condition}",
    shape: "c-block",
    category: "steuerung",
  ),
  "control.stop": (
    de: "stoppe {option}",
    en: "stop {option}",
    shape: "cap",
    category: "steuerung",
  ),
  "control.start_as_clone": (
    de: "Wenn ich als Klon entstehe",
    en: "when I start as a clone",
    shape: "hat",
    category: "steuerung",
  ),
  "control.create_clone_of": (
    de: "erzeuge Klon von {clone}",
    en: "create clone of {clone}",
    shape: "stack",
    category: "steuerung",
  ),
  "control.delete_this_clone": (
    de: "lösche diesen Klon",
    en: "delete this clone",
    shape: "cap",
    category: "steuerung",
  ),

  // =====================
  // FÜHLEN (Sensing)
  // =====================
  "sensing.touching_object": (
    de: "wird {object} berührt?",
    en: "touching {object} ?",
    shape: "boolean",
    category: "fühlen",
  ),
  "sensing.touching_color": (
    de: "wird Farbe {color} berührt?",
    en: "touching color {color} ?",
    shape: "boolean",
    category: "fühlen",
  ),
  "sensing.color_is_touching_color": (
    de: "Farbe {color1} berührt {color2} ?",
    en: "color {color1} is touching {color2} ?",
    shape: "boolean",
    category: "fühlen",
  ),
  "sensing.distance_to": (
    de: "Entfernung von {object}",
    en: "distance to {object}",
    shape: "reporter",
    category: "fühlen",
  ),
  "sensing.ask_and_wait": (
    de: "frage {question} und warte",
    en: "ask {question} and wait",
    shape: "stack",
    category: "fühlen",
  ),
  "sensing.answer": (
    de: "Antwort",
    en: "answer",
    shape: "reporter",
    category: "fühlen",
  ),
  "sensing.key_pressed": (
    de: "Taste {key} gedrückt?",
    en: "key {key} pressed?",
    shape: "boolean",
    category: "fühlen",
  ),
  "sensing.mouse_down": (
    de: "Maustaste gedrückt?",
    en: "mouse down?",
    shape: "boolean",
    category: "fühlen",
  ),
  "sensing.mouse_x": (
    de: "Maus x-Position",
    en: "mouse x",
    shape: "reporter",
    category: "fühlen",
  ),
  "sensing.mouse_y": (
    de: "Maus y-Position",
    en: "mouse y",
    shape: "reporter",
    category: "fühlen",
  ),
  "sensing.set_drag_mode": (
    de: "setze Ziehbarkeit auf {mode}",
    en: "set drag mode {mode}",
    shape: "stack",
    category: "fühlen",
  ),
  "sensing.loudness": (
    de: "Lautstärke",
    en: "loudness",
    shape: "reporter",
    category: "fühlen",
  ),
  "sensing.timer": (
    de: "Stoppuhr",
    en: "timer",
    shape: "reporter",
    category: "fühlen",
  ),
  "sensing.reset_timer": (
    de: "setze Stoppuhr zurück",
    en: "reset timer",
    shape: "stack",
    category: "fühlen",
  ),
  "sensing.of": (
    de: "{property} von {object}",
    en: "{property} of {object}",
    shape: "reporter",
    category: "fühlen",
  ),
  "sensing.current": (
    de: "aktuell {timeunit}",
    en: "current {timeunit}",
    shape: "reporter",
    category: "fühlen",
  ),
  "sensing.days_since_2000": (
    de: "Tage seit 2000",
    en: "days since 2000",
    shape: "reporter",
    category: "fühlen",
  ),
  "sensing.username": (
    de: "Benutzername",
    en: "username",
    shape: "reporter",
    category: "fühlen",
  ),

  // =====================
  // OPERATOREN (Operators)
  // =====================
  "operator.add": (
    de: "{num1} + {num2}",
    en: "{num1} + {num2}",
    shape: "reporter",
    category: "operatoren",
  ),
  "operator.subtract": (
    de: "{num1} - {num2}",
    en: "{num1} - {num2}",
    shape: "reporter",
    category: "operatoren",
  ),
  "operator.multiply": (
    de: "{num1} * {num2}",
    en: "{num1} * {num2}",
    shape: "reporter",
    category: "operatoren",
  ),
  "operator.divide": (
    de: "{num1} / {num2}",
    en: "{num1} / {num2}",
    shape: "reporter",
    category: "operatoren",
  ),
  "operator.random": (
    de: "Zufallszahl von {from} bis {to}",
    en: "pick random {from} to {to}",
    shape: "reporter",
    category: "operatoren",
  ),
  "operator.gt": (
    de: "{operand1} > {operand2}",
    en: "{operand1} > {operand2}",
    shape: "boolean",
    category: "operatoren",
  ),
  "operator.lt": (
    de: "{operand1} < {operand2}",
    en: "{operand1} < {operand2}",
    shape: "boolean",
    category: "operatoren",
  ),
  "operator.equals": (
    de: "{operand1} = {operand2}",
    en: "{operand1} = {operand2}",
    shape: "boolean",
    category: "operatoren",
  ),
  "operator.and": (
    de: "{operand1} und {operand2}",
    en: "{operand1} and {operand2}",
    shape: "boolean",
    category: "operatoren",
  ),
  "operator.or": (
    de: "{operand1} oder {operand2}",
    en: "{operand1} or {operand2}",
    shape: "boolean",
    category: "operatoren",
  ),
  "operator.not": (
    de: "nicht {operand}",
    en: "not {operand}",
    shape: "boolean",
    category: "operatoren",
  ),
  "operator.join": (
    de: "verbinde {string1} und {string2}",
    en: "join {string1} {string2}",
    shape: "reporter",
    category: "operatoren",
  ),
  "operator.letter_of": (
    de: "Zeichen {letter} von {string}",
    en: "letter {letter} of {string}",
    shape: "reporter",
    category: "operatoren",
  ),
  "operator.length": (
    de: "Länge von {string}",
    en: "length of {string}",
    shape: "reporter",
    category: "operatoren",
  ),
  "operator.contains": (
    de: "{string1} enthält {string2} ?",
    en: "{string1} contains {string2} ?",
    shape: "boolean",
    category: "operatoren",
  ),
  "operator.mod": (
    de: "{num1} mod {num2}",
    en: "{num1} mod {num2}",
    shape: "reporter",
    category: "operatoren",
  ),
  "operator.round": (
    de: "runde {num}",
    en: "round {num}",
    shape: "reporter",
    category: "operatoren",
  ),
  "operator.mathop": (
    de: "{operator} von {num}",
    en: "{operator} of {num}",
    shape: "reporter",
    category: "operatoren",
  ),

  // =====================
  // VARIABLEN (Variables)
  // =====================
  "data.set_variable_to": (
    de: "setze {variable} auf {value}",
    en: "set {variable} to {value}",
    shape: "stack",
    category: "variablen",
  ),
  "data.change_variable_by": (
    de: "ändere {variable} um {value}",
    en: "change {variable} by {value}",
    shape: "stack",
    category: "variablen",
  ),
  "data.show_variable": (
    de: "zeige Variable {variable}",
    en: "show variable {variable}",
    shape: "stack",
    category: "variablen",
  ),
  "data.hide_variable": (
    de: "verstecke Variable {variable}",
    en: "hide variable {variable}",
    shape: "stack",
    category: "variablen",
  ),

  // =====================
  // LISTEN (Lists)
  // =====================
  "data.add_to_list": (
    de: "füge {item} zu {list} hinzu",
    en: "add {item} to {list}",
    shape: "stack",
    category: "listen",
  ),
  "data.delete_of_list": (
    de: "entferne {index} aus {list}",
    en: "delete {index} of {list}",
    shape: "stack",
    category: "listen",
  ),
  "data.delete_all_of_list": (
    de: "entferne alles aus {list}",
    en: "delete all of {list}",
    shape: "stack",
    category: "listen",
  ),
  "data.insert_at_list": (
    de: "füge {item} bei {index} in {list} ein",
    en: "insert {item} at {index} of {list}",
    shape: "stack",
    category: "listen",
  ),
  "data.replace_item_of_list": (
    de: "ersetze Element {index} von {list} durch {item}",
    en: "replace item {index} of {list} with {item}",
    shape: "stack",
    category: "listen",
  ),
  "data.item_of_list": (
    de: "Element {index} von {list}",
    en: "item {index} of {list}",
    shape: "reporter",
    category: "listen",
  ),
  "data.item_number_of_list": (
    de: "Nummer von {item} in {list}",
    en: "item # of {item} in {list}",
    shape: "reporter",
    category: "listen",
  ),
  "data.length_of_list": (
    de: "Länge von {list}",
    en: "length of {list}",
    shape: "reporter",
    category: "listen",
  ),
  "data.list_contains_item": (
    de: "{list} enthält {item} ?",
    en: "{list} contains {item} ?",
    shape: "boolean",
    category: "listen",
  ),
  "data.show_list": (
    de: "zeige Liste {list}",
    en: "show list {list}",
    shape: "stack",
    category: "listen",
  ),
  "data.hide_list": (
    de: "verstecke Liste {list}",
    en: "hide list {list}",
    shape: "stack",
    category: "listen",
  ),

  // =====================
  // EIGENE BLÖCKE (Custom Blocks)
  // =====================
  "custom.input": (
    de: "{text}",
    en: "{text}",
    shape: "input",
    category: "eigene",
  ),
  "custom.block": (
    de: "{label}",
    en: "{label}",
    shape: "stack",
    category: "eigene",
  ),
  "custom.define": (
    de: "Definiere {label}",
    en: "define {label}",
    shape: "define",
    category: "eigene",
  ),
)
