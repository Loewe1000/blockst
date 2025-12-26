// lang/fr.typ — Alias français (Compléts)
// Tous les blocs Scratch avec des noms français (et anglais)

#import "../core.typ": block
#import "../scratch.typ": eigener-block as eigener-block-alt, definiere-fr as definiere-alt, parameter

// =====================
// Evénements
// =====================

#let when-flag-clicked(body) = block(
  "event.when_flag_clicked",
  args: (:),
  lang-code: "fr",
  body: body,
)
#let quand-drapeau(body) = block(
  "event.when_flag_clicked",
  args: (:),
  lang-code: "fr",
  body: body,
)

#let when-key-pressed(key, body) = block(
  "event.when_key_pressed",
  args: (key: key),
  lang-code: "fr",
  body: body,
)
#let quand-touche(key, body) = block(
  "event.when_key_pressed",
  args: (key: key),
  lang-code: "fr",
  body: body,
)

#let when-sprite-clicked(body) = block(
  "event.when_sprite_clicked",
  args: (:),
  lang-code: "fr",
  body: body,
)
#let quand-sprite(body) = block(
  "event.when_sprite_clicked",
  args: (:),
  lang-code: "fr",
  body: body,
)

#let when-backdrop-switches(backdrop, body) = block(
  "event.when_scene_starts",
  args: (scene: backdrop),
  lang-code: "fr",
  body: body,
)
#let quand-arriere-plan(backdrop, body) = block(
  "event.when_scene_starts",
  args: (scene: backdrop),
  lang-code: "fr",
  body: body,
)

#let when-exceeds(element, value, body) = block(
  "event.when_value_exceeds",
  args: (element: element, value: value),
  lang-code: "fr",
  body: body,
)
#let quand-depasse(element, value, body) = block(
  "event.when_value_exceeds",
  args: (element: element, value: value),
  lang-code: "fr",
  body: body,
)

#let when-message-received(message, body) = block(
  "event.when_message_received",
  args: (message1: message),
  lang-code: "fr",
  body: body,
)
#let quand-message(message, body) = block(
  "event.when_message_received",
  args: (message1: message),
  lang-code: "fr",
  body: body,
)

#let broadcast(message) = block(
  "event.broadcast",
  args: (message2: message),
  lang-code: "fr",
)
#let envoyer(message) = block(
  "event.broadcast",
  args: (message2: message),
  lang-code: "fr",
)

#let broadcast-and-wait(message) = block(
  "event.broadcast_and_wait",
  args: (message2: message),
  lang-code: "fr",
)
#let envoyer-et-attendre(message) = block(
  "event.broadcast_and_wait",
  args: (message2: message),
  lang-code: "fr",
)

// =====================
// Mouvement
// =====================

#let move(val: 10) = block(
  "motion.move_steps",
  args: (steps: val),
  lang-code: "fr",
)
#let avancer(val: 10) = block(
  "motion.move_steps",
  args: (steps: val),
  lang-code: "fr",
)

#let turn-right(val: 15) = block(
  "motion.turn_right",
  args: (degrees: val),
  lang-code: "fr",
)
#let tourner-à-droite(val: 15) = block(
  "motion.turn_right",
  args: (degrees: val),
  lang-code: "fr",
)

#let turn-left(val: 15) = block(
  "motion.turn_left",
  args: (degrees: val),
  lang-code: "fr",
)
#let tourner-à-gauche(val: 15) = block(
  "motion.turn_left",
  args: (degrees: val),
  lang-code: "fr",
)

#let goto(to) = block(
  "motion.goto",
  args: (to: to),
  lang-code: "fr",
)
#let aller(to) = block(
  "motion.goto",
  args: (to: to),
  lang-code: "fr",
)

#let goto-xy(x: 0, y: 0) = block(
  "motion.goto_xy",
  args: (x: x, y: y),
  lang-code: "fr",
)
#let aller-xy(x: 0, y: 0) = block(
  "motion.goto_xy",
  args: (x: x, y: y),
  lang-code: "fr",
)

#let glide(val: 1, to) = block(
  "motion.glide",
  args: (secs: val, to: to),
  lang-code: "fr",
)
#let glisser(val: 1, to) = block(
  "motion.glide",
  args: (secs: val, to: to),
  lang-code: "fr",
)

#let glide-to-xy(val: 1, x: 0, y: 0) = block(
  "motion.glide_to_xy",
  args: (secs: val, x: x, y: y),
  lang-code: "fr",
)
#let glisser-xy(val: 1, x: 0, y: 0) = block(
  "motion.glide_to_xy",
  args: (secs: val, x: x, y: y),
  lang-code: "fr",
)

#let point-in-direction(val: 90) = block(
  "motion.point_in_direction",
  args: (direction: val),
  lang-code: "fr",
)
#let orienter-à(val: 90) = block(
  "motion.point_in_direction",
  args: (direction: val),
  lang-code: "fr",
)

#let point-towards(towards) = block(
  "motion.point_towards",
  args: (towards: towards),
  lang-code: "fr",
)
#let orienter-vers(towards) = block(
  "motion.point_towards",
  args: (towards: towards),
  lang-code: "fr",
)

#let change-x(x: 10) = block(
  "motion.change_x",
  args: (dx: x),
  lang-code: "fr",
)
#let ajouter-x(x: 10) = block(
  "motion.change_x",
  args: (dx: x),
  lang-code: "fr",
)

#let set-x(x: 0) = block(
  "motion.set_x",
  args: (x: x),
  lang-code: "fr",
)
#let mettre-x(x: 0) = block(
  "motion.set_x",
  args: (x: x),
  lang-code: "fr",
)

#let change-y(y: 10) = block(
  "motion.change_y",
  args: (dy: y),
  lang-code: "fr",
)
#let ajouter-y(y: 10) = block(
  "motion.change_y",
  args: (dy: y),
  lang-code: "fr",
)

#let set-y(y: 0) = block(
  "motion.set_y",
  args: (y: y),
  lang-code: "fr",
)
#let mettre-y(y: 0) = block(
  "motion.set_y",
  args: (y: y),
  lang-code: "fr",
)

#let if-on-edge-bounce() = block(
  "motion.if_on_edge_bounce",
  args: (:),
  lang-code: "fr",
)
#let rebondir() = block(
  "motion.if_on_edge_bounce",
  args: (:),
  lang-code: "fr",
)

#let set-rotation-style(style) = block(
  "motion.set_rotation_style",
  args: (style: style),
  lang-code: "fr",
)
#let sens-rotation(sens) = block(
  "motion.set_rotation_style",
  args: (style: sens),
  lang-code: "fr",
)

#let x-position() = block(
  "motion.x_position",
  args: (:),
  lang-code: "fr",
)
#let abscisse() = block(
  "motion.x_position",
  args: (:),
  lang-code: "fr",
)

#let y-position() = block(
  "motion.y_position",
  args: (:),
  lang-code: "fr",
)
#let ordonnée() = block(
  "motion.y_position",
  args: (:),
  lang-code: "fr",
)

#let direction() = block(
  "motion.direction",
  args: (:),
  lang-code: "fr",
)

// =====================
// Apparence
// =====================

#let say-for-secs(message, val: 2) = block(
  "looks.say_for_secs",
  args: (message: message, secs: val),
  lang-code: "fr",
)
#let dire-pendant(message, val: 2) = block(
  "looks.say_for_secs",
  args: (message: message, secs: val),
  lang-code: "fr",
)

#let say(message) = block(
  "looks.say",
  args: (message: message),
  lang-code: "fr",
)
#let dire(message) = block(
  "looks.say",
  args: (message: message),
  lang-code: "fr",
)

#let think-for-secs(message, val: 2) = block(
  "looks.think_for_secs",
  args: (message: message, secs: val),
  lang-code: "fr",
)
#let penser-pendant(message, val: 2) = block(
  "looks.think_for_secs",
  args: (message: message, secs: val),
  lang-code: "fr",
)

#let think(message) = block(
  "looks.think",
  args: (message: message),
  lang-code: "fr",
)
#let penser(message) = block(
  "looks.think",
  args: (message: message),
  lang-code: "fr",
)

#let switch-costume-to(costume) = block(
  "looks.switch_costume_to",
  args: (costume: costume),
  lang-code: "fr",
)
#let changer-costume(costume) = block(
  "looks.switch_costume_to",
  args: (costume: costume),
  lang-code: "fr",
)

#let next-costume() = block(
  "looks.next_costume",
  args: (:),
  lang-code: "fr",
)
#let costume-suivant() = block(
  "looks.next_costume",
  args: (:),
  lang-code: "fr",
)

#let switch-backdrop-to(backdrop) = block(
  "looks.switch_backdrop_to",
  args: (backdrop: backdrop),
  lang-code: "fr",
)
#let changer-arrière-plan(backdrop) = block(
  "looks.switch_backdrop_to",
  args: (backdrop: backdrop),
  lang-code: "fr",
)

#let next-backdrop() = block(
  "looks.next_backdrop",
  args: (:),
  lang-code: "fr",
)
#let arrière-plan-suivant() = block(
  "looks.next_backdrop",
  args: (:),
  lang-code: "fr",
)

#let change-size-by(val: 10) = block(
  "looks.change_size_by",
  args: (change: val),
  lang-code: "fr",
)
#let changer-taille(val: 10) = block(
  "looks.change_size_by",
  args: (change: val),
  lang-code: "fr",
)

#let set-size-to(val: 100) = block(
  "looks.set_size_to",
  args: (size: size),
  lang-code: "fr",
)
#let mettre-taille(val: 100) = block(
  "looks.set_size_to",
  args: (size: val),
  lang-code: "fr",
)

#let change-effect-by(effect, val: 25) = block(
  "looks.change_effect_by",
  args: (effect: effect, change: val),
  lang-code: "fr",
)
#let ajouter-effet(effect, val: 25) = block(
  "looks.change_effect_by",
  args: (effect: effect, change: val),
  lang-code: "fr",
)

#let set-effect-to(effect, val: 0) = block(
  "looks.set_effect_to",
  args: (effect: effect, value: val),
  lang-code: "fr",
)
#let mettre-effet(effect, val: 0) = block(
  "looks.set_effect_to",
  args: (effect: effect, value: val),
  lang-code: "fr",
)

#let clear-graphic-effects() = block(
  "looks.clear_graphic_effects",
  args: (:),
  lang-code: "fr",
)
#let annuler-effets() = block(
  "looks.clear_graphic_effects",
  args: (:),
  lang-code: "fr",
)

#let show-sprite() = block(
  "looks.show",
  args: (:),
  lang-code: "fr",
)
#let montrer() = block(
  "looks.show",
  args: (:),
  lang-code: "fr",
)

#let hide-sprite() = block(
  "looks.hide",
  args: (:),
  lang-code: "fr",
)
#let cacher() = block(
  "looks.hide",
  args: (:),
  lang-code: "fr",
)

#let goto-layer(layer) = block(
  "looks.goto_front_back",
  args: (layer: layer),
  lang-code: "fr",
)
#let aller-au-plan(layer) = block(
  "looks.goto_front_back",
  args: (layer: layer),
  lang-code: "fr",
)

#let go-layers(val: 1, direction) = block(
  "looks.go_forward_backward_layers",
  args: (num: val, direction: direction),
  lang-code: "fr",
)
#let déplacer-de-plan(val: 1, direction) = block(
  "looks.go_forward_backward_layers",
  args: (num: val, direction: direction),
  lang-code: "fr",
)

#let costume-property(property) = block(
  "looks.costume_number_name",
  args: (property: property),
  lang-code: "fr",
)
#let numéro-costume(property) = block(
  "looks.costume_number_name",
  args: (property: property),
  lang-code: "fr",
)

#let backdrop-property(property) = block(
  "looks.backdrop_number_name",
  args: (property: property),
  lang-code: "fr",
)
#let numéro-arrière-plan(property) = block(
  "looks.backdrop_number_name",
  args: (property: property),
  lang-code: "fr",
)

#let size() = block(
  "looks.size",
  args: (:),
  lang-code: "fr",
)
#let taille() = block(
  "looks.size",
  args: (:),
  lang-code: "fr",
)

// =====================
// Son
// =====================

#let play-sound-until-done(sound) = block(
  "sound.play_until_done",
  args: (sound: sound),
  lang-code: "fr",
)
#let jouer-bout(sound) = block(
  "sound.play_until_done",
  args: (sound: sound),
  lang-code: "fr",
)

#let start-sound(sound) = block(
  "sound.start_sound",
  args: (sound: sound),
  lang-code: "fr",
)
#let jouer(sound) = block(
  "sound.start_sound",
  args: (sound: sound),
  lang-code: "fr",
)

#let stop-all-sounds() = block(
  "sound.stop_all_sounds",
  args: (:),
  lang-code: "fr",
)
#let arreter-sons() = block(
  "sound.stop_all_sounds",
  args: (:),
  lang-code: "fr",
)

#let change-sound-effect-by(effect, val: 10) = block(
  "sound.change_effect_by",
  args: (effect: effect, value: val),
  lang-code: "fr",
)
#let ajouter-effet-son(effect, val: 10) = block(
  "sound.change_effect_by",
  args: (effect: effect, value: val),
  lang-code: "fr",
)

#let set-sound-effect-to(effect, val: 100) = block(
  "sound.set_effect_to",
  args: (effect: effect, value: val),
  lang-code: "fr",
)
#let mettre-effet-son(effect, val: 100) = block(
  "sound.set_effect_to",
  args: (effect: effect, value: val),
  lang-code: "fr",
)

#let clear-sound-effects() = block(
  "sound.clear_effects",
  args: (:),
  lang-code: "fr",
)
#let annuler-sons() = block(
  "sound.clear_effects",
  args: (:),
  lang-code: "fr",
)

#let change-volume-by(val: 10) = block(
  "sound.change_volume_by",
  args: (volume: val),
  lang-code: "fr",
)
#let ajouter-volume(val: 10) = block(
  "sound.change_volume_by",
  args: (volume: val),
  lang-code: "fr",
)

#let set-volume-to(val: 100) = block(
  "sound.set_volume_to",
  args: (volume: val),
  lang-code: "fr",
)
#let mettre-volume(val: 100) = block(
  "sound.set_volume_to",
  args: (volume: val),
  lang-code: "fr",
)

#let volume() = block(
  "sound.volume",
  args: (:),
  lang-code: "fr",
)

// =====================
// Contrôle
// =====================

#let wait(val: 1) = block(
  "control.wait",
  args: (duration: val),
  lang-code: "fr",
)
#let attendre(val: 1) = block(
  "control.wait",
  args: (duration: val),
  lang-code: "fr",
)

#let repeat(val: 10, body) = block(
  "control.repeat",
  args: (times: val),
  lang-code: "fr",
  body: body,
)
#let répéter(val: 10, body) = block(
  "control.repeat",
  args: (times: val),
  lang-code: "fr",
  body: body,
)

#let forever(body) = block(
  "control.forever",
  args: (:),
  lang-code: "fr",
  body: body,
)
#let indéfiniment(body) = block(
  "control.forever",
  args: (:),
  lang-code: "fr",
  body: body,
)

#let if-then(condition, body) = block(
  "control.if",
  args: (condition: condition),
  lang-code: "fr",
  body: body,
)
#let si-alors(condition, body) = block(
  "control.if",
  args: (condition: condition),
  lang-code: "fr",
  body: body,
)

#let if-then-else(condition, then-body, else-body) = block(
  "control.if_else",
  args: (condition: condition),
  lang-code: "fr",
  body: then-body,
  else-body: else-body,
)
#let si-alors-sinon(condition, then-body, else-body) = block(
  "control.if_else",
  args: (condition: condition),
  lang-code: "fr",
  body: then-body,
  else-body: else-body,
)

#let wait-until(condition) = block(
  "control.wait_until",
  args: (condition: condition),
  lang-code: "fr",
)
#let attendre-que(condition) = block(
  "control.wait_until",
  args: (condition: condition),
  lang-code: "fr",
)

#let repeat-until(condition, body) = block(
  "control.repeat_until",
  args: (condition: condition),
  lang-code: "fr",
  body: body,
)
#let tant-que(condition, body) = block(
  "control.repeat_until",
  args: (condition: condition),
  lang-code: "fr",
  body: body,
)

#let stop(option) = block(
  "control.stop",
  args: (option: option),
  lang-code: "fr",
)

#let when-i-start-as-clone(body) = block(
  "control.start_as_clone",
  args: (:),
  lang-code: "fr",
  body: body,
)
#let quand-clone(body) = block(
  "control.start_as_clone",
  args: (:),
  lang-code: "fr",
  body: body,
)

#let create-clone-of(clone) = block(
  "control.create_clone_of",
  args: (clone: clone),
  lang-code: "fr",
)
#let créer-clone(clone) = block(
  "control.create_clone_of",
  args: (clone: clone),
  lang-code: "fr",
)

#let delete-this-clone() = block(
  "control.delete_this_clone",
  args: (:),
  lang-code: "fr",
)
#let supprimer-clone() = block(
  "control.delete_this_clone",
  args: (:),
  lang-code: "fr",
)

// =====================
// Capteurs
// =====================

#let touching-object(object) = block(
  "sensing.touching_object",
  args: (object: object),
  lang-code: "fr",
)
#let toucher-objet(object) = block(
  "sensing.touching_object",
  args: (object: object),
  lang-code: "fr",
)

#let touching-color(color) = block(
  "sensing.touching_color",
  args: (color: color),
  lang-code: "fr",
)
#let toucher-couleur(color) = block(
  "sensing.touching_color",
  args: (color: color),
  lang-code: "fr",
)

#let color-is-touching-color(color1, color2) = block(
  "sensing.color_is_touching_color",
  args: (color1: color1, color2: color2),
  lang-code: "fr",
)
#let couleurs-se-touchent(color1, color2) = block(
  "sensing.color_is_touching_color",
  args: (color1: color1, color2: color2),
  lang-code: "fr",
)

#let distance-to(object) = block(
  "sensing.distance_to",
  args: (object: object),
  lang-code: "fr",
)
#let distance-de(object) = block(
  "sensing.distance_to",
  args: (object: object),
  lang-code: "fr",
)

#let ask-and-wait(question) = block(
  "sensing.ask_and_wait",
  args: (question: question),
  lang-code: "fr",
)
#let demander(question) = block(
  "sensing.ask_and_wait",
  args: (question: question),
  lang-code: "fr",
)

#let answer() = block(
  "sensing.answer",
  args: (:),
  lang-code: "fr",
)
#let réponse() = block(
  "sensing.answer",
  args: (:),
  lang-code: "fr",
)

#let key-pressed(key) = block(
  "sensing.key_pressed",
  args: (key2: key),
  lang-code: "fr",
)
#let touche(key) = block(
  "sensing.key_pressed",
  args: (key2: key),
  lang-code: "fr",
)

#let mouse-down() = block(
  "sensing.mouse_down",
  args: (:),
  lang-code: "fr",
)
#let souris-pressée() = block(
  "sensing.mouse_down",
  args: (:),
  lang-code: "fr",
)

#let mouse-x() = block(
  "sensing.mouse_x",
  args: (:),
  lang-code: "fr",
)
#let souris-x() = block(
  "sensing.mouse_x",
  args: (:),
  lang-code: "fr",
)

#let mouse-y() = block(
  "sensing.mouse_y",
  args: (:),
  lang-code: "fr",
)
#let souris-y() = block(
  "sensing.mouse_y",
  args: (:),
  lang-code: "fr",
)

#let set-drag-mode(mode) = block(
  "sensing.set_drag_mode",
  args: (mode: mode),
  lang-code: "fr",
)
#let mettre-glissement(mode) = block(
  "sensing.set_drag_mode",
  args: (mode: mode),
  lang-code: "fr",
)

#let loudness() = block(
  "sensing.loudness",
  args: (:),
  lang-code: "fr",
)
#let volume-sonore() = block(
  "sensing.loudness",
  args: (:),
  lang-code: "fr",
)

#let timer() = block(
  "sensing.timer",
  args: (:),
  lang-code: "fr",
)
#let chrono() = block(
  "sensing.timer",
  args: (:),
  lang-code: "fr",
)

#let reset-timer() = block(
  "sensing.reset_timer",
  args: (:),
  lang-code: "fr",
)
#let reinitialiser-chrono() = block(
  "sensing.reset_timer",
  args: (:),
  lang-code: "fr",
)

#let property-of(property, object) = block(
  "sensing.of",
  args: (property: property, object: object),
  lang-code: "fr",
)
#let propriété-scène(property, object) = block(
  "sensing.of",
  args: (property: property, object: object),
  lang-code: "fr",
)

#let current(timeunit) = block(
  "sensing.current",
  args: (timeunit: timeunit),
  lang-code: "fr",
)
#let temps-actuel(timeunit) = block(
  "sensing.current",
  args: (timeunit: timeunit),
  lang-code: "fr",
)

#let days-since-2000() = block(
  "sensing.days_since_2000",
  args: (:),
  lang-code: "fr",
)
#let jours-depuis-2000() = block(
  "sensing.days_since_2000",
  args: (:),
  lang-code: "fr",
)

#let username() = block(
  "sensing.username",
  args: (:),
  lang-code: "fr",
)
#let nom-utilisateur() = block(
  "sensing.username",
  args: (:),
  lang-code: "fr",
)

// =====================
// Opérateurs
// =====================

#let add(num1, num2) = block(
  "operator.add",
  args: (num1: num1, num2: num2),
  lang-code: "fr",
)
#let addition(num1, num2) = block(
  "operator.add",
  args: (num1: num1, num2: num2),
  lang-code: "fr",
)

#let subtract(num1, num2) = block(
  "operator.subtract",
  args: (num1: num1, num2: num2),
  lang-code: "fr",
)
#let soustraction(num1, num2) = block(
  "operator.subtract",
  args: (num1: num1, num2: num2),
  lang-code: "fr",
)

#let multiply(num1, num2) = block(
  "operator.multiply",
  args: (num1: num1, num2: num2),
  lang-code: "fr",
)
#let multiplication(num1, num2) = block(
  "operator.multiply",
  args: (num1: num1, num2: num2),
  lang-code: "fr",
)

#let divide(num1, num2) = block(
  "operator.divide",
  args: (num1: num1, num2: num2),
  lang-code: "fr",
)
#let division(num1, num2) = block(
  "operator.divide",
  args: (num1: num1, num2: num2),
  lang-code: "fr",
)

#let pick-random(from: 1, to: 10) = block(
  "operator.random",
  args: (from: from, to2: to),
  lang-code: "fr",
)
#let aléatoire(de: 1, à: 10) = block(
  "operator.random",
  args: (from: de, to2: à),
  lang-code: "fr",
)

#let greater-than(operand1, operand2) = block(
  "operator.gt",
  args: (operand1: operand1, operand2: operand2),
  lang-code: "fr",
)
#let supérieur(operand1, operand2) = block(
  "operator.gt",
  args: (operand1: operand1, operand2: operand2),
  lang-code: "fr",
)

#let less-than(operand1, operand2) = block(
  "operator.lt",
  args: (operand1: operand1, operand2: operand2),
  lang-code: "fr",
)
#let inférieur(operand1, operand2) = block(
  "operator.lt",
  args: (operand1: operand1, operand2: operand2),
  lang-code: "fr",
)

#let equals(operand1, operand2) = block(
  "operator.equals",
  args: (operand1: operand1, operand2: operand2),
  lang-code: "fr",
)
#let égal(operand1, operand2) = block(
  "operator.equals",
  args: (operand1: operand1, operand2: operand2),
  lang-code: "fr",
)

#let op-and(operand1, operand2) = block(
  "operator.and",
  args: (operand1: operand1, operand2: operand2),
  lang-code: "fr",
)
#let intersection(operand1, operand2) = block(
  "operator.and",
  args: (operand1: operand1, operand2: operand2),
  lang-code: "fr",
)

#let op-or(operand1, operand2) = block(
  "operator.or",
  args: (operand1: operand1, operand2: operand2),
  lang-code: "fr",
)
#let union(operand1, operand2) = block(
  "operator.or",
  args: (operand1: operand1, operand2: operand2),
  lang-code: "fr",
)

#let op-not(operand) = block(
  "operator.not",
  args: (operand: operand),
  lang-code: "fr",
)
#let contraire(operand) = block(
  "operator.not",
  args: (operand: operand),
  lang-code: "fr",
)

#let join(string1, string2) = block(
  "operator.join",
  args: (string1: string1, string2: string2),
  lang-code: "fr",
)
#let regrouper(string1, string2) = block(
  "operator.join",
  args: (string1: string1, string2: string2),
  lang-code: "fr",
)

#let letter-of(letter, string) = block(
  "operator.letter_of",
  args: (letter: letter, string: string),
  lang-code: "fr",
)
#let lettres(letter, string) = block(
  "operator.letter_of",
  args: (letter: letter, string: string),
  lang-code: "fr",
)

#let length-of(string) = block(
  "operator.length",
  args: (string: string),
  lang-code: "fr",
)
#let longueur(string) = block(
  "operator.length",
  args: (string: string),
  lang-code: "fr",
)

#let contains(string1, string2) = block(
  "operator.contains",
  args: (string1: string1, string2: string2),
  lang-code: "fr",
)
#let contient(string1, string2) = block(
  "operator.contains",
  args: (string1: string1, string2: string2),
  lang-code: "fr",
)

#let mod(num1, num2) = block(
  "operator.mod",
  args: (num1: num1, num2: num2),
  lang-code: "fr",
)
#let modulo(num1, num2) = block(
  "operator.mod",
  args: (num1: num1, num2: num2),
  lang-code: "fr",
)

#let round(num) = block(
  "operator.round",
  args: (num: num),
  lang-code: "fr",
)
#let arrondi(num) = block(
  "operator.round",
  args: (num: num),
  lang-code: "fr",
)

#let mathop(operator, num) = block(
  "operator.mathop",
  args: (operator: operator, num: num),
  lang-code: "fr",
)
#let fonction(operator, num) = block(
  "operator.mathop",
  args: (operator: operator, num: num),
  lang-code: "fr",
)

// =====================
// Variables
// =====================

#let set-variable-to(variable, value) = block(
  "data.set_variable_to",
  args: (variable: variable, value: value),
  lang-code: "fr",
)
#let mettre-variable(variable, value) = block(
  "data.set_variable_to",
  args: (variable: variable, value: value),
  lang-code: "fr",
)

#let change-variable-by(variable, value) = block(
  "data.change_variable_by",
  args: (variable: variable, value: value),
  lang-code: "fr",
)
#let ajouter-variable(variable, value) = block(
  "data.change_variable_by",
  args: (variable: variable, value: value),
  lang-code: "fr",
)

#let show-variable(variable) = block(
  "data.show_variable",
  args: (variable: variable),
  lang-code: "fr",
)
#let montrer-variable(variable) = block(
  "data.show_variable",
  args: (variable: variable),
  lang-code: "fr",
)

#let hide-variable(variable) = block(
  "data.hide_variable",
  args: (variable: variable),
  lang-code: "fr",
)
#let cacher-variable(variable) = block(
  "data.hide_variable",
  args: (variable: variable),
  lang-code: "fr",
)

// =====================
// Listes
// =====================

#let add-to-list(item, list) = block(
  "data.add_to_list",
  args: (item: item, list: list),
  lang-code: "fr",
)
#let ajouter-liste(item, list) = block(
  "data.add_to_list",
  args: (item: item, list: list),
  lang-code: "fr",
)

#let delete-of-list(index, list) = block(
  "data.delete_of_list",
  args: (index: index, list: list),
  lang-code: "fr",
)
#let supprimer-de-liste(index, list) = block(
  "data.delete_of_list",
  args: (index: index, list: list),
  lang-code: "fr",
)

#let delete-all-of-list(list) = block(
  "data.delete_all_of_list",
  args: (list: list),
  lang-code: "fr",
)
#let supprimer-la-liste(list) = block(
  "data.delete_all_of_list",
  args: (list: list),
  lang-code: "fr",
)

#let insert-at-list(item, index, list) = block(
  "data.insert_at_list",
  args: (item: item, index: index, list: list),
  lang-code: "fr",
)
#let insérer(item, index, list) = block(
  "data.insert_at_list",
  args: (item: item, index: index, list: list),
  lang-code: "fr",
)

#let replace-item-of-list(index, list, item) = block(
  "data.replace_item_of_list",
  args: (index: index, list: list, item: item),
  lang-code: "fr",
)
#let remplacer(index, list, item) = block(
  "data.replace_item_of_list",
  args: (index: index, list: list, item: item),
  lang-code: "fr",
)

#let item-of-list(index, list) = block(
  "data.item_of_list",
  args: (index: index, list: list),
  lang-code: "fr",
)
#let élément(index, list) = block(
  "data.item_of_list",
  args: (index: index, list: list),
  lang-code: "fr",
)

#let item-number-of-list(item, list) = block(
  "data.item_number_of_list",
  args: (item: item, list: list),
  lang-code: "fr",
)
#let position(item, list) = block(
  "data.item_number_of_list",
  args: (item: item, list: list),
  lang-code: "fr",
)

#let length-of-list(list) = block(
  "data.length_of_list",
  args: (list: list),
  lang-code: "fr",
)
#let longueur-liste(list) = block(
  "data.length_of_list",
  args: (list: list),
  lang-code: "fr",
)

#let list-contains-item(list, item) = block(
  "data.list_contains_item",
  args: (list: list, item: item),
  lang-code: "fr",
)
#let liste-contient(list, item) = block(
  "data.list_contains_item",
  args: (list: list, item: item),
  lang-code: "fr",
)

#let show-list(list) = block(
  "data.show_list",
  args: (list: list),
  lang-code: "fr",
)
#let montrer-liste(list) = block(
  "data.show_list",
  args: (list: list),
  lang-code: "fr",
)

#let hide-list(list) = block(
  "data.hide_list",
  args: (list: list),
  lang-code: "fr",
)
#let cacher-list(list) = block(
  "data.hide_list",
  args: (list: list),
  lang-code: "fr",
)

// =====================
// Stylo
// =====================


#let pen-erase-all() = block(
  "pen.erase_all",
  args: (:),
  lang-code: "fr",
)
#let effacer-tout() = block(
  "pen.erase_all",
  args: (:),
  lang-code: "fr",
)

#let pen-stamp() = block(
  "pen.stamp",
  args: (:),
  lang-code: "fr",
)
#let estampiller() = block(
  "pen.stamp",
  args: (:),
  lang-code: "fr",
)

#let pen-down() = block(
  "pen.down",
  args: (:),
  lang-code: "fr",
)
#let écrire() = block(
  "pen.down",
  args: (:),
  lang-code: "fr",
)

#let pen-up() = block(
  "pen.up",
  args: (:),
  lang-code: "fr",
)
#let relever() = block(
  "pen.up",
  args: (:),
  lang-code: "fr",
)

#let pen-set-color(color) = block(
  "pen.set_color",
  args: (color:color),
  lang-code: "fr",
)
#let choisir-couleur(color) = block(
  "pen.set_color",
  args: (color:color),
  lang-code: "fr",
)

#let pen-change-component-by(component:"couleur",val:10) = block(
  "pen.change_component_by",
  args: (component:component,value:val),
  lang-code: "fr",
)
#let ajouter-stylo(component:"couleur",val:10) = block(
  "pen.change_component_by",
  args: (component:component,value:val),
  lang-code: "fr",
)

#let pen-set-component-to(component:"couleur",val:50) = block(
  "pen.set_component",
  args: (component:component,value:val),
  lang-code: "fr",
)
#let mettre-stylo(component:"couleur",val:50) = block(
  "pen.set_component",
  args: (component:component,value:val),
  lang-code: "fr",
)

#let pen-change-size-by(val:1) = block(
  "pen.change_size_by",
  args: (value:val),
  lang-code: "fr",
)
#let ajouter-taille(val:1) = block(
  "pen.change_size_by",
  args: (value:val),
  lang-code: "fr",
)

#let pen-set-size-to(val:1) = block(
  "pen.set_size_to",
  args: (value:val),
  lang-code: "fr",
)
#let mettre-taille-stylo(val:1) = block(
  "pen.set_size_to",
  args: (value:val),
  lang-code: "fr",
)

// =====================
// Mes blocs
// =====================

#let custom-input(text) = block(
  "custom.input",
  args: (text: text),
  lang-code: "fr",
)
#let saisie-perso(text) = block(
  "custom.input",
  args: (text: text),
  lang-code: "fr",
)

// Blocs personnels avec paramètres (ancienne API de scratch.typ)
#let custom-block(..args) = eigener-block-alt(..args)
#let bloc-perso(..args) = eigener-block-alt(..args)

// Définir un bloc (ancienne API de scratch.typ)
#let define(label, ..children) = definiere-alt(label, ..children)
#let définir(label, ..children) = definiere-alt(label, ..children)

// Parameter reporter for custom blocks is exported
// (imported directly from scratch.typ and is already available)
