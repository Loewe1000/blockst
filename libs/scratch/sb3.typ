// sb3.typ — Typst-side helpers for direct SB3 import via plugin()

#import "text/en.typ": parse-scratch-text as parse-scratch-text-en
#import "text/parser.typ": _render-nodes
#import "core.typ": block as scratch-render-block

#let _error-prefix = "ERROR:"
#let _default-sb3-plugin = plugin("plugins/sb3_wasm.wasm")

#let _check-plugin-output(text) = {
  if text.starts-with(_error-prefix) {
    panic("SB3 plugin failed: " + text)
  }
  text
}

#let _resolve-plugin(sb3-plugin: auto) = {
  if sb3-plugin == auto {
    _default-sb3-plugin
  } else {
    sb3-plugin
  }
}

#let _normalize-language(language) = {
  if type(language) != str {
    panic("sb3 helpers: language must be a string (supported: en, de, fr).")
  }

  let normalized = language.trim()
  if normalized in ("en", "de", "fr") {
    normalized
  } else {
    panic("sb3 helpers: unsupported language `" + language + "` (supported: en, de, fr).")
  }
}

#let _normalize-target(target) = {
  if target == auto {
    return (mode: "all")
  }

  if type(target) != str {
    panic("sb3 helpers: target must be a string or auto.")
  }

  let normalized = target.trim()
  if normalized == "" {
    panic("sb3 helpers: target must not be empty.")
  }

  if normalized in (
    "stage",
    "Stage",
    "scene",
    "Scene",
    "buehne",
    "Buehne",
    "bühne",
    "Bühne",
  ) {
    (mode: "stage")
  } else {
    (mode: "name", value: normalized)
  }
}

#let _matches-target(item, target-filter) = {
  if target-filter.mode == "all" {
    true
  } else if target-filter.mode == "stage" {
    item.is_stage
  } else {
    item.target_name == target-filter.value
  }
}

#let _resolve-show-headers(show-headers, target-filter) = {
  if show-headers == auto {
    target-filter.mode == "all"
  } else if type(show-headers) == bool {
    show-headers
  } else {
    panic("render-sb3-scripts: show-headers must be a boolean or auto.")
  }
}

#let _resolve-show-target-headers(show-target-headers, target-filter) = {
  if show-target-headers == auto {
    target-filter.mode == "all"
  } else if type(show-target-headers) == bool {
    show-target-headers
  } else {
    panic("render-sb3-lists/render-sb3-variables: show-target-headers must be a boolean or auto.")
  }
}

#let _normalize-target-script-number(target-script-number) = {
  if target-script-number == auto {
    return auto
  }

  if type(target-script-number) != int or target-script-number < 1 {
    panic("render-sb3-scripts: target-script-number must be an integer >= 1.")
  }

  target-script-number
}

#let _normalize-target-list-number(target-list-number) = {
  if target-list-number == auto {
    return auto
  }

  if type(target-list-number) != int or target-list-number < 1 {
    panic("render-sb3-lists: target-list-number must be an integer >= 1.")
  }

  target-list-number
}

#let _normalize-target-list-name(target-list-name) = {
  if target-list-name == auto {
    return auto
  }

  if type(target-list-name) != str {
    panic("render-sb3-lists: target-list-name must be a string.")
  }

  let normalized = target-list-name.trim()
  if normalized == "" {
    panic("render-sb3-lists: target-list-name must not be empty.")
  }

  normalized
}

#let _normalize-target-variable-number(target-variable-number) = {
  if target-variable-number == auto {
    return auto
  }

  if type(target-variable-number) != int or target-variable-number < 1 {
    panic("render-sb3-variables: target-variable-number must be an integer >= 1.")
  }

  target-variable-number
}

#let _normalize-target-variable-name(target-variable-name) = {
  if target-variable-name == auto {
    return auto
  }

  if type(target-variable-name) != str {
    panic("render-sb3-variables: target-variable-name must be a string.")
  }

  let normalized = target-variable-name.trim()
  if normalized == "" {
    panic("render-sb3-variables: target-variable-name must not be empty.")
  }

  normalized
}

#let _normalize-include-parser-text(include-parser-text) = {
  if type(include-parser-text) != bool {
    panic("sb3-scripts-catalog: include-parser-text must be a boolean.")
  }
  include-parser-text
}

#let _strip-script-header(script-text) = {
  let normalized = script-text.replace("\r", "")
  let lines = normalized.split("\n")
  if lines.len() == 0 {
    return normalized.trim()
  }

  let first = lines.at(0).trim()
  if first.starts-with("// [") {
    if lines.len() == 1 {
      ""
    } else {
      lines.slice(1).join("\n").trim()
    }
  } else {
    normalized.trim()
  }
}

#let _compact-variables(target) = {
  let out = ()
  let variables = target.at("variables", default: (:))
  for id in variables.keys() {
    let raw = variables.at(id)
    let name = if type(raw) == array and raw.len() > 0 { raw.at(0) } else { "" }
    let value = if type(raw) == array and raw.len() > 1 { raw.at(1) } else { none }
    out.push((
      id: id,
      name: name,
      value: value,
    ))
  }
  out
}

#let _compact-lists(target) = {
  let out = ()
  let lists = target.at("lists", default: (:))
  for id in lists.keys() {
    let raw = lists.at(id)
    let name = if type(raw) == array and raw.len() > 0 { raw.at(0) } else { "" }
    let values = if type(raw) == array and raw.len() > 1 { raw.at(1) } else { () }
    out.push((
      id: id,
      name: name,
      values: values,
    ))
  }
  out
}

#let _compact-target-state(target) = {
  let is-stage = target.at("isStage", default: false)

  let stage-props = if is-stage {
    (
      current_costume: target.at("currentCostume", default: none),
      volume: target.at("volume", default: none),
      tempo: target.at("tempo", default: none),
      video_state: target.at("videoState", default: none),
      video_transparency: target.at("videoTransparency", default: none),
      text_to_speech_language: target.at("textToSpeechLanguage", default: none),
    )
  } else {
    none
  }

  let sprite-props = if not is-stage {
    (
      x: target.at("x", default: none),
      y: target.at("y", default: none),
      direction: target.at("direction", default: none),
      size: target.at("size", default: none),
      visible: target.at("visible", default: none),
      rotation_style: target.at("rotationStyle", default: none),
      draggable: target.at("draggable", default: none),
      current_costume: target.at("currentCostume", default: none),
      volume: target.at("volume", default: none),
    )
  } else {
    none
  }

  (
    name: target.at("name", default: "Unnamed Target"),
    is_stage: is-stage,
    variables: _compact-variables(target),
    lists: _compact-lists(target),
    stage_props: stage-props,
    sprite_props: sprite-props,
  )
}

#let _pick-target-script(filtered-scripts, target-script-number) = {
  let matches = ()
  for item in filtered-scripts {
    if item.local_number == target-script-number {
      matches.push(item)
    }
  }

  if matches.len() == 0 {
    panic("render-sb3-scripts: target-script-number " + str(target-script-number) + " does not exist for the selected target.")
  }

  if matches.len() > 1 {
    panic("render-sb3-scripts: target-script-number is ambiguous for the selected target filter.")
  }

  matches.at(0)
}

#let _group-scripts-by-target(scripts) = {
  let groups = ()

  for item in scripts {
    let matching-groups = ()
    for group in groups {
      if group.target_name == item.target_name and group.is_stage == item.is_stage {
        matching-groups.push(group)
      }
    }

    if matching-groups.len() == 0 {
      groups.push((
        target_name: item.target_name,
        target_kind: item.target_kind,
        is_stage: item.is_stage,
      ))
    }
  }

  let out = ()
  for group in groups {
    let group-scripts = ()
    for item in scripts {
      if item.target_name == group.target_name and item.is_stage == group.is_stage {
        group-scripts.push(item)
      }
    }

    out.push((
      target_name: group.target_name,
      target_kind: group.target_kind,
      is_stage: group.is_stage,
      scripts: group-scripts,
    ))
  }

  out
}

#let _flatten-grouped-scripts(grouped-catalog) = {
  let out = ()
  for group in grouped-catalog {
    for item in group.scripts {
      out.push(item)
    }
  }
  out
}

#let _pick-target-list(target-states, target-list-number) = {
  let matches = ()
  for target-state in target-states {
    let local-number = 1
    for list-item in target-state.lists {
      if local-number == target-list-number {
        matches.push((
          target_state: target-state,
          list_item: list-item,
          local_number: local-number,
        ))
      }
      local-number += 1
    }
  }

  if matches.len() == 0 {
    panic("render-sb3-lists: target-list-number " + str(target-list-number) + " does not exist for the selected target.")
  }

  if matches.len() > 1 {
    panic("render-sb3-lists: target-list-number is ambiguous for the selected target filter.")
  }

  matches.at(0)
}

#let _pick-target-list-by-name(target-states, target-list-name) = {
  let matches = ()
  for target-state in target-states {
    let local-number = 1
    for list-item in target-state.lists {
      if list-item.name == target-list-name {
        matches.push((
          target_state: target-state,
          list_item: list-item,
          local_number: local-number,
        ))
      }
      local-number += 1
    }
  }

  if matches.len() == 0 {
    panic("render-sb3-lists: target-list-name `" + target-list-name + "` does not exist for the selected target.")
  }

  if matches.len() > 1 {
    panic("render-sb3-lists: target-list-name is ambiguous for the selected target filter.")
  }

  matches.at(0)
}

#let _pick-target-variable(target-states, target-variable-number) = {
  let matches = ()
  for target-state in target-states {
    let local-number = 1
    for variable-item in target-state.variables {
      if local-number == target-variable-number {
        matches.push((
          target_state: target-state,
          variable_item: variable-item,
          local_number: local-number,
        ))
      }
      local-number += 1
    }
  }

  if matches.len() == 0 {
    panic("render-sb3-variables: target-variable-number " + str(target-variable-number) + " does not exist for the selected target.")
  }

  if matches.len() > 1 {
    panic("render-sb3-variables: target-variable-number is ambiguous for the selected target filter.")
  }

  matches.at(0)
}

#let _pick-target-variable-by-name(target-states, target-variable-name) = {
  let matches = ()
  for target-state in target-states {
    let local-number = 1
    for variable-item in target-state.variables {
      if variable-item.name == target-variable-name {
        matches.push((
          target_state: target-state,
          variable_item: variable-item,
          local_number: local-number,
        ))
      }
      local-number += 1
    }
  }

  if matches.len() == 0 {
    panic("render-sb3-variables: target-variable-name `" + target-variable-name + "` does not exist for the selected target.")
  }

  if matches.len() > 1 {
    panic("render-sb3-variables: target-variable-name is ambiguous for the selected target filter.")
  }

  matches.at(0)
}

#let _ui-labels(language) = {
  if language == "de" {
    (
      no-scripts: "Keine Top-Level-Skripte in dieser sb3 gefunden.",
      no-target-scripts: "Keine Top-Level-Skripte fuer den gewaehlten Ziel-Filter gefunden.",
      no-lists: "Keine Listen fuer den gewaehlten Ziel-Filter gefunden.",
      no-variables: "Keine Variablen fuer den gewaehlten Ziel-Filter gefunden.",
      stage: "Buehne (Hintergrund)",
      sprite: "Figur",
      script: "Skript",
      list: "Liste",
      variable: "Variable",
      values: "Werte",
      value: "Wert",
    )
  } else if language == "fr" {
    (
      no-scripts: "Aucun script de premier niveau trouve dans ce sb3.",
      no-target-scripts: "Aucun script de premier niveau ne correspond au filtre cible.",
      no-lists: "Aucune liste ne correspond au filtre cible.",
      no-variables: "Aucune variable ne correspond au filtre cible.",
      stage: "Scene (Arriere-plan)",
      sprite: "Lutin",
      script: "Script",
      list: "Liste",
      variable: "Variable",
      values: "Valeurs",
      value: "Valeur",
    )
  } else {
    (
      no-scripts: "No top-level scripts found in this sb3.",
      no-target-scripts: "No top-level scripts match the selected target filter.",
      no-lists: "No lists match the selected target filter.",
      no-variables: "No variables match the selected target filter.",
      stage: "Stage (Backdrop)",
      sprite: "Sprite",
      script: "Script",
      list: "List",
      variable: "Variable",
      values: "Values",
      value: "Value",
    )
  }
}

#let _target-title(target-state, ui) = {
  let target-kind = if target-state.is_stage { ui.stage } else { ui.sprite }
  [#target-kind: #target-state.name]
}

#let _render-scratch-text-localized(text, language) = {
  // SB3 importer currently emits English parser text.
  // Parse in English, then render nodes in the target UI language.
  let nodes = parse-scratch-text-en(text)
  _render-nodes(nodes, language)
}

// Returns metadata for top-level scripts with global numbering.
// Optional target filter: stage or exact target name.
// Optional parsed_text field per item (enabled by default).
// Output format: ((target_name: "...", is_stage: true/false, scripts: ((number: 1, ...), ...)), ...)
#let sb3-scripts-catalog(sb3-bytes, target: auto, include-parser-text: true, sb3-plugin: auto) = {
  let active-plugin = _resolve-plugin(sb3-plugin: sb3-plugin)
  let target-filter = _normalize-target(target)
  let include-parser-text = _normalize-include-parser-text(include-parser-text)
  let text = str(active-plugin.sb3_scripts_catalog_json(sb3-bytes))
  let catalog = json(bytes(_check-plugin-output(text)))

  let filtered = ()
  for item in catalog.scripts {
    if _matches-target(item, target-filter) {
      filtered.push(item)
    }
  }

  if not include-parser-text {
    return _group-scripts-by-target(filtered)
  }

  let enriched = ()
  for item in filtered {
    let parser-text = _strip-script-header(_check-plugin-output(str(
      active-plugin.sb3_to_scratch_text_by_number(sb3-bytes, bytes(str(item.number))),
    )))
    enriched.push((
      number: item.number,
      local_number: item.local_number,
      target_name: item.target_name,
      target_kind: item.target_kind,
      is_stage: item.is_stage,
      parsed_text: parser-text,
    ))
  }

  _group-scripts-by-target(enriched)
}

// Returns compact state snapshots per target.
// Includes variables, lists, and stage/sprite properties.
// Optional target filter: stage or exact target name.
// Output format: ((name: "...", is_stage: true/false, ...), ...)
#let sb3-state-catalog(sb3-bytes, target: auto, sb3-plugin: auto) = {
  let active-plugin = _resolve-plugin(sb3-plugin: sb3-plugin)
  let target-filter = _normalize-target(target)
  let project = json(bytes(_check-plugin-output(str(active-plugin.extract_project_json(sb3-bytes)))))

  let target-states = ()
  for target in project.targets {
    let match-item = (
      target_name: target.at("name", default: "Unnamed Target"),
      is_stage: target.at("isStage", default: false),
    )
    if _matches-target(match-item, target-filter) {
      target-states.push(_compact-target-state(target))
    }
  }

  target-states
}

// Low-level: convert SB3 bytes to parser text through the plugin.
// script-number is global and 1-based across all targets.
#let sb3-bytes-to-scratch-text(sb3-bytes, script-number: auto, sb3-plugin: auto) = {
  let active-plugin = _resolve-plugin(sb3-plugin: sb3-plugin)
  let text = if script-number == auto {
    str(active-plugin.sb3_to_scratch_text(sb3-bytes))
  } else {
    if type(script-number) != int or script-number < 1 {
      panic("render-sb3-scripts: script-number must be an integer >= 1.")
    }
    str(active-plugin.sb3_to_scratch_text_by_number(sb3-bytes, bytes(str(script-number))))
  }
  _check-plugin-output(text)
}

// Alias for readability in call sites.
#let sb3-to-scratch-text(sb3-bytes, script-number: auto, sb3-plugin: auto) = sb3-bytes-to-scratch-text(
  sb3-bytes,
  script-number: script-number,
  sb3-plugin: sb3-plugin,
)

// Convenience: directly render imported scripts as blockst content.
#let render-sb3-scripts(
  sb3-bytes,
  script-number: auto,
  target-script-number: auto,
  target: auto,
  sb3-plugin: auto,
  language: "en",
  show-headers: auto,
  header-gap: 1.5mm,
  script-gap: 3mm,
) = {
  let active-plugin = _resolve-plugin(sb3-plugin: sb3-plugin)
  let language = _normalize-language(language)
  let target-script-number = _normalize-target-script-number(target-script-number)
  let target-filter = _normalize-target(target)
  let show-headers = _resolve-show-headers(show-headers, target-filter)
  let ui = _ui-labels(language)

  if script-number != auto and target-script-number != auto {
    panic("render-sb3-scripts: script-number cannot be combined with target-script-number.")
  }

  if script-number != auto and target-filter.mode != "all" {
    panic("render-sb3-scripts: target cannot be combined with script-number.")
  }

  if target-script-number != auto and target-filter.mode == "all" {
    panic("render-sb3-scripts: target-script-number requires target.")
  }

  if script-number == auto {
    let catalog = sb3-scripts-catalog(
      sb3-bytes,
      target: target,
      include-parser-text: false,
      sb3-plugin: active-plugin,
    )

    let filtered-scripts = _flatten-grouped-scripts(catalog)

    if filtered-scripts.len() == 0 {
      let empty-message = if target-filter.mode == "all" { ui.no-scripts } else { ui.no-target-scripts }
      return text(size: 9pt, fill: rgb("666666"))[#empty-message]
    }

    if target-script-number != auto {
      let item = _pick-target-script(filtered-scripts, target-script-number)
      return [
        #if show-headers [
          #let target-kind = if item.is_stage { ui.stage } else { ui.sprite }
          #text(weight: "bold", size: 9pt)[#item.number. #target-kind: #item.target_name - #ui.script #item.local_number]
          #v(header-gap)
        ]
        #_render-scratch-text-localized(sb3-to-scratch-text(
          sb3-bytes,
          script-number: item.number,
          sb3-plugin: active-plugin,
        ), language)
      ]
    }

    [
      #for item in filtered-scripts [
        #if show-headers [
          #let target-kind = if item.is_stage { ui.stage } else { ui.sprite }
          #text(weight: "bold", size: 9pt)[#item.number. #target-kind: #item.target_name - #ui.script #item.local_number]
          #v(header-gap)
        ]
        #_render-scratch-text-localized(sb3-to-scratch-text(
          sb3-bytes,
          script-number: item.number,
          sb3-plugin: active-plugin,
        ), language)
        #v(script-gap)
      ]
    ]
  } else {
    _render-scratch-text-localized(sb3-to-scratch-text(
      sb3-bytes,
      script-number: script-number,
      sb3-plugin: active-plugin,
    ), language)
  }
}

#let render-sb3-lists(
  sb3-bytes,
  target: auto,
  target-list-name: auto,
  target-list-number: auto,
  sb3-plugin: auto,
  language: "en",
  show-target-headers: auto,
  target-gap: 2mm,
  item-gap: 0.8mm,
) = {
  let active-plugin = _resolve-plugin(sb3-plugin: sb3-plugin)
  let language = _normalize-language(language)
  let target-list-name = _normalize-target-list-name(target-list-name)
  let target-list-number = _normalize-target-list-number(target-list-number)
  let target-filter = _normalize-target(target)
  let show-target-headers = _resolve-show-target-headers(show-target-headers, target-filter)
  let ui = _ui-labels(language)
  let state = sb3-state-catalog(sb3-bytes, target: target, sb3-plugin: active-plugin)

  if target-list-name != auto and target-list-number != auto {
    panic("render-sb3-lists: target-list-name cannot be combined with target-list-number.")
  }

  if (target-list-name != auto or target-list-number != auto) and target-filter.mode == "all" {
    panic("render-sb3-lists: target-list-name/target-list-number requires target.")
  }

  if target-list-name != auto or target-list-number != auto {
    let match = if target-list-name != auto {
      _pick-target-list-by-name(state, target-list-name)
    } else {
      _pick-target-list(state, target-list-number)
    }
    return [
      #if show-target-headers [
        #text(weight: "bold", size: 9pt)[#_target-title(match.target_state, ui)]
        #v(1mm)
      ]
      #scratch-render-block(
        "data.monitor_list",
        args: (
          name: match.list_item.name,
          items: match.list_item.values,
          width: 5.2cm,
        ),
        lang-code: language,
      )
    ]
  }

  let total-lists = 0
  for target-state in state {
    total-lists += target-state.lists.len()
  }

  if total-lists == 0 {
    return text(size: 9pt, fill: rgb("666666"))[#ui.no-lists]
  }

  [
    #for target-state in state [
      #if target-state.lists.len() > 0 [
        #if show-target-headers [
          #text(weight: "bold", size: 9pt)[#_target-title(target-state, ui)]
          #v(1mm)
        ]
        #for list-item in target-state.lists [
          #scratch-render-block(
            "data.monitor_list",
            args: (
              name: list-item.name,
              items: list-item.values,
              width: 5.2cm,
            ),
            lang-code: language,
          )
          #v(item-gap)
        ]
        #v(target-gap)
      ]
    ]
  ]
}

#let render-sb3-variables(
  sb3-bytes,
  target: auto,
  target-variable-name: auto,
  target-variable-number: auto,
  sb3-plugin: auto,
  language: "en",
  show-target-headers: auto,
  target-gap: 2mm,
  item-gap: 0.8mm,
) = {
  let active-plugin = _resolve-plugin(sb3-plugin: sb3-plugin)
  let language = _normalize-language(language)
  let target-variable-name = _normalize-target-variable-name(target-variable-name)
  let target-variable-number = _normalize-target-variable-number(target-variable-number)
  let target-filter = _normalize-target(target)
  let show-target-headers = _resolve-show-target-headers(show-target-headers, target-filter)
  let ui = _ui-labels(language)
  let state = sb3-state-catalog(sb3-bytes, target: target, sb3-plugin: active-plugin)

  if target-variable-name != auto and target-variable-number != auto {
    panic("render-sb3-variables: target-variable-name cannot be combined with target-variable-number.")
  }

  if (target-variable-name != auto or target-variable-number != auto) and target-filter.mode == "all" {
    panic("render-sb3-variables: target-variable-name/target-variable-number requires target.")
  }

  if target-variable-name != auto or target-variable-number != auto {
    let match = if target-variable-name != auto {
      _pick-target-variable-by-name(state, target-variable-name)
    } else {
      _pick-target-variable(state, target-variable-number)
    }
    return [
      #if show-target-headers [
        #text(weight: "bold", size: 9pt)[#_target-title(match.target_state, ui)]
        #v(1mm)
      ]
      #scratch-render-block(
        "data.monitor_variable",
        args: (
          name: match.variable_item.name,
          value: match.variable_item.value,
        ),
        lang-code: language,
      )
    ]
  }

  let total-variables = 0
  for target-state in state {
    total-variables += target-state.variables.len()
  }

  if total-variables == 0 {
    return text(size: 9pt, fill: rgb("666666"))[#ui.no-variables]
  }

  [
    #for target-state in state [
      #if target-state.variables.len() > 0 [
        #if show-target-headers [
          #text(weight: "bold", size: 9pt)[#_target-title(target-state, ui)]
          #v(1mm)
        ]
        #for variable-item in target-state.variables [
          #scratch-render-block(
            "data.monitor_variable",
            args: (
              name: variable-item.name,
              value: variable-item.value,
            ),
            lang-code: language,
          )
          #v(item-gap)
        ]
        #v(target-gap)
      ]
    ]
  ]
}
