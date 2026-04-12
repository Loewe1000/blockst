// text/profiles.typ — language profile registry for parser normalization

#let _trim(value) = value.trim()

#let _collapse-spaces(value) = {
  let text = value.replace("\t", " ")
  while text.contains("  ") {
    text = text.replace("  ", " ")
  }
  text
}

#let _strip-category-annotations(value) = {
  let text = value
  let categories = (
    "motion",
    "looks",
    "sound",
    "events",
    "control",
    "sensing",
    "operators",
    "variables",
    "lists",
    "pen",
    "custom",
    "grey",
  )

  for category in categories {
    text = text.replace(":: " + category, "")
    text = text.replace(" :: " + category, "")
  }

  _collapse-spaces(text)
}

#let _apply-en-aliases(value) = {
  let text = _strip-category-annotations(value)

  text = text.replace("when @greenFlag clicked", "when flag clicked")
  text = text.replace("when ⚑ clicked", "when flag clicked")
  text = text.replace("when gf clicked", "when flag clicked")

  text = text.replace("turn @turnRight ", "turn right ")
  text = text.replace("turn @turnLeft ", "turn left ")
  text = text.replace("turn ↻ ", "turn right ")
  text = text.replace("turn ↺ ", "turn left ")
  text = text.replace("turn cw ", "turn right ")
  text = text.replace("turn ccw ", "turn left ")

  text = text.replace(" seconds", " secs")
  text = text.replace(")%", ") %")

  text = text.replace("switch to costume ", "switch costume to ")
  text = text.replace("switch to background ", "switch backdrop to ")
  text = text.replace("next background", "next backdrop")
  text = text.replace("when Sprite1 clicked", "when this sprite clicked")

  if text == "stop script" {
    text = "stop [this script v]"
  }
  if text == "stop all" {
    text = "stop [all v]"
  }

  if text == "if <>" {
    text = "if <> then"
  }
  if text.starts-with("if <") and not text.ends-with(" then") and not text.contains(", else") {
    text = text + " then"
  }

  text = text.replace("<loud ? >", "<loudness > (10)>")
  text = text.replace("…", "...")

  text = text.replace("?>", " ? >")
  text = text.replace("? >", " ? >")

  text = text.replace("(background #)", "(backdrop #)")
  text = text.replace(". . .", "...")

  if text.starts-with("<") and text.ends-with(">") and text.contains(" contains ") and not text.contains("?") {
    text = text.slice(0, text.len() - 1) + " ? >"
  }

  if text.starts-with("<sensor ") and not text.ends-with(">") {
    text = text + " >"
  }

  _collapse-spaces(text)
}

#let _apply-de-aliases(value) = {
  let text = _strip-category-annotations(value)

  text = text.replace("drehe dich im Uhrzeigersinn um ", "drehe dich nach rechts um ")
  text = text.replace("drehe dich gegen den Uhrzeigersinn um ", "drehe dich nach links um ")
  text = text.replace("gleite in ", "gleite ")
  text = text.replace(" sek. ", " Sek. ")
  text = text.replace(")%", ") %")

  text = text.replace("spiele Ton ", "spiele Note ")
  text = text.replace("pausiere (", "pausiere für (")
  text = text.replace("wische Malspuren weg", "lösche alles")
  text = text.replace("lösche (", "entferne (")
  text = text.replace(" als (", " bei (")
  text = text.replace("(user id)", "(Benutzer-ID)")
  text = text.replace("ziehe Kostüm ", "wechsle zu Kostüm ")
  text = text.replace("wechsle zum Hintergrund ", "wechsle zu Bühnenbild ")
  text = text.replace("nächster Hintergrund", "nächstes Bühnenbild")
  text = text.replace("(Hintergrund Nr.)", "(Bühnenbildnummer)")
  text = text.replace("(Hintergrund Name)", "(Bühnenbildname)")
  text = text.replace("Wenn die grüne Flagge angeklickt", "Wenn [⚑ v] angeklickt wird")
  text = text.replace("Wenn grüne Flagge angeklickt", "Wenn [⚑ v] angeklickt wird")
  text = text.replace("Wenn Flagge angeklickt", "Wenn [⚑ v] angeklickt wird")
  text = text.replace("Wenn ⚑ angeklickt", "Wenn [⚑ v] angeklickt wird")
  text = text.replace("Wenn gf angeklickt", "Wenn [⚑ v] angeklickt wird")

  if text.starts-with("Wenn Taste ") and text.ends-with(" gedrückt") {
    text = text + " wird"
  }

  if text.starts-with("Frage ") and text.ends-with(" und warte") {
    text = "frage " + text.slice("Frage ".len())
  }

  if text == "Wenn ich angeklickt werde" {
    text = "Wenn diese Figur angeklickt wird"
  }

  if text.starts-with("Wenn das Bühnenbild zu ") and text.ends-with(" wechselt") {
    text = "Wenn Bühnenbild zu " + text.slice("Wenn das Bühnenbild zu ".len())
  }

  let looks-effects = (
    "color",
    "fisheye",
    "whirl",
    "pixelate",
    "mosaic",
    "brightness",
    "ghost",
    "Farbe",
    "Fischauge",
    "Wirbel",
    "Pixeln",
    "Mosaik",
    "Helligkeit",
    "Durchsichtigkeit",
    "Transparenz",
  )

  for effect in looks-effects {
    text = text.replace("ändere [" + effect + " v]-Effekt um ", "ändere Effekt [" + effect + " v] um ")
    text = text.replace("setze [" + effect + " v]-Effekt auf ", "setze Effekt [" + effect + " v] auf ")
  }

  text = text.replace("ändere Stiftfarbe um ", "ändere Stift [Farbe v] um ")
  text = text.replace("ändere Farbstärke um ", "ändere Stift [Farbstärke v] um ")
  text = text.replace("setze Farbstärke auf ", "setze Stift [Farbstärke v] auf ")

  if text == "schalte Stift ein" {
    text = "[pen v] schalte Stift ein"
  }
  if text == "schalte Stift aus" {
    text = "[pen v] schalte Stift aus"
  }

  if text.starts-with("wechsle zu Kostüm ") and text.ends-with(" an") {
    text = text.slice(0, text.len() - " an".len())
  }

  if text == "komme nach vorn" {
    text = "gehe zu vorn Ebene"
  }
  if text == "komme nach hinten" {
    text = "gehe zu hinten Ebene"
  }

  if text.starts-with("warte ") and text.ends-with(" Sek.") {
    text = text.slice(0, text.len() - " Sek.".len()) + " Sekunden"
  }

  if text.starts-with("setze Tempo auf (") and text.ends-with(" Schläge/Min.") {
    text = text.slice(0, text.len() - " Schläge/Min.".len()) + " bpm"
  }

  if text.starts-with("setze Richtung auf (") and text.ends-with(")") and not text.ends-with(" Grad") {
    if text.ends-with(" v)") {
      text = text.slice(0, text.len() - " v)".len()) + ")"
    }
    text = text + " Grad"
  }

  if text.starts-with("sage ") and text.contains(" für ") and text.ends-with(" Sek.") {
    text = text.slice(0, text.len() - " Sek.".len()) + " Sekunden"
  }

  if text.starts-with("denke ") and text.contains(" für ") and text.ends-with(" Sek.") {
    text = text.slice(0, text.len() - " Sek.".len()) + " Sekunden"
  }

  if text.starts-with("sage ") and text.contains(" für ") and text.ends-with(" Sekunden.") {
    text = text.slice(0, text.len() - 1)
  }

  if text.starts-with("denke ") and text.contains(" für ") and text.ends-with(" Sekunden.") {
    text = text.slice(0, text.len() - 1)
  }

  if text.starts-with("([") and text.ends-with(" im Moment)") {
    let time-unit = text.slice(1, text.len() - " im Moment)".len())
    text = "(aktuell " + time-unit + ")"
  }

  if text.starts-with("(verbinde ") and text.contains("] [") {
    text = text.replace("] [", "] und [")
  }

  if text.starts-with("((") and text.ends-with(" gerundet)") {
    let rounded-value = text.slice(1, text.len() - " gerundet)".len())
    text = "(runde " + rounded-value + ")"
  }

  if text.starts-with("schalte ") and text.contains(" für (") and text.ends-with(" Sekunden an") {
    text = text.slice(0, text.len() - " Sekunden an".len()) + " Sek. ein"
  }

  if text.starts-with("schalte ") and text.ends-with(" an") {
    text = text.slice(0, text.len() - " an".len()) + " ein"
  }

  if text == "(Abstand)" {
    text = "(Entfernung)"
  }

  if text == "<laut ? >" {
    text = "<Lautstärke > (10)>"
  }

  if text.starts-with("<") and text.ends-with(">") and text.contains(" enthält ") and not text.contains("?") {
    text = text.slice(0, text.len() - 1) + " ? >"
  }

  _collapse-spaces(text)
}

#let _apply-fr-aliases(value) = _collapse-spaces(_strip-category-annotations(value))

#let _PROFILE_EN = (
  lang-code: "en",
  end-marker: "end",
  else-marker: "else",
  line-comment-prefix: "//",
  apply-aliases: _apply-en-aliases,
)

#let _PROFILE_DE = (
  lang-code: "de",
  end-marker: "ende",
  else-marker: "sonst",
  line-comment-prefix: "//",
  apply-aliases: _apply-de-aliases,
)

#let _PROFILE_FR = (
  lang-code: "fr",
  end-marker: "fin",
  else-marker: "sinon",
  line-comment-prefix: "//",
  apply-aliases: _apply-fr-aliases,
)

#let get-language-profile(lang-code) = {
  if lang-code == "de" {
    _PROFILE_DE
  } else if lang-code == "fr" {
    _PROFILE_FR
  } else {
    _PROFILE_EN
  }
}
