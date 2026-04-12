use std::collections::HashSet;
use std::io::{Cursor, Read};

use serde::Serialize;
use serde_json::{Map, Value};
use zip::ZipArchive;

#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::wasm_func;

#[cfg(target_arch = "wasm32")]
wasm_minimal_protocol::initiate_protocol!();

#[derive(Debug, Serialize)]
struct ScriptsDump {
  targets: Vec<ScriptsTarget>,
}

#[derive(Debug, Serialize)]
struct ScriptsTarget {
  name: String,
  is_stage: bool,
  scripts: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
struct ScriptCatalogItem {
  number: usize,
  local_number: usize,
  target_name: String,
  target_kind: String,
  is_stage: bool,
}

#[derive(Debug)]
struct NumberedScript {
  meta: ScriptCatalogItem,
  lines: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ScriptsCatalog {
  scripts: Vec<ScriptCatalogItem>,
}

#[derive(Debug, Serialize, Clone)]
struct ImageAssetCatalogItem {
  number: usize,
  local_number: usize,
  target_name: String,
  target_kind: String,
  is_stage: bool,
  asset_kind: String,
  asset_name: String,
  asset_id: String,
  md5ext: String,
  data_format: String,
}

#[derive(Debug, Serialize)]
struct ImageAssetsCatalog {
  images: Vec<ImageAssetCatalogItem>,
}

fn err_bytes(message: impl AsRef<str>) -> Vec<u8> {
  format!("ERROR: {}", message.as_ref()).into_bytes()
}

fn value_to_text(value: &Value) -> String {
  match value {
    Value::String(s) => s.clone(),
    Value::Number(n) => n.to_string(),
    Value::Bool(b) => {
      if *b {
        "true".to_string()
      } else {
        "false".to_string()
      }
    }
    Value::Null => "".to_string(),
    _ => value.to_string(),
  }
}

fn decode_literal_value(value: &Value) -> String {
  if let Some(arr) = value.as_array() {
    if arr.len() >= 2 {
      return value_to_text(&arr[1]);
    }
    if let Some(first) = arr.first() {
      return value_to_text(first);
    }
    return "".to_string();
  }

  value_to_text(value)
}

fn sanitize_square_brackets(value: &str) -> String {
  value.replace('[', "(").replace(']', ")")
}

fn as_number_token(value: &str) -> String {
  let normalized = value.trim();
  if normalized.starts_with('(') && normalized.ends_with(')') {
    normalized.to_string()
  } else if normalized.is_empty() {
    "(0)".to_string()
  } else {
    format!("({normalized})")
  }
}

fn as_text_token(value: &str) -> String {
  let normalized = sanitize_square_brackets(value).trim().to_string();
  if normalized.starts_with('[') && normalized.ends_with(']') {
    normalized
  } else {
    format!("[{normalized}]")
  }
}

fn as_color_arg(value: &str) -> String {
  let normalized = value.trim();
  if normalized.starts_with('[') && normalized.ends_with(']') && normalized.len() >= 2 {
    normalized[1..normalized.len() - 1].trim().to_string()
  } else {
    normalized.to_string()
  }
}

fn normalize_clone_target(value: &str) -> String {
  let normalized = value.trim();
  let unwrapped = if normalized.starts_with('[') && normalized.ends_with(']') && normalized.len() >= 2 {
    normalized[1..normalized.len() - 1].trim()
  } else {
    normalized
  };

  if unwrapped == "_myself_" {
    "myself".to_string()
  } else {
    unwrapped.to_string()
  }
}

fn as_condition_token(value: &str) -> String {
  let normalized = value.trim();
  if normalized.is_empty() {
    "< >".to_string()
  } else {
    normalized.to_string()
  }
}

fn decode_expression_literal(value: &Value) -> Option<String> {
  let arr = value.as_array()?;
  let kind = arr.first().and_then(Value::as_i64)?;

  match kind {
    // Scratch variable literal: [12, "VariableName", "variableId"]
    12 => {
      let name = arr
        .get(1)
        .map(value_to_text)
        .unwrap_or_else(|| "variable".to_string());
      Some(format!("var {}", as_text_token(&name)))
    }
    _ => None,
  }
}

fn get_field_value(block: &Map<String, Value>, key: &str, fallback: &str) -> String {
  let Some(fields) = block.get("fields").and_then(Value::as_object) else {
    return fallback.to_string();
  };

  let Some(raw) = fields.get(key) else {
    return fallback.to_string();
  };

  if let Some(arr) = raw.as_array() {
    if let Some(first) = arr.first() {
      return value_to_text(first);
    }
    return fallback.to_string();
  }

  value_to_text(raw)
}

fn get_input_entry<'a>(block: &'a Map<String, Value>, key: &str) -> Option<&'a Vec<Value>> {
  block
    .get("inputs")
    .and_then(Value::as_object)
    .and_then(|inputs| inputs.get(key))
    .and_then(Value::as_array)
}

fn get_substack_start(blocks: &Map<String, Value>, block: &Map<String, Value>, key: &str) -> Option<String> {
  let entry = get_input_entry(block, key)?;
  let block_id = entry.get(1)?.as_str()?.to_string();

  if blocks.contains_key(&block_id) {
    Some(block_id)
  } else {
    None
  }
}

fn get_mutation_value(block: &Map<String, Value>, key: &str) -> Option<String> {
  block
    .get("mutation")
    .and_then(Value::as_object)
    .and_then(|mutation| mutation.get(key))
    .and_then(Value::as_str)
    .map(|value| value.to_string())
}

fn parse_string_array(raw: Option<String>) -> Vec<String> {
  let Some(raw) = raw else {
    return Vec::new();
  };

  serde_json::from_str::<Vec<String>>(&raw).unwrap_or_default()
}

fn procedure_slot_token(value: &str) -> String {
  let trimmed = value.trim();
  if trimmed.is_empty() {
    return "[]".to_string();
  }

  if trimmed.starts_with('[') && trimmed.ends_with(']') {
    trimmed.to_string()
  } else {
    as_text_token(trimmed)
  }
}

fn resolve_procedure_signature(
  blocks: &Map<String, Value>,
  block: &Map<String, Value>,
) -> (String, Vec<String>) {
  let fallback_proccode = get_mutation_value(block, "proccode").unwrap_or_else(|| "my block".to_string());
  let fallback_arg_names = parse_string_array(get_mutation_value(block, "argumentnames"));

  let Some(proto_id) = get_input_entry(block, "custom_block")
    .and_then(|entry| entry.get(1))
    .and_then(Value::as_str)
  else {
    return (fallback_proccode, fallback_arg_names);
  };

  let Some(proto_block) = blocks.get(proto_id).and_then(Value::as_object) else {
    return (fallback_proccode, fallback_arg_names);
  };

  let proccode = get_mutation_value(proto_block, "proccode")
    .or_else(|| get_mutation_value(block, "proccode"))
    .unwrap_or_else(|| "my block".to_string());

  let arg_names = {
    let from_proto = parse_string_array(get_mutation_value(proto_block, "argumentnames"));
    if from_proto.is_empty() {
      parse_string_array(get_mutation_value(block, "argumentnames"))
    } else {
      from_proto
    }
  };

  (proccode, arg_names)
}

fn fill_procedure_placeholders(proccode: &str, values: &[String]) -> String {
  let mut out = proccode.to_string();

  for value in values {
    if out.contains("%s") {
      out = out.replacen("%s", value, 1);
    } else if out.contains("%b") {
      out = out.replacen("%b", value, 1);
    } else {
      break;
    }
  }

  out
}

fn is_expression_input(block: &Map<String, Value>, key: &str) -> bool {
  let Some(entry) = get_input_entry(block, key) else {
    return false;
  };

  let Some(kind) = entry.first().and_then(Value::as_i64) else {
    return false;
  };

  kind == 2 || kind == 3
}

fn render_input_expression(
  blocks: &Map<String, Value>,
  block: &Map<String, Value>,
  key: &str,
  seen: &mut HashSet<String>,
) -> String {
  let Some(entry) = get_input_entry(block, key) else {
    return "".to_string();
  };

  if let Some(payload) = entry.get(1) {
    if let Some(block_id) = payload.as_str() {
      if blocks.contains_key(block_id) {
        return render_expression_by_id(blocks, block_id, seen);
      }
    }

    if let Some(expression_literal) = decode_expression_literal(payload) {
      return expression_literal;
    }

    return decode_literal_value(payload);
  }

  if let Some(shadow) = entry.get(2) {
    if let Some(expression_literal) = decode_expression_literal(shadow) {
      return expression_literal;
    }
    return decode_literal_value(shadow);
  }

  "".to_string()
}

fn render_expression_by_id(blocks: &Map<String, Value>, block_id: &str, seen: &mut HashSet<String>) -> String {
  if seen.contains(block_id) {
    return "".to_string();
  }

  let Some(block) = blocks.get(block_id).and_then(Value::as_object) else {
    return "".to_string();
  };

  seen.insert(block_id.to_string());

  let input_text = |key: &str, seen: &mut HashSet<String>| render_input_expression(blocks, block, key, seen);
  let opcode = block
    .get("opcode")
    .and_then(Value::as_str)
    .unwrap_or("unknown_opcode");

  let out = match opcode {
    "operator_add" => format!(
      "{} + {}",
      as_number_token(&input_text("NUM1", seen)),
      as_number_token(&input_text("NUM2", seen))
    ),
    "operator_subtract" => format!(
      "{} - {}",
      as_number_token(&input_text("NUM1", seen)),
      as_number_token(&input_text("NUM2", seen))
    ),
    "operator_multiply" => format!(
      "{} * {}",
      as_number_token(&input_text("NUM1", seen)),
      as_number_token(&input_text("NUM2", seen))
    ),
    "operator_divide" => format!(
      "{} / {}",
      as_number_token(&input_text("NUM1", seen)),
      as_number_token(&input_text("NUM2", seen))
    ),
    "operator_mod" => format!(
      "{} mod {}",
      as_number_token(&input_text("NUM1", seen)),
      as_number_token(&input_text("NUM2", seen))
    ),
    "operator_random" => format!(
      "pick random {} to {}",
      as_number_token(&input_text("FROM", seen)),
      as_number_token(&input_text("TO", seen))
    ),
    "operator_join" => format!(
      "join {} {}",
      as_text_token(&input_text("STRING1", seen)),
      as_text_token(&input_text("STRING2", seen))
    ),
    "operator_letter_of" => format!(
      "letter {} of {}",
      as_number_token(&input_text("LETTER", seen)),
      as_text_token(&input_text("STRING", seen))
    ),
    "operator_length" => format!("length of {}", as_text_token(&input_text("STRING", seen))),
    "operator_round" => format!("round {}", as_number_token(&input_text("NUM", seen))),
    "operator_mathop" => {
      let operator = get_field_value(block, "OPERATOR", "abs");
      format!(
        "{} of {}",
        sanitize_square_brackets(&operator),
        as_number_token(&input_text("NUM", seen))
      )
    }
    "operator_contains" => format!(
      "{} contains {} ?",
      as_text_token(&input_text("STRING1", seen)),
      as_text_token(&input_text("STRING2", seen))
    ),
    "operator_gt" => format!(
      "{} > {}",
      as_number_token(&input_text("OPERAND1", seen)),
      as_number_token(&input_text("OPERAND2", seen))
    ),
    "operator_lt" => format!(
      "{} < {}",
      as_number_token(&input_text("OPERAND1", seen)),
      as_number_token(&input_text("OPERAND2", seen))
    ),
    "operator_equals" => format!(
      "{} = {}",
      as_text_token(&input_text("OPERAND1", seen)),
      as_text_token(&input_text("OPERAND2", seen))
    ),
    "operator_and" => format!(
      "{} and {}",
      as_condition_token(&input_text("OPERAND1", seen)),
      as_condition_token(&input_text("OPERAND2", seen))
    ),
    "operator_or" => format!(
      "{} or {}",
      as_condition_token(&input_text("OPERAND1", seen)),
      as_condition_token(&input_text("OPERAND2", seen))
    ),
    "operator_not" => format!("not {}", as_condition_token(&input_text("OPERAND", seen))),
    "sensing_touchingobject" => {
      let object = get_field_value(block, "TOUCHINGOBJECTMENU", "mouse-pointer");
      format!("touching {} ?", as_text_token(&object))
    }
    "sensing_touchingcolor" => {
      let color = as_color_arg(&input_text("COLOR", seen));
      format!("touching color {} ?", color)
    }
    "sensing_touchingobjectmenu" => get_field_value(block, "TOUCHINGOBJECTMENU", "mouse-pointer"),
    "motion_glideto_menu" => get_field_value(block, "TO", "random position"),
    "sensing_keypressed" => {
      let key_input = input_text("KEY_OPTION", seen);
      let key = if key_input.is_empty() {
        get_field_value(block, "KEY_OPTION", "space")
      } else {
        key_input
      };
      format!("key {} pressed?", as_text_token(&key))
    }
    "sensing_keyoptions" => get_field_value(block, "KEY_OPTION", "space"),
    "sensing_mousedown" => "mouse down?".to_string(),
    "sensing_mousex" => "mouse x".to_string(),
    "sensing_mousey" => "mouse y".to_string(),
    "motion_xposition" => "x position".to_string(),
    "motion_yposition" => "y position".to_string(),
    "motion_direction" => "direction".to_string(),
    "looks_costume" => get_field_value(block, "COSTUME", "costume1"),
    "looks_backdrops" => get_field_value(block, "BACKDROP", "backdrop1"),
    "music_menu_INSTRUMENT" => get_field_value(block, "INSTRUMENT", "1"),
    "control_create_clone_of_menu" => {
      normalize_clone_target(&get_field_value(block, "CLONE_OPTION", "_myself_"))
    }
    "sensing_answer" => "answer".to_string(),
    "sensing_timer" => "timer".to_string(),
    "sensing_loudness" => "loudness".to_string(),
    "data_itemoflist" => {
      let index = input_text("INDEX", seen);
      let list = get_field_value(block, "LIST", "list");
      format!("item {} of {}", as_number_token(&index), as_text_token(&list))
    }
    "data_lengthoflist" => {
      let list = get_field_value(block, "LIST", "list");
      format!("length of {}", as_text_token(&list))
    }
    "data_variable" => {
      let variable = get_field_value(block, "VARIABLE", "variable");
      format!("var {}", as_text_token(&variable))
    }
    "data_itemnumoflist" => {
      let list = get_field_value(block, "LIST", "list");
      let item = input_text("ITEM", seen);
      format!("item # of {} in {}", as_text_token(&item), as_text_token(&list))
    }
    "data_listcontainsitem" => {
      let list = get_field_value(block, "LIST", "list");
      let item = input_text("ITEM", seen);
      format!("{} contains {} ?", as_text_token(&list), as_text_token(&item))
    }
    "data_listcontents" => {
      let list = get_field_value(block, "LIST", "list");
      as_text_token(&list)
    }
    "argument_reporter_string_number" => {
      let arg_name = get_field_value(block, "VALUE", "input");
      format!("var {}", as_text_token(&arg_name))
    }
    "argument_reporter_boolean" => {
      let arg_name = get_field_value(block, "VALUE", "condition");
      format!("<{}>", sanitize_square_brackets(&arg_name))
    }
    "control_get_counter" => "counter".to_string(),
    "looks_backdropnumbername" => {
      let property = get_field_value(block, "NUMBER_NAME", "number");
      format!("backdrop {}", as_text_token(&property))
    }
    "looks_costumenumbername" => {
      let property = get_field_value(block, "NUMBER_NAME", "number");
      format!("costume {}", as_text_token(&property))
    }
    "looks_size" => "size".to_string(),
    "motion_xscroll" => "x scroll".to_string(),
    "motion_yscroll" => "y scroll".to_string(),
    "sensing_coloristouchingcolor" => format!(
      "color {} is touching {} ?",
      as_color_arg(&input_text("COLOR", seen)),
      as_color_arg(&input_text("COLOR2", seen))
    ),
    "sensing_current" => {
      let current = get_field_value(block, "CURRENTMENU", "year");
      format!("current {}", as_text_token(&current))
    }
    "sensing_dayssince2000" => "days since 2000".to_string(),
    "sensing_distanceto" => {
      let object_input = input_text("DISTANCETOMENU", seen);
      let object = if object_input.is_empty() {
        get_field_value(block, "DISTANCETOMENU", "mouse-pointer")
      } else {
        object_input
      };
      format!("distance to {}", as_text_token(&object))
    }
    "sensing_loud" => "loudness > (10)".to_string(),
    "sensing_of" => {
      let property = get_field_value(block, "PROPERTY", "x position");
      let object_input = input_text("OBJECT", seen);
      let object = if object_input.is_empty() {
        get_field_value(block, "OBJECT", "Stage")
      } else {
        object_input
      };
      format!("{} of {}", as_text_token(&property), as_text_token(&object))
    }
    "sensing_userid" => "user id".to_string(),
    "sensing_username" => "username".to_string(),
    "sound_beats_menu" => get_field_value(block, "BEATS", "0.25"),
    "sound_effects_menu" => get_field_value(block, "EFFECT", "pitch"),
    "sound_sounds_menu" => get_field_value(block, "SOUND_MENU", "pop"),
    "sound_volume" => "volume".to_string(),
    "note" => get_field_value(block, "NOTE", "60"),
    other => sanitize_square_brackets(&format!("unsupported:{other}")),
  };

  seen.remove(block_id);
  out
}

fn render_stack(
  blocks: &Map<String, Value>,
  start_block_id: &str,
  indent: &str,
  inherited_seen: &HashSet<String>,
) -> Vec<String> {
  let mut lines = Vec::new();
  let mut local_seen = inherited_seen.clone();
  let mut current = Some(start_block_id.to_string());

  while let Some(block_id) = current {
    if local_seen.contains(&block_id) {
      lines.push(format!("{indent}// cycle detected at block {block_id}"));
      break;
    }

    local_seen.insert(block_id.clone());

    let Some(block) = blocks.get(&block_id).and_then(Value::as_object) else {
      lines.push(format!("{indent}// missing block {block_id}"));
      break;
    };

    lines.extend(render_statement(blocks, &block_id, indent, &mut local_seen));

    current = block
      .get("next")
      .and_then(Value::as_str)
      .map(|next| next.to_string());
  }

  lines
}

fn render_statement(
  blocks: &Map<String, Value>,
  block_id: &str,
  indent: &str,
  seen: &mut HashSet<String>,
) -> Vec<String> {
  let Some(block) = blocks.get(block_id).and_then(Value::as_object) else {
    return vec![format!("{indent}// missing block: {block_id}")];
  };

  let mut expr_seen = HashSet::new();
  let input_text = |key: &str, expr_seen: &mut HashSet<String>| {
    render_input_expression(blocks, block, key, expr_seen)
  };
  let input_is_expression = |key: &str| is_expression_input(block, key);
  let opcode = block
    .get("opcode")
    .and_then(Value::as_str)
    .unwrap_or("unknown_opcode");

  match opcode {
    "event_whenflagclicked" => vec![format!("{indent}when flag clicked")],
    "event_whenkeypressed" => {
      let key = get_field_value(block, "KEY_OPTION", "space");
      vec![format!("{indent}when {} key pressed", as_text_token(&key))]
    }
    "event_whenthisspriteclicked" => vec![format!("{indent}when this sprite clicked")],
    "event_whenbackdropswitchesto" => {
      let backdrop = get_field_value(block, "BACKDROP", "backdrop1");
      vec![format!("{indent}when backdrop switches to {}", as_text_token(&backdrop))]
    }
    "event_whenbroadcastreceived" => {
      let message = get_field_value(block, "BROADCAST_OPTION", "message1");
      vec![format!("{indent}when I receive {}", as_text_token(&message))]
    }
    "event_whengreaterthan" => {
      let menu = get_field_value(block, "WHENGREATERTHANMENU", "LOUDNESS").to_ascii_lowercase();
      let element = if menu == "timer" { "timer" } else { "loudness" };
      vec![format!(
        "{indent}when {} > {}",
        as_text_token(element),
        as_number_token(&input_text("VALUE", &mut expr_seen))
      )]
    }
    "event_whenstageclicked" => vec![format!("{indent}// legacy opcode: event_whenstageclicked")],
    "event_whentouchingobject" => {
      let object = get_field_value(block, "TOUCHINGOBJECTMENU", "mouse-pointer");
      vec![format!(
        "{indent}// legacy opcode: event_whentouchingobject ({})",
        sanitize_square_brackets(&object)
      )]
    }
    "event_broadcast" => {
      let input = input_text("BROADCAST_INPUT", &mut expr_seen);
      let message = if input.is_empty() {
        get_field_value(block, "BROADCAST_OPTION", "message1")
      } else {
        input
      };
      vec![format!("{indent}broadcast {}", as_text_token(&message))]
    }
    "event_broadcastandwait" => {
      let input = input_text("BROADCAST_INPUT", &mut expr_seen);
      let message = if input.is_empty() {
        get_field_value(block, "BROADCAST_OPTION", "message1")
      } else {
        input
      };
      vec![format!("{indent}broadcast {} and wait", as_text_token(&message))]
    }
    "motion_movesteps" => vec![format!(
      "{indent}move {} steps",
      as_number_token(&input_text("STEPS", &mut expr_seen))
    )],
    "motion_turnright" => vec![format!(
      "{indent}turn right {} degrees",
      as_number_token(&input_text("DEGREES", &mut expr_seen))
    )],
    "motion_turnleft" => vec![format!(
      "{indent}turn left {} degrees",
      as_number_token(&input_text("DEGREES", &mut expr_seen))
    )],
    "motion_gotoxy" => vec![format!(
      "{indent}go to x: {} y: {}",
      as_number_token(&input_text("X", &mut expr_seen)),
      as_number_token(&input_text("Y", &mut expr_seen))
    )],
    "motion_goto" => {
      let to_input = input_text("TO", &mut expr_seen);
      let to = if to_input.is_empty() {
        get_field_value(block, "TO", "random position")
      } else {
        to_input
      };
      vec![format!("{indent}go to {}", as_text_token(&to))]
    }
    "motion_glideto" => {
      let to_input = input_text("TO", &mut expr_seen);
      let to = if to_input.is_empty() {
        get_field_value(block, "TO", "random position")
      } else {
        to_input
      };
      vec![format!(
        "{indent}glide {} secs to {}",
        as_number_token(&input_text("SECS", &mut expr_seen)),
        as_text_token(&to)
      )]
    }
    "motion_glidesecstoxy" => vec![format!(
      "{indent}glide {} secs to x: {} y: {}",
      as_number_token(&input_text("SECS", &mut expr_seen)),
      as_number_token(&input_text("X", &mut expr_seen)),
      as_number_token(&input_text("Y", &mut expr_seen))
    )],
    "motion_pointindirection" => vec![format!(
      "{indent}point in direction {}",
      as_number_token(&input_text("DIRECTION", &mut expr_seen))
    )],
    "motion_pointtowards" => {
      let towards_input = input_text("TOWARDS", &mut expr_seen);
      let towards = if towards_input.is_empty() {
        get_field_value(block, "TOWARDS", "mouse-pointer")
      } else {
        towards_input
      };
      vec![format!("{indent}point towards {}", as_text_token(&towards))]
    }
    "motion_changexby" => vec![format!(
      "{indent}change x by {}",
      as_number_token(&input_text("DX", &mut expr_seen))
    )],
    "motion_setx" => vec![format!(
      "{indent}set x to {}",
      as_number_token(&input_text("X", &mut expr_seen))
    )],
    "motion_changeyby" => vec![format!(
      "{indent}change y by {}",
      as_number_token(&input_text("DY", &mut expr_seen))
    )],
    "motion_sety" => vec![format!(
      "{indent}set y to {}",
      as_number_token(&input_text("Y", &mut expr_seen))
    )],
    "motion_setrotationstyle" => {
      let style = get_field_value(block, "STYLE", "all around");
      vec![format!("{indent}set rotation style {}", as_text_token(&style))]
    }
    "motion_align_scene" => vec![format!("{indent}// legacy opcode: motion_align_scene")],
    "motion_scroll_right" => vec![format!(
      "{indent}// legacy opcode: motion_scroll_right {}",
      as_number_token(&input_text("DISTANCE", &mut expr_seen))
    )],
    "motion_scroll_up" => vec![format!(
      "{indent}// legacy opcode: motion_scroll_up {}",
      as_number_token(&input_text("DISTANCE", &mut expr_seen))
    )],
    "motion_ifonedgebounce" => vec![format!("{indent}if on edge, bounce")],
    "looks_say" => vec![format!(
      "{indent}say {}",
      as_text_token(&input_text("MESSAGE", &mut expr_seen))
    )],
    "looks_sayforsecs" => vec![format!(
      "{indent}say {} for {} seconds",
      as_text_token(&input_text("MESSAGE", &mut expr_seen)),
      as_number_token(&input_text("SECS", &mut expr_seen))
    )],
    "looks_think" => vec![format!(
      "{indent}think {}",
      as_text_token(&input_text("MESSAGE", &mut expr_seen))
    )],
    "looks_thinkforsecs" => vec![format!(
      "{indent}think {} for {} seconds",
      as_text_token(&input_text("MESSAGE", &mut expr_seen)),
      as_number_token(&input_text("SECS", &mut expr_seen))
    )],
    "looks_switchcostumeto" => {
      let input = input_text("COSTUME", &mut expr_seen);
      let costume = if input.is_empty() {
        get_field_value(block, "COSTUME", "costume1")
      } else {
        input
      };
      vec![format!("{indent}switch costume to {}", as_text_token(&costume))]
    }
    "looks_nextcostume" => vec![format!("{indent}next costume")],
    "looks_switchbackdropto" => {
      let input = input_text("BACKDROP", &mut expr_seen);
      let backdrop = if input.is_empty() {
        get_field_value(block, "BACKDROP", "backdrop1")
      } else {
        input
      };
      vec![format!("{indent}switch backdrop to {}", as_text_token(&backdrop))]
    }
    "looks_switchbackdroptoandwait" => {
      let input = input_text("BACKDROP", &mut expr_seen);
      let backdrop = if input.is_empty() {
        get_field_value(block, "BACKDROP", "backdrop1")
      } else {
        input
      };
      vec![format!(
        "{indent}switch backdrop to {} and wait",
        as_text_token(&backdrop)
      )]
    }
    "looks_nextbackdrop" => vec![format!("{indent}next backdrop")],
    "looks_changesizeby" => vec![format!(
      "{indent}change size by {}",
      as_number_token(&input_text("CHANGE", &mut expr_seen))
    )],
    "looks_setsizeto" => vec![format!(
      "{indent}set size to {} %",
      as_number_token(&input_text("SIZE", &mut expr_seen))
    )],
    "looks_changeeffectby" => {
      let effect = get_field_value(block, "EFFECT", "color").to_ascii_lowercase();
      vec![format!(
        "{indent}change {} effect by {}",
        as_text_token(&effect),
        as_number_token(&input_text("CHANGE", &mut expr_seen))
      )]
    }
    "looks_seteffectto" => {
      let effect = get_field_value(block, "EFFECT", "color").to_ascii_lowercase();
      vec![format!(
        "{indent}set {} effect to {}",
        as_text_token(&effect),
        as_number_token(&input_text("VALUE", &mut expr_seen))
      )]
    }
    "looks_gotofrontback" => {
      let layer = get_field_value(block, "FRONT_BACK", "front").to_ascii_lowercase();
      vec![format!("{indent}go to {} layer", as_text_token(&layer))]
    }
    "looks_goforwardbackwardlayers" => {
      let direction = get_field_value(block, "FORWARD_BACKWARD", "forward").to_ascii_lowercase();
      vec![format!(
        "{indent}go {} {} layers",
        as_text_token(&direction),
        as_number_token(&input_text("NUM", &mut expr_seen))
      )]
    }
    "looks_changestretchby" => vec![format!("{indent}// legacy opcode: looks_changestretchby")],
    "looks_setstretchto" => vec![format!("{indent}// legacy opcode: looks_setstretchto")],
    "looks_hideallsprites" => vec![format!("{indent}// legacy opcode: looks_hideallsprites")],
    "looks_cleargraphiceffects" => vec![format!("{indent}clear graphic effects")],
    "looks_show" => vec![format!("{indent}show")],
    "looks_hide" => vec![format!("{indent}hide")],
    "sound_playuntildone" => {
      let input = input_text("SOUND_MENU", &mut expr_seen);
      let sound = if input.is_empty() {
        get_field_value(block, "SOUND_MENU", "pop")
      } else {
        input
      };
      vec![format!("{indent}play sound {} until done", as_text_token(&sound))]
    }
    "sound_play" => {
      let input = input_text("SOUND_MENU", &mut expr_seen);
      let sound = if input.is_empty() {
        get_field_value(block, "SOUND_MENU", "pop")
      } else {
        input
      };
      vec![format!("{indent}start sound {}", as_text_token(&sound))]
    }
    "music_playNoteForBeats" => {
      let note = if let Some(entry) = get_input_entry(block, "NOTE") {
        if let Some(shadow_id) = entry.get(2).and_then(Value::as_str) {
          render_expression_by_id(blocks, shadow_id, &mut expr_seen)
        } else {
          input_text("NOTE", &mut expr_seen)
        }
      } else {
        input_text("NOTE", &mut expr_seen)
      };
      vec![format!(
        "{indent}play note {} for {} beats",
        as_number_token(&note),
        as_number_token(&input_text("BEATS", &mut expr_seen))
      )]
    }
    "music_setInstrument" => vec![format!(
      "{indent}set instrument to {}",
      as_number_token(&input_text("INSTRUMENT", &mut expr_seen))
    )],
    "sound_stopallsounds" => vec![format!("{indent}stop all sounds")],
    "sound_changeeffectby" => {
      let effect = get_field_value(block, "EFFECT", "pitch").to_ascii_lowercase();
      vec![format!(
        "{indent}change {} effect by {}",
        as_text_token(&effect),
        as_number_token(&input_text("VALUE", &mut expr_seen))
      )]
    }
    "sound_seteffectto" => {
      let effect = get_field_value(block, "EFFECT", "pitch").to_ascii_lowercase();
      vec![format!(
        "{indent}set {} effect to {}",
        as_text_token(&effect),
        as_number_token(&input_text("VALUE", &mut expr_seen))
      )]
    }
    "sound_cleareffects" => vec![format!("{indent}clear sound effects")],
    "sound_changevolumeby" => vec![format!(
      "{indent}change volume by {}",
      as_number_token(&input_text("VOLUME", &mut expr_seen))
    )],
    "sound_setvolumeto" => vec![format!(
      "{indent}set volume to {} %",
      as_number_token(&input_text("VOLUME", &mut expr_seen))
    )],
    "sensing_askandwait" => {
      let question = input_text("QUESTION", &mut expr_seen);
      let final_question = if question.is_empty() {
        "What's your name?".to_string()
      } else {
        question
      };
      vec![format!("{indent}ask {} and wait", as_text_token(&final_question))]
    }
    "sensing_setdragmode" => {
      let mode = get_field_value(block, "DRAG_MODE", "draggable").to_ascii_lowercase();
      vec![format!("{indent}set drag mode {}", as_text_token(&mode))]
    }
    "sensing_resettimer" => vec![format!("{indent}reset timer")],
    "data_setvariableto" => {
      let variable = get_field_value(block, "VARIABLE", "variable");
      let value = input_text("VALUE", &mut expr_seen);
      let rendered_value = if input_is_expression("VALUE") {
        value.trim().to_string()
      } else {
        as_text_token(&value)
      };
      vec![format!(
        "{indent}set {} to {}",
        as_text_token(&variable),
        rendered_value
      )]
    }
    "data_changevariableby" => {
      let variable = get_field_value(block, "VARIABLE", "variable");
      let value = input_text("VALUE", &mut expr_seen);
      vec![format!(
        "{indent}change {} by {}",
        as_text_token(&variable),
        as_number_token(&value)
      )]
    }
    "data_showvariable" => {
      let variable = get_field_value(block, "VARIABLE", "variable");
      vec![format!("{indent}show variable {}", as_text_token(&variable))]
    }
    "data_hidevariable" => {
      let variable = get_field_value(block, "VARIABLE", "variable");
      vec![format!("{indent}hide variable {}", as_text_token(&variable))]
    }
    "data_addtolist" => {
      let list = get_field_value(block, "LIST", "list");
      let item = input_text("ITEM", &mut expr_seen);
      let rendered_item = if input_is_expression("ITEM") {
        item.trim().to_string()
      } else {
        as_text_token(&item)
      };
      vec![format!("{indent}add {} to {}", rendered_item, as_text_token(&list))]
    }
    "data_deleteoflist" => {
      let list = get_field_value(block, "LIST", "list");
      let index = input_text("INDEX", &mut expr_seen);
      vec![format!(
        "{indent}delete {} of {}",
        as_number_token(&index),
        as_text_token(&list)
      )]
    }
    "data_deletealloflist" => {
      let list = get_field_value(block, "LIST", "list");
      vec![format!("{indent}delete all of {}", as_text_token(&list))]
    }
    "data_insertatlist" => {
      let list = get_field_value(block, "LIST", "list");
      let item = input_text("ITEM", &mut expr_seen);
      let index = input_text("INDEX", &mut expr_seen);
      let rendered_item = if input_is_expression("ITEM") {
        item.trim().to_string()
      } else {
        as_text_token(&item)
      };
      vec![format!(
        "{indent}insert {} at {} of {}",
        rendered_item,
        as_number_token(&index),
        as_text_token(&list)
      )]
    }
    "data_replaceitemoflist" => {
      let list = get_field_value(block, "LIST", "list");
      let item = input_text("ITEM", &mut expr_seen);
      let index = input_text("INDEX", &mut expr_seen);
      let rendered_item = if input_is_expression("ITEM") {
        item.trim().to_string()
      } else {
        as_text_token(&item)
      };
      vec![format!(
        "{indent}replace item {} of {} with {}",
        as_number_token(&index),
        as_text_token(&list),
        rendered_item
      )]
    }
    "data_showlist" => {
      let list = get_field_value(block, "LIST", "list");
      vec![format!("{indent}show list {}", as_text_token(&list))]
    }
    "data_hidelist" => {
      let list = get_field_value(block, "LIST", "list");
      vec![format!("{indent}hide list {}", as_text_token(&list))]
    }
    "control_wait" => vec![format!(
      "{indent}wait {} seconds",
      as_number_token(&input_text("DURATION", &mut expr_seen))
    )],
    "control_wait_until" => vec![format!(
      "{indent}wait until {}",
      as_condition_token(&input_text("CONDITION", &mut expr_seen))
    )],
    "control_repeat" => {
      let mut lines = vec![format!(
        "{indent}repeat {}",
        as_number_token(&input_text("TIMES", &mut expr_seen))
      )];
      if let Some(start) = get_substack_start(blocks, block, "SUBSTACK") {
        lines.extend(render_stack(blocks, &start, &format!("{indent}  "), seen));
      }
      lines.push(format!("{indent}end"));
      lines
    }
    "control_forever" => {
      let mut lines = vec![format!("{indent}forever")];
      if let Some(start) = get_substack_start(blocks, block, "SUBSTACK") {
        lines.extend(render_stack(blocks, &start, &format!("{indent}  "), seen));
      }
      lines.push(format!("{indent}end"));
      lines
    }
    "control_if" => {
      let mut lines = vec![format!(
        "{indent}if {} then",
        as_condition_token(&input_text("CONDITION", &mut expr_seen))
      )];
      if let Some(start) = get_substack_start(blocks, block, "SUBSTACK") {
        lines.extend(render_stack(blocks, &start, &format!("{indent}  "), seen));
      }
      lines.push(format!("{indent}end"));
      lines
    }
    "control_if_else" => {
      let mut lines = vec![format!(
        "{indent}if {} then",
        as_condition_token(&input_text("CONDITION", &mut expr_seen))
      )];
      if let Some(start) = get_substack_start(blocks, block, "SUBSTACK") {
        lines.extend(render_stack(blocks, &start, &format!("{indent}  "), seen));
      }
      lines.push(format!("{indent}else"));
      if let Some(start) = get_substack_start(blocks, block, "SUBSTACK2") {
        lines.extend(render_stack(blocks, &start, &format!("{indent}  "), seen));
      }
      lines.push(format!("{indent}end"));
      lines
    }
    "control_repeat_until" => {
      let mut lines = vec![format!(
        "{indent}repeat until {}",
        as_condition_token(&input_text("CONDITION", &mut expr_seen))
      )];
      if let Some(start) = get_substack_start(blocks, block, "SUBSTACK") {
        lines.extend(render_stack(blocks, &start, &format!("{indent}  "), seen));
      }
      lines.push(format!("{indent}end"));
      lines
    }
    "control_while" => {
      let mut lines = vec![format!(
        "{indent}// legacy opcode: control_while {}",
        as_condition_token(&input_text("CONDITION", &mut expr_seen))
      )];
      if let Some(start) = get_substack_start(blocks, block, "SUBSTACK") {
        lines.extend(render_stack(blocks, &start, &format!("{indent}  "), seen));
      }
      lines
    }
    "control_for_each" => {
      let mut lines = vec![format!("{indent}// legacy opcode: control_for_each")];
      if let Some(start) = get_substack_start(blocks, block, "SUBSTACK") {
        lines.extend(render_stack(blocks, &start, &format!("{indent}  "), seen));
      }
      lines
    }
    "control_all_at_once" => {
      let mut lines = vec![format!("{indent}// legacy opcode: control_all_at_once")];
      if let Some(start) = get_substack_start(blocks, block, "SUBSTACK") {
        lines.extend(render_stack(blocks, &start, &format!("{indent}  "), seen));
      }
      lines
    }
    "control_incr_counter" => vec![format!("{indent}// legacy opcode: control_incr_counter")],
    "control_clear_counter" => vec![format!("{indent}// legacy opcode: control_clear_counter")],
    "control_stop" => {
      let option = get_field_value(block, "STOP_OPTION", "all");
      vec![format!("{indent}stop {}", as_text_token(&option))]
    }
    "control_start_as_clone" => vec![format!("{indent}when I start as a clone")],
    "control_create_clone_of" => {
      let clone_input = input_text("CLONE_OPTION", &mut expr_seen);
      let clone = if clone_input.is_empty() {
        normalize_clone_target(&get_field_value(block, "CLONE_OPTION", "_myself_"))
      } else {
        normalize_clone_target(&clone_input)
      };
      vec![format!("{indent}create clone of {}", as_text_token(&clone))]
    }
    "control_delete_this_clone" => vec![format!("{indent}delete this clone")],
    "procedures_definition" => {
      let (proccode, arg_names) = resolve_procedure_signature(blocks, block);
      let rendered_slots: Vec<String> = arg_names
        .iter()
        .map(|name| procedure_slot_token(name))
        .collect();
      let label = fill_procedure_placeholders(&proccode, &rendered_slots);
      vec![format!("{indent}define {label}")]
    }
    "procedures_prototype" => Vec::new(),
    "procedures_call" => {
      let proccode = get_mutation_value(block, "proccode").unwrap_or_else(|| "my block".to_string());
      let arg_ids = parse_string_array(get_mutation_value(block, "argumentids"));
      let rendered_slots: Vec<String> = arg_ids
        .iter()
        .map(|arg_id| procedure_slot_token(&input_text(arg_id, &mut expr_seen)))
        .collect();
      let label = fill_procedure_placeholders(&proccode, &rendered_slots);
      vec![format!("{indent}call {label}")]
    }
    "music_menu_INSTRUMENT" => Vec::new(),
    "sensing_keyoptions" => Vec::new(),
    "control_create_clone_of_menu" => Vec::new(),
    "note" => Vec::new(),
    _ => vec![format!("{indent}// unsupported opcode: {opcode}")],
  }
}

fn is_top_level_script(block: &Map<String, Value>) -> bool {
  let is_top_level = block.get("topLevel").and_then(Value::as_bool).unwrap_or(false);
  let parent_is_none = block.get("parent").map(Value::is_null).unwrap_or(true);
  is_top_level && parent_is_none
}

fn top_level_script_ids(blocks: &Map<String, Value>) -> Vec<String> {
  let mut ids = Vec::new();

  for (id, block_value) in blocks {
    if let Some(block) = block_value.as_object() {
      if is_top_level_script(block) {
        ids.push(id.clone());
      }
    }
  }

  ids.sort();
  ids
}

fn target_kind_label(is_stage: bool) -> &'static str {
  if is_stage {
    "Stage (Backdrop)"
  } else {
    "Sprite"
  }
}

fn format_script_header(meta: &ScriptCatalogItem) -> String {
  format!(
    "// [{}] {}: {} - Script {}",
    meta.number, meta.target_kind, meta.target_name, meta.local_number
  )
}

fn is_diagnostic_line(line: &str) -> bool {
  line.starts_with("// unsupported opcode:")
    || line.starts_with("// legacy opcode:")
    || line.starts_with("// missing block")
    || line.starts_with("// cycle detected at block")
}

fn is_effectively_empty_script(lines: &[String]) -> bool {
  !lines.iter().any(|line| {
    let trimmed = line.trim();
    !trimmed.is_empty() && !is_diagnostic_line(trimmed)
  })
}

fn is_supported_image_format(data_format: &str) -> bool {
  matches!(data_format, "png" | "jpg" | "jpeg" | "svg")
}

fn collect_image_assets(root: &Value) -> Result<Vec<ImageAssetCatalogItem>, String> {
  let targets = root
    .get("targets")
    .and_then(Value::as_array)
    .ok_or_else(|| "project.json has no targets array.".to_string())?;

  let mut assets_out: Vec<ImageAssetCatalogItem> = Vec::new();
  let mut global_number: usize = 0;

  for target in targets {
    let Some(target_obj) = target.as_object() else {
      continue;
    };

    let target_name = target_obj
      .get("name")
      .and_then(Value::as_str)
      .unwrap_or("Unnamed Target")
      .to_string();

    let is_stage = target_obj
      .get("isStage")
      .and_then(Value::as_bool)
      .unwrap_or(false);

    let Some(costumes) = target_obj.get("costumes").and_then(Value::as_array) else {
      continue;
    };

    let mut local_number: usize = 0;

    for costume in costumes {
      let Some(costume_obj) = costume.as_object() else {
        continue;
      };

      let data_format = costume_obj
        .get("dataFormat")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();

      if !is_supported_image_format(&data_format) {
        continue;
      }

      let asset_id = costume_obj
        .get("assetId")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_string();

      let mut md5ext = costume_obj
        .get("md5ext")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_string();

      if md5ext.is_empty() && !asset_id.is_empty() {
        md5ext = format!("{asset_id}.{data_format}");
      }

      if md5ext.is_empty() {
        continue;
      }

      local_number += 1;
      global_number += 1;

      let asset_name = costume_obj
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("Unnamed Asset")
        .to_string();

      assets_out.push(ImageAssetCatalogItem {
        number: global_number,
        local_number,
        target_name: target_name.clone(),
        target_kind: target_kind_label(is_stage).to_string(),
        is_stage,
        asset_kind: if is_stage {
          "backdrop".to_string()
        } else {
          "costume".to_string()
        },
        asset_name,
        asset_id,
        md5ext,
        data_format,
      });
    }
  }

  Ok(assets_out)
}

fn collect_numbered_scripts(root: &Value) -> Result<Vec<NumberedScript>, String> {
  let targets = root
    .get("targets")
    .and_then(Value::as_array)
    .ok_or_else(|| "project.json has no targets array.".to_string())?;

  let mut scripts_out: Vec<NumberedScript> = Vec::new();
  let mut global_number: usize = 0;

  for target in targets {
    let Some(target_obj) = target.as_object() else {
      continue;
    };

    let name = target_obj
      .get("name")
      .and_then(Value::as_str)
      .unwrap_or("Unnamed Target")
      .to_string();

    let is_stage = target_obj
      .get("isStage")
      .and_then(Value::as_bool)
      .unwrap_or(false);

    let Some(blocks) = target_obj.get("blocks").and_then(Value::as_object) else {
      continue;
    };

    let top_scripts = top_level_script_ids(blocks);
    let mut local_number: usize = 0;

    for top_id in &top_scripts {
      let lines = render_stack(blocks, top_id, "", &HashSet::new());
      if is_effectively_empty_script(&lines) {
        continue;
      }

      local_number += 1;
      global_number += 1;

      let meta = ScriptCatalogItem {
        number: global_number,
        local_number,
        target_name: name.clone(),
        target_kind: target_kind_label(is_stage).to_string(),
        is_stage,
      };

      scripts_out.push(NumberedScript { meta, lines });
    }
  }

  Ok(scripts_out)
}

fn parse_positive_number_arg(raw_bytes: &[u8], label: &str) -> Result<usize, String> {
  let raw = std::str::from_utf8(raw_bytes)
    .map_err(|err| format!("{label} argument is not valid UTF-8: {err}"))?;
  let trimmed = raw.trim();
  if trimmed.is_empty() {
    return Err(format!("{label} argument is empty."));
  }

  let parsed: usize = trimmed
    .parse()
    .map_err(|_| format!("{label} '{trimmed}' is not a valid positive integer."))?;

  if parsed == 0 {
    return Err(format!("{label} must be >= 1."));
  }

  Ok(parsed)
}

fn parse_non_empty_utf8_arg(raw_bytes: &[u8], label: &str) -> Result<String, String> {
  let raw = std::str::from_utf8(raw_bytes)
    .map_err(|err| format!("{label} argument is not valid UTF-8: {err}"))?;
  let trimmed = raw.trim();
  if trimmed.is_empty() {
    return Err(format!("{label} argument is empty."));
  }

  Ok(trimmed.to_string())
}

fn parse_script_number(script_number_bytes: &[u8]) -> Result<usize, String> {
  parse_positive_number_arg(script_number_bytes, "script number")
}

fn extract_project_json_raw(sb3_bytes: &[u8]) -> Result<String, String> {
  let reader = Cursor::new(sb3_bytes);
  let mut archive =
    ZipArchive::new(reader).map_err(|err| format!("Could not open sb3 (zip): {err}"))?;

  let mut project_json = String::new();

  if let Ok(mut file) = archive.by_name("project.json") {
    file
      .read_to_string(&mut project_json)
      .map_err(|err| format!("Failed to read project.json as UTF-8 text: {err}"))?;
    return Ok(project_json);
  }

  if let Ok(mut file) = archive.by_name("PROJECT.JSON") {
    file
      .read_to_string(&mut project_json)
      .map_err(|err| format!("Failed to read PROJECT.JSON as UTF-8 text: {err}"))?;
    return Ok(project_json);
  }

  Err("project.json not found inside sb3 archive.".to_string())
}

fn extract_zip_file_bytes_raw(sb3_bytes: &[u8], filename: &str) -> Result<Vec<u8>, String> {
  let reader = Cursor::new(sb3_bytes);
  let mut archive =
    ZipArchive::new(reader).map_err(|err| format!("Could not open sb3 (zip): {err}"))?;

  let mut out: Vec<u8> = Vec::new();

  if let Ok(mut file) = archive.by_name(filename) {
    file
      .read_to_end(&mut out)
      .map_err(|err| format!("Failed to read '{filename}' from sb3 archive: {err}"))?;
    return Ok(out);
  }

  let upper = filename.to_ascii_uppercase();
  if upper != filename {
    if let Ok(mut file) = archive.by_name(&upper) {
      file
        .read_to_end(&mut out)
        .map_err(|err| format!("Failed to read '{filename}' from sb3 archive: {err}"))?;
      return Ok(out);
    }
  }

  Err(format!("file '{filename}' not found inside sb3 archive."))
}

fn find_attr_span(tag: &str, attr: &str) -> Option<(usize, usize)> {
  let pattern = format!("{attr}=\"");
  let start = tag.find(&pattern)?;
  let value_start = start + pattern.len();
  let end_rel = tag[value_start..].find('"')?;
  Some((value_start, value_start + end_rel))
}

fn get_attr_value(tag: &str, attr: &str) -> Option<String> {
  let (start, end) = find_attr_span(tag, attr)?;
  Some(tag[start..end].to_string())
}

fn set_or_insert_attr(tag: &mut String, attr: &str, value: &str) {
  if let Some((start, end)) = find_attr_span(tag, attr) {
    tag.replace_range(start..end, value);
    return;
  }

  if let Some(pos) = tag.rfind('>') {
    let insertion = format!(" {attr}=\"{value}\"");
    tag.insert_str(pos, &insertion);
  }
}

fn parse_svg_length(raw: &str) -> Option<f64> {
  let trimmed = raw.trim();
  if trimmed.is_empty() {
    return None;
  }

  let without_px = trimmed.strip_suffix("px").unwrap_or(trimmed).trim();
  without_px.parse::<f64>().ok()
}

fn parse_viewbox(raw: &str) -> Option<[f64; 4]> {
  let normalized = raw.replace(',', " ");
  let values: Vec<f64> = normalized
    .split_whitespace()
    .filter_map(|part| part.parse::<f64>().ok())
    .collect();

  if values.len() == 4 {
    Some([values[0], values[1], values[2], values[3]])
  } else {
    None
  }
}

fn format_svg_number(value: f64) -> String {
  if (value - value.round()).abs() < 1e-9 {
    return format!("{:.0}", value.round());
  }

  let mut out = format!("{value:.6}");
  while out.contains('.') && out.ends_with('0') {
    out.pop();
  }
  if out.ends_with('.') {
    out.pop();
  }
  out
}

fn normalize_svg_for_typst(svg_text: &str) -> String {
  let Some(svg_start) = svg_text.find("<svg") else {
    return svg_text.to_string();
  };
  let Some(svg_end_rel) = svg_text[svg_start..].find('>') else {
    return svg_text.to_string();
  };

  let svg_end = svg_start + svg_end_rel;
  let mut tag = svg_text[svg_start..=svg_end].to_string();

  let mut width = get_attr_value(&tag, "width").and_then(|raw| parse_svg_length(&raw));
  let mut height = get_attr_value(&tag, "height").and_then(|raw| parse_svg_length(&raw));

  if width.unwrap_or(0.0) <= 0.0 {
    width = Some(1.0);
  }
  if height.unwrap_or(0.0) <= 0.0 {
    height = Some(1.0);
  }

  let mut viewbox = get_attr_value(&tag, "viewBox").and_then(|raw| parse_viewbox(&raw));
  if let Some(parsed) = viewbox {
    if parsed[2] <= 0.0 || parsed[3] <= 0.0 {
      viewbox = None;
    }
  }

  let width_val = width.unwrap_or(1.0);
  let height_val = height.unwrap_or(1.0);

  if viewbox.is_none() {
    viewbox = Some([0.0, 0.0, width_val, height_val]);
  }

  let view = viewbox.unwrap_or([0.0, 0.0, 1.0, 1.0]);

  set_or_insert_attr(&mut tag, "width", &format_svg_number(width_val));
  set_or_insert_attr(&mut tag, "height", &format_svg_number(height_val));
  set_or_insert_attr(
    &mut tag,
    "viewBox",
    &format!(
      "{} {} {} {}",
      format_svg_number(view[0]),
      format_svg_number(view[1]),
      format_svg_number(view[2]),
      format_svg_number(view[3])
    ),
  );

  let mut out = String::new();
  out.push_str(&svg_text[..svg_start]);
  out.push_str(&tag);
  out.push_str(&svg_text[svg_end + 1..]);
  out
}

fn normalize_svg_bytes_for_typst(bytes: Vec<u8>) -> Vec<u8> {
  let Ok(text) = String::from_utf8(bytes.clone()) else {
    return bytes;
  };

  normalize_svg_for_typst(&text).into_bytes()
}

fn extract_scripts_json_raw(project_json: &str) -> Result<String, String> {
  let root: Value =
    serde_json::from_str(project_json).map_err(|err| format!("Invalid project.json content: {err}"))?;

  let targets = root
    .get("targets")
    .and_then(Value::as_array)
    .ok_or_else(|| "project.json has no targets array.".to_string())?;

  let mut out_targets = Vec::new();

  for target in targets {
    let Some(target_obj) = target.as_object() else {
      continue;
    };

    let name = target_obj
      .get("name")
      .and_then(Value::as_str)
      .unwrap_or("Unnamed Target")
      .to_string();

    let is_stage = target_obj
      .get("isStage")
      .and_then(Value::as_bool)
      .unwrap_or(false);

    let blocks = target_obj
      .get("blocks")
      .and_then(Value::as_object)
      .ok_or_else(|| format!("Target '{name}' has no blocks map."))?;

    out_targets.push(ScriptsTarget {
      name,
      is_stage,
      scripts: top_level_script_ids(blocks),
    });
  }

  serde_json::to_string(&ScriptsDump { targets: out_targets })
    .map_err(|err| format!("Failed to serialize scripts dump: {err}"))
}

fn sb3_to_scratch_text_raw(sb3_bytes: &[u8]) -> Result<String, String> {
  let project_json = extract_project_json_raw(sb3_bytes)?;
  let root: Value =
    serde_json::from_str(&project_json).map_err(|err| format!("Invalid project.json content: {err}"))?;

  let numbered_scripts = collect_numbered_scripts(&root)?;
  let mut output_lines: Vec<String> = Vec::new();

  for script in &numbered_scripts {
    output_lines.push(format_script_header(&script.meta));
    output_lines.extend(script.lines.clone());
    output_lines.push(String::new());
  }

  if output_lines.is_empty() {
    Ok("// No top-level scripts found in this sb3.".to_string())
  } else {
    Ok(output_lines.join("\n").trim().to_string())
  }
}

fn sb3_to_scratch_text_by_number_raw(sb3_bytes: &[u8], script_number: usize) -> Result<String, String> {
  let project_json = extract_project_json_raw(sb3_bytes)?;
  let root: Value =
    serde_json::from_str(&project_json).map_err(|err| format!("Invalid project.json content: {err}"))?;

  let numbered_scripts = collect_numbered_scripts(&root)?;
  if numbered_scripts.is_empty() {
    return Ok("// No top-level scripts found in this sb3.".to_string());
  }

  let Some(script) = numbered_scripts
    .iter()
    .find(|item| item.meta.number == script_number)
  else {
    return Err(format!(
      "script number {} does not exist (available range: 1..={}).",
      script_number,
      numbered_scripts.len()
    ));
  };

  let mut output_lines: Vec<String> = Vec::new();
  output_lines.push(format_script_header(&script.meta));
  output_lines.extend(script.lines.clone());

  Ok(output_lines.join("\n").trim().to_string())
}

fn sb3_scripts_catalog_json_raw(sb3_bytes: &[u8]) -> Result<String, String> {
  let project_json = extract_project_json_raw(sb3_bytes)?;
  let root: Value =
    serde_json::from_str(&project_json).map_err(|err| format!("Invalid project.json content: {err}"))?;

  let numbered_scripts = collect_numbered_scripts(&root)?;
  let catalog = ScriptsCatalog {
    scripts: numbered_scripts.into_iter().map(|entry| entry.meta).collect(),
  };

  serde_json::to_string(&catalog).map_err(|err| format!("Failed to serialize scripts catalog: {err}"))
}

fn sb3_image_assets_catalog_json_raw(sb3_bytes: &[u8]) -> Result<String, String> {
  let project_json = extract_project_json_raw(sb3_bytes)?;
  let root: Value =
    serde_json::from_str(&project_json).map_err(|err| format!("Invalid project.json content: {err}"))?;

  let assets = collect_image_assets(&root)?;
  let catalog = ImageAssetsCatalog { images: assets };

  serde_json::to_string(&catalog).map_err(|err| format!("Failed to serialize image assets catalog: {err}"))
}

fn sb3_image_bytes_by_number_raw(sb3_bytes: &[u8], image_number: usize) -> Result<Vec<u8>, String> {
  let project_json = extract_project_json_raw(sb3_bytes)?;
  let root: Value =
    serde_json::from_str(&project_json).map_err(|err| format!("Invalid project.json content: {err}"))?;

  let assets = collect_image_assets(&root)?;
  if assets.is_empty() {
    return Err("No supported image assets (png/jpg/jpeg/svg) found in this sb3.".to_string());
  }

  let Some(asset) = assets.iter().find(|item| item.number == image_number) else {
    return Err(format!(
      "image number {} does not exist (available range: 1..={}).",
      image_number,
      assets.len()
    ));
  };

  let bytes = extract_zip_file_bytes_raw(sb3_bytes, &asset.md5ext)?;

  if asset.data_format == "svg" {
    Ok(normalize_svg_bytes_for_typst(bytes))
  } else {
    Ok(bytes)
  }
}

fn sb3_image_bytes_by_md5ext_raw(sb3_bytes: &[u8], md5ext: &str) -> Result<Vec<u8>, String> {
  let bytes = extract_zip_file_bytes_raw(sb3_bytes, md5ext)?;
  if md5ext.to_ascii_lowercase().ends_with(".svg") {
    Ok(normalize_svg_bytes_for_typst(bytes))
  } else {
    Ok(bytes)
  }
}

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn extract_project_json(sb3_bytes: &[u8]) -> Vec<u8> {
  match extract_project_json_raw(sb3_bytes) {
    Ok(json_text) => json_text.into_bytes(),
    Err(err) => err_bytes(err),
  }
}

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn extract_scripts_json(project_json_bytes: &[u8]) -> Vec<u8> {
  let project_json = match std::str::from_utf8(project_json_bytes) {
    Ok(value) => value,
    Err(err) => return err_bytes(format!("project_json argument is not valid UTF-8: {err}")),
  };

  match extract_scripts_json_raw(project_json) {
    Ok(json_text) => json_text.into_bytes(),
    Err(err) => err_bytes(err),
  }
}

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn sb3_to_scratch_text(sb3_bytes: &[u8]) -> Vec<u8> {
  match sb3_to_scratch_text_raw(sb3_bytes) {
    Ok(text) => text.into_bytes(),
    Err(err) => err_bytes(err),
  }
}

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn sb3_to_scratch_text_by_number(sb3_bytes: &[u8], script_number_bytes: &[u8]) -> Vec<u8> {
  let script_number = match parse_script_number(script_number_bytes) {
    Ok(value) => value,
    Err(err) => return err_bytes(err),
  };

  match sb3_to_scratch_text_by_number_raw(sb3_bytes, script_number) {
    Ok(text) => text.into_bytes(),
    Err(err) => err_bytes(err),
  }
}

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn sb3_scripts_catalog_json(sb3_bytes: &[u8]) -> Vec<u8> {
  match sb3_scripts_catalog_json_raw(sb3_bytes) {
    Ok(text) => text.into_bytes(),
    Err(err) => err_bytes(err),
  }
}

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn sb3_image_assets_catalog_json(sb3_bytes: &[u8]) -> Vec<u8> {
  match sb3_image_assets_catalog_json_raw(sb3_bytes) {
    Ok(text) => text.into_bytes(),
    Err(err) => err_bytes(err),
  }
}

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn sb3_image_bytes_by_number(sb3_bytes: &[u8], image_number_bytes: &[u8]) -> Vec<u8> {
  let image_number = match parse_positive_number_arg(image_number_bytes, "image number") {
    Ok(value) => value,
    Err(err) => return err_bytes(err),
  };

  match sb3_image_bytes_by_number_raw(sb3_bytes, image_number) {
    Ok(bytes) => bytes,
    Err(err) => err_bytes(err),
  }
}

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn sb3_image_bytes_by_md5ext(sb3_bytes: &[u8], md5ext_bytes: &[u8]) -> Vec<u8> {
  let md5ext = match parse_non_empty_utf8_arg(md5ext_bytes, "md5ext") {
    Ok(value) => value,
    Err(err) => return err_bytes(err),
  };

  match sb3_image_bytes_by_md5ext_raw(sb3_bytes, &md5ext) {
    Ok(bytes) => bytes,
    Err(err) => err_bytes(err),
  }
}
