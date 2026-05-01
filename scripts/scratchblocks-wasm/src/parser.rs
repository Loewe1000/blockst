use crate::model::{BlockSpec, DocumentSpec, ScriptSpec, SegmentSpec};
use crate::render::render_document;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

// TOML data files (embedded at compile time)
const BLOCKS_TOML: &str = include_str!("../data/blocks.toml");

// Locale data loaded from generated module (all 25+ languages)

// TOML deserialization structures
#[derive(Debug, Deserialize)]
struct BlocksToml {
    #[serde(rename = "blocks")]
    blocks: HashMap<String, BlockDef>,
    #[serde(default)]
    defaults: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct BlockDef {
    #[serde(default)]
    shape: String,
    #[serde(default)]
    category: String,
    #[serde(default)]
    inputs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct LocaleToml {
    specs: HashMap<String, String>,
    #[serde(default)]
    aliases: HashMap<String, String>,
}

// Internal structures (same as before, loaded from TOML instead of JSON)

#[derive(Debug)]
struct LanguageRuntime {
    code: String,
    blocks_by_hash: HashMap<String, Vec<String>>,
    native_specs: HashMap<String, String>,
    else_spec: String,
    define_spec: String,
    call_spec: String,
}

#[derive(Debug)]
struct ParserData {
    commands_by_id: HashMap<String, BlockDef>,
    languages: HashMap<String, LanguageRuntime>,
    default_blocks: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct ParseRequest {
    code: String,
    #[serde(default = "default_language")]
    language: String,
    #[serde(default)]
    inline: bool,
    theme: Option<String>,
    scale: Option<f32>,
    #[serde(default)]
    line_numbers: Option<bool>,
    #[serde(default)]
    line_number_start: Option<u32>,
    #[serde(default)]
    line_number_gutter: Option<f32>,
    #[serde(default)]
    inset_scale: Option<f32>,
    #[serde(default)]
    widths: Option<HashMap<String, f32>>,
    #[serde(default = "default_font")]
    font: String,
}

fn default_language() -> String {
    "en".to_string()
}

fn default_font() -> String {
    "Helvetica Neue, Helvetica, sans-serif".to_string()
}

#[derive(Debug, Clone)]
enum Child {
    Label(String),
    Icon(String),
    Input(InputNode),
}

#[derive(Debug, Clone)]
struct InputNode {
    shape: String,
    value: String,
    nested: Option<Box<ParsedBlock>>,
}

#[derive(Debug, Clone)]
pub(crate) struct ParsedBlock {
    id: String,
    selector: String,
    shape: String,
    category: String,
    language: String,
    children: Vec<Child>,
    body: Vec<ParsedBlock>,
    else_body: Vec<ParsedBlock>,
    has_else: bool,
    line_number: Option<u32>,
    line_label: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PublicNode {
    pub id: String,
    pub selector: String,
    pub shape: String,
    pub category: String,
    pub parts: Vec<PublicPart>,
    #[serde(default, rename = "body")]
    pub body: Vec<PublicNode>,
    #[serde(default, rename = "else-body")]
    pub else_body: Vec<PublicNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind")]
pub enum PublicPart {
    #[serde(rename = "label")]
    Label { value: String },
    #[serde(rename = "icon")]
    Icon { name: String },
    #[serde(rename = "input")]
    Input {
        shape: String,
        value: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        nested: Option<Box<PublicNode>>,
    },
}

static DATA: OnceLock<ParserData> = OnceLock::new();

pub fn parse_request_json(input: &str) -> Result<String, String> {
    let request: ParseRequest = serde_json::from_str(input)
        .map_err(|err| format!("scratchblocks-wasm: invalid parse request: {err}"))?;
    let nodes = parse_code(&request.code, &request.language, request.inline)?;
    serde_json::to_string(&nodes)
        .map_err(|err| format!("scratchblocks-wasm: failed to encode parse result: {err}"))
}

pub fn render_request_json(input: &str) -> Result<String, String> {
    let request: ParseRequest = serde_json::from_str(input)
        .map_err(|err| format!("scratchblocks-wasm: invalid render request: {err}"))?;
    let scripts = parse_internal(&request.code, &request.language, request.inline)?;
    let document = DocumentSpec {
        scale: request.scale,
        theme: request.theme,
        line_numbers: request.line_numbers.unwrap_or(false),
        line_number_start: request.line_number_start.unwrap_or(1),
        line_number_gutter: request.line_number_gutter.unwrap_or(24.0),
        inset_scale: request.inset_scale.unwrap_or(1.0),
        font: if request.font.is_empty() { default_font() } else { request.font },
        scripts: scripts
            .into_iter()
            .map(|blocks| ScriptSpec {
                blocks: blocks.into_iter().map(to_render_block).collect(),
            })
            .collect(),
    };
    crate::measure::set_inset_scale(document.inset_scale);
    let result = if let Some(widths) = request.widths {
        if !widths.is_empty() {
            crate::measure::set_widths(widths);
            let rendered = Ok(render_document(&document));
            crate::measure::clear_widths();
            rendered
        } else {
            Ok(render_document(&document))
        }
    } else {
        Ok(render_document(&document))
    };
    crate::measure::clear_inset_scale();
    result
}

/// Parse scratch code and return a JSON array of all unique text strings
/// that would appear in the rendered output. Used by Typst to pre-measure
/// text widths with the actual font.
pub fn extract_texts_json(input: &str) -> Result<String, String> {
    let request: ParseRequest = serde_json::from_str(input)
        .map_err(|err| format!("scratchblocks-wasm: invalid extract request: {err}"))?;
    let scripts = parse_internal(&request.code, &request.language, request.inline)?;
    let mut texts = std::collections::BTreeSet::new();
    for script in &scripts {
        collect_texts(script, &mut texts);
    }
    let result: Vec<String> = texts.into_iter().collect();
    serde_json::to_string(&result)
        .map_err(|err| format!("scratchblocks-wasm: failed to encode texts: {err}"))
}

fn collect_texts(blocks: &[ParsedBlock], texts: &mut std::collections::BTreeSet<String>) {
    for block in blocks {
        for child in &block.children {
            match child {
                Child::Label(value) => { texts.insert(value.clone()); }
                Child::Input(input) => {
                    if !input.value.is_empty() {
                        texts.insert(input.value.clone());
                    }
                    if let Some(ref nested) = input.nested {
                        collect_texts(&[nested.as_ref().clone()], texts);
                    }
                }
                _ => {}
            }
        }
        collect_texts(&block.body, texts);
        collect_texts(&block.else_body, texts);
    }
}

fn data() -> &'static ParserData {
    DATA.get_or_init(|| {
        let blocks_file: BlocksToml = toml::from_str(BLOCKS_TOML).expect("blocks.toml");
        let commands_by_id = blocks_file.blocks;
        let default_blocks = blocks_file.defaults;

        let mut languages: HashMap<String, LanguageRuntime> = HashMap::new();

        for &(code, locale_toml) in crate::generated::LOCALE_DATA {
            let locale: LocaleToml = toml::from_str(locale_toml)
                .unwrap_or_else(|e| panic!("locales/{code}.toml: {e}"));
            let native_specs: HashMap<String, String> = locale.specs;

            let else_spec = native_specs
                .get("control_else")
                .cloned()
                .unwrap_or_else(|| "else".to_string());

            // Extract define/call keywords from locale templates
            // Template format: "define %1" → keyword = "define"
            let define_spec = native_specs
                .get("procedures_definition")
                .and_then(|s| s.split('%').next())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "define".to_string());
            let call_spec = native_specs
                .get("procedures_call")
                .and_then(|s| s.split('%').next())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "call".to_string());

            let mut blocks_by_hash: HashMap<String, Vec<String>> = HashMap::new();

            for (block_id, spec) in &native_specs {
                blocks_by_hash.entry(hash_spec(spec)).or_default().push(block_id.clone());
                if let Some(block_def) = commands_by_id.get(block_id) {
                    if !block_def.inputs.is_empty() {
                        let typed_hash = build_typed_hash(spec, &block_def.inputs);
                        blocks_by_hash.entry(typed_hash).or_default().push(block_id.clone());
                    }
                    if let Some(icon) = extract_icon(spec) {
                        let icon_hash = hash_spec(spec).replace(&hash_spec(icon), unicode_icon(icon));
                        blocks_by_hash.entry(icon_hash).or_default().push(block_id.clone());
                    }
                    // Also register reverse: if spec contains unicode icons,
                    // register the @iconName version too
                    if let Some(at_name) = unicode_to_at_spec(spec) {
                        let at_hash = hash_spec(&at_name);
                        blocks_by_hash.entry(at_hash).or_default().push(block_id.clone());
                        // Also register typed hash for the @ version
                        if !block_def.inputs.is_empty() {
                            let typed_hash = build_typed_hash(&at_name, &block_def.inputs);
                            blocks_by_hash.entry(typed_hash).or_default().push(block_id.clone());
                        }
                    }
                }
            }

            // Register aliases (e.g. Scratch 2 → Scratch 3 name mappings)
            for (alias_spec, block_id) in &locale.aliases {
                blocks_by_hash.entry(hash_spec(alias_spec)).or_default().push(block_id.clone());
                if let Some(block_def) = commands_by_id.get(block_id) {
                    if !block_def.inputs.is_empty() {
                        let typed_hash = build_typed_hash(alias_spec, &block_def.inputs);
                        blocks_by_hash.entry(typed_hash).or_default().push(block_id.clone());
                    }
                    if let Some(icon) = extract_icon(alias_spec) {
                        let icon_hash = hash_spec(alias_spec).replace(&hash_spec(icon), unicode_icon(icon));
                        blocks_by_hash.entry(icon_hash).or_default().push(block_id.clone());
                    }
                    if let Some(at_name) = unicode_to_at_spec(alias_spec) {
                        let at_hash = hash_spec(&at_name);
                        blocks_by_hash.entry(at_hash).or_default().push(block_id.clone());
                    }
                }
            }

            languages.insert(
                code.to_string(),
                LanguageRuntime {
                    code: code.to_string(),
                    blocks_by_hash,
                    native_specs,
                    else_spec,
                    define_spec,
                    call_spec,
                },
            );
        }

        ParserData { commands_by_id, languages, default_blocks }
    })
}

fn parse_code(code: &str, language: &str, inline: bool) -> Result<Vec<PublicNode>, String> {
    Ok(parse_internal(code, language, inline)?
        .into_iter()
        .flat_map(|blocks| blocks.into_iter().map(to_public_node))
        .collect())
}

pub(crate) fn parse_internal(code: &str, language: &str, inline: bool) -> Result<Vec<Vec<ParsedBlock>>, String> {
    let requested = data()
        .languages
        .get(language)
        .ok_or_else(|| format!("scratchblocks-wasm: unknown language '{language}'"))?;
    let english = data().languages.get("en").expect("english");
    let mut parser = Parser::new(
        if inline { code.replace('\n', " ") } else { code.to_string() }
            .replace("&lt;", "<")
            .replace("&gt;", ">"),
        if language == "en" { vec![english] } else { vec![requested, english] },
    );
    let mut scripts = parser.parse_file()?;
    let mut line = 1u32;
    for script in &mut scripts {
        assign_line_numbers(script, &mut line);
    }
    validate_unique_labels(&scripts)?;
    Ok(scripts)
}

fn assign_line_numbers(blocks: &mut [ParsedBlock], line: &mut u32) {
    for block in blocks {
        block.line_number = Some(*line);
        *line += 1;
        assign_line_numbers(&mut block.body, line);
        assign_line_numbers(&mut block.else_body, line);
    }
}

fn validate_unique_labels(scripts: &[Vec<ParsedBlock>]) -> Result<(), String> {
    let mut seen = std::collections::BTreeSet::new();
    for script in scripts {
        collect_labels(script, &mut seen)?;
    }
    Ok(())
}

fn collect_labels(blocks: &[ParsedBlock], seen: &mut std::collections::BTreeSet<String>) -> Result<(), String> {
    for block in blocks {
        if let Some(label) = &block.line_label {
            if !seen.insert(label.clone()) {
                return Err(format!("scratchblocks-wasm: duplicate line label '#{label}'."));
            }
        }
        collect_labels(&block.body, seen)?;
        collect_labels(&block.else_body, seen)?;
    }
    Ok(())
}

fn effect_dropdown_value(children: &[Child]) -> Option<&str> {
    children.iter().find_map(|child| match child {
        Child::Input(input) if matches!(input.shape.as_str(), "dropdown" | "number-dropdown") => {
            Some(input.value.trim())
        }
        _ => None,
    })
}

fn is_sound_effect_value(value: &str) -> bool {
    let lower = value.trim().to_lowercase();
    lower == "pitch"
        || lower.contains("pan")
        || lower.contains("balance")
        || lower.contains("höhe")
        || lower.contains("hauteur")
}

fn is_looks_effect_value(value: &str) -> bool {
    let lower = value.trim().to_lowercase();
    lower == "color"
        || lower == "farbe"
        || lower == "couleur"
        || lower == "fisheye"
        || lower == "fischauge"
        || lower == "oeil de poisson"
        || lower == "whirl"
        || lower == "wirbel"
        || lower == "tourbillon"
        || lower == "pixelate"
        || lower == "pixeln"
        || lower == "pixelliser"
        || lower == "mosaic"
        || lower == "mosaik"
        || lower == "mosaïque"
        || lower == "brightness"
        || lower == "helligkeit"
        || lower == "luminosité"
        || lower == "ghost"
        || lower == "durchsichtigkeit"
        || lower == "fantôme"
        || lower == "fantome"
        || lower == "transparence"
}

fn prefer_effect_block_candidate<'a>(ids: &'a [String], children: &[Child]) -> Option<&'a str> {
    let dropdown = effect_dropdown_value(children)?;
    let preferred_prefix = if is_sound_effect_value(dropdown) {
        Some("SOUND_")
    } else if is_looks_effect_value(dropdown) {
        Some("LOOKS_")
    } else {
        None
    }?;

    ids.iter()
        .find(|id| id.starts_with(preferred_prefix))
        .map(|id| id.as_str())
}

fn to_public_node(block: ParsedBlock) -> PublicNode {
    PublicNode {
        id: block.id,
        selector: block.selector,
        shape: block.shape,
        category: normalize_category(&block.category).to_string(),
        parts: block
            .children
            .into_iter()
            .map(|child| match child {
                Child::Label(value) => PublicPart::Label { value },
                Child::Icon(name) => PublicPart::Icon { name },
                Child::Input(input) => PublicPart::Input {
                    shape: input.shape,
                    value: input.value,
                    nested: input.nested.map(|nested| Box::new(to_public_node(*nested))),
                },
            })
            .collect(),
        body: block.body.into_iter().map(to_public_node).collect(),
        else_body: block.else_body.into_iter().map(to_public_node).collect(),
        line: block.line_number,
        label: block.line_label,
    }
}

/// Returns true if dropdown inputs on this block should be rendered as
/// square field-dropdowns (same fill as block) rather than round input-value
/// dropdowns (darker fill). Based on scratch-blocks JSON definitions:
/// field_dropdown and field_variable types.
/// Returns true if the dropdown at the given input index in this block should
/// be rendered as square (field_dropdown, same fill as block) rather than
/// round (input_value shadow, darker fill).
fn is_square_dropdown_index(block_id: &str, input_idx: usize) -> bool {
    match block_id {
        // Pen color-parameter dropdown (color/saturation/brightness/transparency)
        // is an input-value dropdown and should stay round.
        "pen.changeColorParam" | "pen.setColorParam" => false,
        // SENSING_OF: first input (%m.attribute = PROPERTY field_dropdown) is
        // square, second (%m.spriteOrStage = OBJECT input_value shadow) is round.
        "SENSING_OF" => input_idx == 0,
        _ => matches!(block_id,
            "EVENT_WHENKEYPRESSED" |
            "EVENT_WHENGREATERTHAN" |
            "EVENT_WHENBACKDROPSWITCHESTO" |
            "EVENT_WHENBROADCASTRECEIVED" |
            "OPERATORS_MATHOP" |
            "SOUND_SETEFFECTO" | "SOUND_CHANGEEFFECTBY" |
            "LOOKS_SETEFFECTTO" | "LOOKS_CHANGEEFFECTBY" |
            "LOOKS_GOTOFRONTBACK" |
            "LOOKS_GOFORWARDBACKWARDLAYERS" |
            "LOOKS_COSTUMENUMBERNAME" |
            "LOOKS_BACKDROPNUMBERNAME" |
            "SENSING_CURRENT" |
            "SENSING_SETDRAGMODE" |
            "videoSensing.videoOn" |
            "DATA_ITEMOFLIST" |
            "DATA_ITEMNUMOFLIST" |
            "DATA_LENGTHOFLIST" |
            "DATA_LISTCONTAINSITEM" |
            "DATA_ADDTOLIST" |
            "DATA_DELETEOFLIST" |
            "DATA_DELETEALLOFLIST" |
            "DATA_INSERTATLIST" |
            "DATA_REPLACEITEMOFLIST" |
            "DATA_SHOWLIST" |
            "DATA_HIDELIST" |
            "DATA_SHOWVARIABLE" |
            "DATA_HIDEVARIABLE" |
            "DATA_SETVARIABLETO" |
            "DATA_CHANGEVARIABLEBY"
        )
    }
}

fn to_render_block(block: ParsedBlock) -> BlockSpec {
    let has_else_body = block.has_else;
    let mut segments = Vec::new();
    let mut dropdown_idx = 0;
    let is_proc_def = block.id == "procedures_definition";
    for child in block.children {
        match child {
            Child::Label(value) => {
                segments.push(SegmentSpec::Text { value })
            }
            Child::Icon(name) => segments.push(SegmentSpec::Icon { name }),
            Child::Input(input) => {
                // Reporter inputs with a nested block are rendered as inline blocks
                if input.shape == "reporter" || input.shape == "boolean" {
                    if let Some(nested) = input.nested {
                        // In procedure definitions, convert unrecognized reporter args
                        // to custom-arg blocks (pink outline)
                        let render_block = if is_proc_def && nested.id == "unrecognized" {
                            let mut custom_block = nested.as_ref().clone();
                            custom_block.category = "custom-arg".to_string();
                            custom_block.shape = input.shape.clone();
                            custom_block.id = "argument_reporter_string_number".to_string();
                            Box::new(to_render_block(custom_block))
                        } else {
                            Box::new(to_render_block(*nested))
                        };
                        segments.push(SegmentSpec::Block { block: render_block });
                        continue;
                    }
                }
                let is_dropdown_input = matches!(input.shape.as_str(), "dropdown" | "number-dropdown");
                // In procedure definitions, dropdown args become custom-arg reporters
                if is_proc_def && is_dropdown_input {
                    let custom_block = ParsedBlock {
                        id: "argument_reporter_string_number".to_string(),
                        selector: String::new(),
                        shape: "reporter".to_string(),
                        category: "custom-arg".to_string(),
                        language: block.language.clone(),
                        children: vec![Child::Label(input.value.clone())],
                        body: Vec::new(),
                        else_body: Vec::new(),
                        has_else: false,
                        line_number: None,
                        line_label: None,
                    };
                    segments.push(SegmentSpec::Block { block: Box::new(to_render_block(custom_block)) });
                    continue;
                }
                let square = is_dropdown_input && is_square_dropdown_index(&block.id, dropdown_idx);
                if is_dropdown_input { dropdown_idx += 1; }
                segments.push(SegmentSpec::Input {
                    input: match input.shape.as_str() {
                        "number" => "number",
                        "number-dropdown" | "dropdown" => {
                            if square { "dropdown-field" } else { "dropdown" }
                        },
                        "color" => "color",
                        "boolean" => "boolean",
                        _ => "string",
                    }
                    .to_string(),
                    value: input.value.clone(),
                    color: if input.shape == "color" { input.value } else { String::new() },
                    nested: input.nested.map(|b| Box::new(to_render_block(*b))),
                });
            }
        }
    }
    let else_segs: Vec<SegmentSpec> = if has_else_body {
        let else_label = data().languages
            .get(block.language.as_str())
            .map(|l| l.else_spec.as_str())
            .unwrap_or("else");
        vec![SegmentSpec::Text { value: else_label.to_string() }]
    } else {
        Vec::new()
    };
    BlockSpec {
        shape: normalize_shape(&block.shape).to_string(),
        category: normalize_category(&block.category).to_string(),
        line_number: block.line_number,
        segments,
        body: block.body.into_iter().map(to_render_block).collect(),
        else_body: block.else_body.into_iter().map(to_render_block).collect(),
        else_segments: else_segs,
    }
}

fn normalize_shape(shape: &str) -> &str {
    match shape {
        "c-block" | "c-block e-block" => "c-block",
        "c-block cap" => "c-block cap",
        "hat" => "hat",
        "define-hat" => "define-hat",
        "cap" => "cap",
        "boolean" => "boolean",
        "reporter" | "ring" => "reporter",
        _ => "stack",
    }
}

fn normalize_category(category: &str) -> &str {
    match category {
        "event" => "events",
        "list" => "lists",
        "operator" => "operators",
        other => other,
    }
}

fn normalize_category_token(category: &str) -> &str {
    match category {
        "event" => "events",
        "list" => "lists",
        "operator" => "operators",
        "variable" => "variables",
        other => other,
    }
}

fn extract_line_label(children: &mut Vec<Child>) -> Result<Option<String>, String> {
    let Some(last) = children.last() else {
        return Ok(None);
    };
    let Child::Label(value) = last else {
        return Ok(None);
    };
    if !value.starts_with('#') {
        return Ok(None);
    }

    let raw = value.trim_start_matches('#').to_string();
    if raw.is_empty() {
        return Err("scratchblocks-wasm: empty line label '#'. Use '#name'.".to_string());
    }
    if !raw.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-') {
        return Err(format!(
            "scratchblocks-wasm: invalid line label '#{raw}'. Allowed: a-z, A-Z, 0-9, _, -"
        ));
    }

    children.pop();
    Ok(Some(raw))
}

struct Parser<'a> {
    chars: Vec<char>,
    index: usize,
    languages: Vec<&'a LanguageRuntime>,
}

impl<'a> Parser<'a> {
    fn match_block_candidate(&self, children: &[Child], forced_category: Option<&str>) -> Option<(String, usize)> {
        let hash = minify_hash(&children_hash(children));
        let typed_hash = minify_hash(&children_typed_hash(children));
        let has_dropdown_input = children
            .iter()
            .any(|c| matches!(c, Child::Input(i) if i.shape == "dropdown"));

        const CATEGORY_PRIORITY: &[&str] = &[
            "grey", "obsolete", "music", "pen", "list", "variables", "operators", "sensing", "control", "events", "sound",
            "looks", "motion",
        ];

        fn category_rank(cat: &str) -> usize {
            CATEGORY_PRIORITY
                .iter()
                .position(|&c| c == cat)
                .map(|i| i + 1)
                .unwrap_or(0)
        }

        for (lang_idx, language) in self.languages.iter().enumerate() {
            let lookup_key = if language.blocks_by_hash.contains_key(&typed_hash) {
                &typed_hash
            } else {
                &hash
            };

            let Some(ids) = language.blocks_by_hash.get(lookup_key) else {
                continue;
            };

            let candidate_ids: Vec<String> = ids
                .iter()
                .filter(|id| {
                    let Some(block_def) = data().commands_by_id.get(*id) else {
                        return false;
                    };
                    if let Some(category) = forced_category {
                        normalize_category(&block_def.category) == category
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();

            if candidate_ids.is_empty() {
                continue;
            }

            if let Some(preferred_id) = prefer_effect_block_candidate(&candidate_ids, children) {
                return Some((preferred_id.to_string(), lang_idx));
            }

            let mut best: Option<&str> = None;
            for id in &candidate_ids {
                if let Some(block_def) = data().commands_by_id.get(id) {
                    if has_dropdown_input && block_def.category == "list" {
                        best = Some(id);
                        break;
                    }
                    if let Some(current_best) = best {
                        let current_rank = data()
                            .commands_by_id
                            .get(current_best)
                            .map(|b| category_rank(&b.category))
                            .unwrap_or(0);
                        if category_rank(&block_def.category) > current_rank {
                            best = Some(id);
                        }
                    } else {
                        best = Some(id);
                    }
                }
            }

            if let Some(best_id) = best {
                return Some((best_id.to_string(), lang_idx));
            }
        }

        None
    }

    fn new(code: String, languages: Vec<&'a LanguageRuntime>) -> Self {
        Self { chars: code.chars().collect(), index: 0, languages }
    }

    fn parse_file(&mut self) -> Result<Vec<Vec<ParsedBlock>>, String> {
        let mut lines = Vec::new();
        while self.peek().is_some() {
            if self.peek() == Some('\n') {
                self.next();
                continue;
            }
            if let Some(line) = self.parse_line()? {
                lines.push(line);
            }
            if self.peek() == Some('\n') {
                self.next();
            }
        }

        let mut index = 0;
        let mut scripts = Vec::new();
        while index < lines.len() {
            let mut blocks = Vec::new();
            while index < lines.len() {
                if lines[index].shape == "hat" && !blocks.is_empty() {
                    break;
                }
                let block = self.attach_bodies(&lines, &mut index)?;
                let is_command = block.shape.contains("stack") || block.shape.contains("hat") || block.shape.contains("cap") || block.shape.contains("block");
                let is_hat = block.shape == "hat";
                // "c-block cap" (like forever) has a rounded bottom — it terminates a script
                let is_terminal_cap = block.shape == "cap" || block.shape == "c-block cap";
                blocks.push(block);
                if !is_command {
                    break;
                }
                if index >= lines.len() || (lines[index].shape == "hat" && !is_hat) || is_terminal_cap {
                    break;
                }
            }
            if !blocks.is_empty() {
                scripts.push(blocks);
            }
        }
        Ok(scripts)
    }

    fn attach_bodies(&self, lines: &[LineBlock], index: &mut usize) -> Result<ParsedBlock, String> {
        let mut block = lines[*index].block.clone();
        *index += 1;
        if !block.shape.contains("block") {
            return Ok(block);
        }
        block.body = self.parse_mouth(lines, index)?;
        if *index < lines.len() && lines[*index].shape == "celse" {
            *index += 1;
            block.else_body = self.parse_mouth(lines, index)?;
            block.has_else = true;
        }
        if *index < lines.len() && lines[*index].shape == "cend" {
            *index += 1;
        }
        Ok(block)
    }

    fn parse_mouth(&self, lines: &[LineBlock], index: &mut usize) -> Result<Vec<ParsedBlock>, String> {
        let mut blocks = Vec::new();
        while *index < lines.len() {
            let shape = lines[*index].shape.as_str();
            if matches!(shape, "celse" | "cend" | "hat") {
                break;
            }
            blocks.push(self.attach_bodies(lines, index)?);
        }
        Ok(blocks)
    }

    fn parse_line(&mut self) -> Result<Option<LineBlock>, String> {
        let mut children = self.parse_parts(None)?;
        if children.is_empty() {
            return Ok(None);
        }
        let line_label = extract_line_label(&mut children)?;
        if children.is_empty() {
            return Ok(None);
        }
        if let Some(default_block) = self.try_category_default(&children)? {
            return Ok(Some(LineBlock {
                shape: default_block.shape.clone(),
                block: ParsedBlock {
                    line_label,
                    ..default_block
                },
            }));
        }
        // If the line is a single boolean/reporter input with a nested block, unwrap it directly.
        // This handles standalone lines like '<mouse down?>' or '(x position)'.
        if children.len() == 1 {
            if let Child::Input(ref input) = children[0] {
                if let Some(ref nested) = input.nested {
                    let mut block = *nested.clone();
                    block.line_label = line_label;
                    let shape = block.shape.clone();
                    return Ok(Some(LineBlock { shape, block }));
                }
            }
        }
        
        // Check for procedure definition or call using language-specific keywords
        let first_label = children.first().and_then(|c| {
            if let Child::Label(v) = c { Some(v.as_str()) } else { None }
        });
        
        if let Some(label) = first_label {
            for lang in &self.languages {
                if label.eq_ignore_ascii_case(&lang.define_spec) {
                    return Ok(Some(LineBlock {
                        shape: "define-hat".to_string(),
                        block: ParsedBlock {
                            id: "procedures_definition".to_string(),
                            selector: String::new(),
                            shape: "define-hat".to_string(),
                            category: "custom".to_string(),
                            language: self.languages[0].code.clone(),
                            children,
                            body: Vec::new(),
                            else_body: Vec::new(),
                            has_else: false,
                            line_number: None,
                            line_label,
                        },
                    }));
                }
                if label.eq_ignore_ascii_case(&lang.call_spec) {
                    return Ok(Some(LineBlock {
                        shape: "stack".to_string(),
                        block: ParsedBlock {
                            id: "procedures_call".to_string(),
                            selector: String::new(),
                            shape: "stack".to_string(),
                            category: "custom".to_string(),
                            language: self.languages[0].code.clone(),
                            children,
                            body: Vec::new(),
                            else_body: Vec::new(),
                            has_else: false,
                            line_number: None,
                            line_label,
                        },
                    }));
                }
            }
        }
        
        let mut block = self.paint_block("stack", children)?;
        block.line_label = line_label;
        let shape = block.shape.clone();
        Ok(Some(LineBlock { shape, block }))
    }

    fn try_category_default(&self, children: &[Child]) -> Result<Option<ParsedBlock>, String> {
        let Some(Child::Icon(name)) = children.first() else {
            return Ok(None);
        };

        let normalized = normalize_category_token(name.as_str());
        let Some(default_id) = data().default_blocks.get(normalized) else {
            return Ok(None);
        };

        // @category + text first tries a real category-constrained parse.
        // If that fails, keep the previous category-forced unrecognized fallback.
        if children.len() > 1 {
            let content_children = children[1..].to_vec();
            if let Some((matched_id, lang_idx)) = self.match_block_candidate(&content_children, Some(normalized)) {
                let block_def = &data().commands_by_id[&matched_id];
                let language = self.languages[lang_idx];
                let spec = language
                    .native_specs
                    .get(&matched_id)
                    .map(|s| s.as_str())
                    .unwrap_or("");
                return Ok(Some(ParsedBlock {
                    id: matched_id,
                    selector: String::new(),
                    shape: block_def.shape.clone(),
                    category: block_def.category.clone(),
                    language: language.code.clone(),
                    children: canonical_children(spec, false, content_children),
                    body: Vec::new(),
                    else_body: Vec::new(),
                    has_else: false,
                    line_number: None,
                    line_label: None,
                }));
            }

            return Ok(Some(ParsedBlock {
                id: "unrecognized".to_string(),
                selector: String::new(),
                shape: "stack".to_string(),
                category: normalized.to_string(),
                language: self.languages[0].code.clone(),
                children: content_children,
                body: Vec::new(),
                else_body: Vec::new(),
                has_else: false,
                line_number: None,
                line_label: None,
            }));
        }

        let block_def = data()
            .commands_by_id
            .get(default_id)
            .ok_or_else(|| format!("scratchblocks-wasm: unknown default block id '{default_id}' for category '{normalized}'"))?;
        let language = self.languages[0];
        let spec = language
            .native_specs
            .get(default_id)
            .map(|s| s.as_str())
            .ok_or_else(|| format!("scratchblocks-wasm: missing locale spec for default block '{default_id}' in language '{}'", language.code))?;

        Ok(Some(ParsedBlock {
            id: default_id.clone(),
            selector: String::new(),
            shape: block_def.shape.clone(),
            category: block_def.category.clone(),
            language: language.code.clone(),
            children: canonical_children(spec, false, Vec::new()),
            body: Vec::new(),
            else_body: Vec::new(),
            has_else: false,
            line_number: None,
            line_label: None,
        }))
    }

    fn parse_parts(&mut self, end: Option<char>) -> Result<Vec<Child>, String> {
        let mut children = Vec::new();
        let mut label = String::new();
        while let Some(tok) = self.peek() {
            // Infix > operator: only possible when there's already content
            // in the current parse scope (at least one child or non-empty label).
            if tok == '>' && end == Some('>')
                && (!children.is_empty() || !label.trim().is_empty())
                && self.is_infix_greater_than()
            {
                self.flush_label(&mut label, &mut children);
                label.push(tok);
                self.next();
                continue;
            }
            if Some(tok) == end || tok == '\n' {
                break;
            }
            match tok {
                '[' => {
                    self.flush_label(&mut label, &mut children);
                    children.push(Child::Input(self.parse_string()?));
                }
                '(' => {
                    self.flush_label(&mut label, &mut children);
                    children.push(self.parse_reporter()?);
                }
                '<' => {
                    if end == Some('>') && self.is_infix_less_than() {
                        label.push(tok);
                        self.next();
                    } else {
                        self.flush_label(&mut label, &mut children);
                        children.push(self.parse_predicate()?);
                    }
                }
                ' ' | '\t' => {
                    self.next();
                    self.flush_label(&mut label, &mut children);
                }
                '@' => {
                    self.flush_label(&mut label, &mut children);
                    children.push(Child::Icon(self.parse_icon_name()));
                }
                _ => {
                    label.push(tok);
                    self.next();
                }
            }
        }
        self.flush_label(&mut label, &mut children);
        Ok(children)
    }

    fn parse_string(&mut self) -> Result<InputNode, String> {
        self.expect('[')?;
        let mut value = String::new();
        while let Some(tok) = self.peek() {
            if tok == ']' || tok == '\n' {
                break;
            }
            if tok == '\\' {
                self.next();
                if let Some(next) = self.peek() {
                    value.push(next);
                    self.next();
                }
            } else {
                value.push(tok);
                self.next();
            }
        }
        if self.peek() == Some(']') {
            self.next();
        }
        let mut is_dropdown = false;
        if value.ends_with(" v") {
            value.truncate(value.len() - 2);
            is_dropdown = true;
        }
        let shape = if is_hex_color(&value) {
            "color"
        } else if is_dropdown {
            "dropdown"
        } else {
            "string"
        };
        Ok(InputNode { shape: shape.to_string(), value, nested: None })
    }

    fn parse_reporter(&mut self) -> Result<Child, String> {
        self.expect('(')?;
        let inner = self.parse_parts(Some(')'))?;
        if self.peek() == Some(')') {
            self.next();
        }
        if inner.is_empty() {
            return Ok(Child::Input(InputNode { shape: "number".to_string(), value: String::new(), nested: None }));
        }

        let mut all_labels = true;
        let mut text = String::new();
        for child in &inner {
            if let Child::Label(value) = child {
                if !text.is_empty() {
                    text.push(' ');
                }
                text.push_str(value);
            } else {
                all_labels = false;
                break;
            }
        }

        if all_labels {
            if text.ends_with(" v") {
                let mut val = text.clone();
                val.truncate(val.len() - 2);
                return Ok(Child::Input(InputNode { shape: "number-dropdown".to_string(), value: val, nested: None }));
            }
            if inner.len() == 1 {
                if let Child::Label(value) = &inner[0] {
                    if is_number_literal(value) {
                        return Ok(Child::Input(InputNode { shape: "number".to_string(), value: value.clone(), nested: None }));
                    }
                }
            }
        }
        
        // Custom-arg reporter: (var [name]) pattern from SB3 import
        // Construct a proper ParsedBlock with custom-arg category so it renders correctly.
        if inner.len() == 2 {
            if let (Child::Label(label_first), Child::Input(input)) = (&inner[0], &inner[1]) {
                if label_first == "var" && input.nested.is_none() {
                    let child_block = ParsedBlock {
                        id: "argument_reporter_string_number".to_string(),
                        selector: String::new(),
                        shape: "reporter".to_string(),
                        category: "custom-arg".to_string(),
                        language: self.languages[0].code.clone(),
                        children: vec![
                            Child::Label(input.value.clone()),
                        ],
                        body: Vec::new(),
                        else_body: Vec::new(),
                        has_else: false,
                        line_number: None,
                        line_label: None,
                    };
                    return Ok(Child::Input(InputNode {
                        shape: "reporter".to_string(),
                        value: String::new(),
                        nested: Some(Box::new(child_block)),
                    }));
                }
            }
        }

        // Try to paint as a reporter/ring block (e.g. '(x position)', '(() + ())')
        let nested_block = self.paint_block("reporter", inner)?;
        Ok(Child::Input(InputNode { shape: "reporter".to_string(), value: String::new(), nested: Some(Box::new(nested_block)) }))
    }

    fn parse_predicate(&mut self) -> Result<Child, String> {
        self.expect('<')?;
        let inner = self.parse_parts(Some('>'))?;
        if self.peek() == Some('>') {
            self.next();
        }
        if inner.is_empty() {
            return Ok(Child::Input(InputNode { shape: "boolean".to_string(), value: String::new(), nested: None }));
        }
        let nested_block = self.paint_block("boolean", inner)?;
        Ok(Child::Input(InputNode { shape: "boolean".to_string(), value: String::new(), nested: Some(Box::new(nested_block)) }))
    }

    fn paint_block(&self, fallback_shape: &str, children: Vec<Child>) -> Result<ParsedBlock, String> {
        if let Some((matched_id, lang_idx)) = self.match_block_candidate(&children, None) {
            let block_def = &data().commands_by_id[&matched_id];
            let language = self.languages[lang_idx];
            let spec = language.native_specs.get(&matched_id).map(|s| s.as_str()).unwrap_or("");
            return Ok(ParsedBlock {
                id: matched_id,
                selector: String::new(),
                shape: block_def.shape.clone(),
                category: block_def.category.clone(),
                language: language.code.clone(),
                children: canonical_children(
                    spec,
                    false,  // has_loop_arrow (not used by canonical_children)
                    children,
                ),
                body: Vec::new(),
                else_body: Vec::new(),
                has_else: false,
                line_number: None,
                line_label: None,
            });
        }
        Ok(ParsedBlock {
            id: "unrecognized".to_string(),
            selector: String::new(),
            shape: fallback_shape.to_string(),
            category: if fallback_shape == "reporter" { "variables".to_string() } else { "obsolete".to_string() },
            language: self.languages[0].code.clone(),
            children,
            body: Vec::new(),
            else_body: Vec::new(),
            has_else: false,
            line_number: None,
            line_label: None,
        })
    }

    fn flush_label(&self, label: &mut String, children: &mut Vec<Child>) {
        let trimmed = label.trim();
        if !trimmed.is_empty() {
            children.push(Child::Label(trimmed.to_string()));
        }
        label.clear();
    }

    fn parse_icon_name(&mut self) -> String {
        self.next();
        let mut name = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphabetic() {
                name.push(ch);
                self.next();
            } else {
                break;
            }
        }
        name
    }

    fn expect(&mut self, expected: char) -> Result<(), String> {
        match self.next() {
            Some(actual) if actual == expected => Ok(()),
            Some(actual) => Err(format!("scratchblocks-wasm: expected '{expected}', found '{actual}'")),
            None => Err(format!("scratchblocks-wasm: expected '{expected}', found end of input")),
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.index).copied()
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.index += 1;
        Some(ch)
    }

    fn is_infix_greater_than(&self) -> bool {
        // Inside a predicate: '>' followed by whitespace + an input-like
        // token (reporter '(', string '[', or predicate '<') is infix.
        // If followed by a label character (text), it's the predicate close.
        if !self.chars.get(self.index + 1).is_some_and(|c| c.is_whitespace()) {
            return false;
        }
        match self.chars.get(self.index + 2..) {
            Some(rest) => {
                let first_non_ws = rest.iter().skip_while(|c| c.is_whitespace()).next();
                matches!(first_non_ws, Some('(' | '[' | '<' | '"' | '\''))
            }
            None => false,
        }
    }

    fn is_infix_less_than(&self) -> bool {
        // Need a whitespace character right after '<'
        if !self.chars.get(self.index + 1).is_some_and(|c| c.is_whitespace()) {
            return false;
        }
        // Check what the first non-whitespace char after that is.
        // If it's '>', this '<' starts a predicate like '< >', not infix.
        match self.chars.get(self.index + 2..) {
            Some(rest) => {
                let first_non_ws = rest.iter().skip_while(|c| c.is_whitespace()).next();
                match first_non_ws {
                    Some('>') => false,   // '< >' → predicate start
                    Some(_) => true,      // '< x...' → infix operator
                    None => false,        // only whitespace remains → predicate
                }
            }
            None => false,                // nothing after whitespace → predicate
        }
    }
}

#[derive(Debug, Clone)]
struct LineBlock {
    shape: String,
    block: ParsedBlock,
}


fn tokenize_spec(spec: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = spec.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == ' ' {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
        } else if ch == '%' {
            let is_placeholder = chars.peek().map_or(false, |c| c.is_ascii_alphanumeric());
            if is_placeholder {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                current.push('%');
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_alphanumeric() || next == '.' {
                        current.push(next);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(current.clone());
                current.clear();
            } else {
                current.push(ch);
            }
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

fn canonical_children(spec: &str, _has_loop_arrow: bool, parsed_children: Vec<Child>) -> Vec<Child> {
    let mut parsed_inputs = parsed_children.into_iter().filter_map(|child| match child {
        Child::Input(input) => Some(input),
        _ => None,
    });
    let mut children = Vec::new();

    for token in tokenize_spec(spec).iter() {
        if token.starts_with('%') && token.len() > 1 {
            let mut input = parsed_inputs.next().unwrap_or_else(|| InputNode {
                shape: placeholder_shape(token).to_string(),
                value: String::new(),
                nested: None,
            });
            // Convert inputs to the shape required by the spec token.
            // e.g. (1 v) is parsed as "number" but %d.listItem requires "dropdown".
            // If the parsed input is already a dropdown, preserve it (e.g. %1 should accept dropdowns).
            // Only override when the spec explicitly demands a different shape.
            if input.nested.is_none() && input.shape != "boolean" && input.shape != "dropdown" && input.shape != "number-dropdown" {
                let spec_shape = placeholder_shape(token);
                if input.shape == "string"
                    || spec_shape == "dropdown"
                    || spec_shape == "number-dropdown"
                {
                    input.shape = spec_shape.to_string();
                }
            }
            children.push(Child::Input(input));
        } else if token.starts_with('@') {
            children.push(Child::Icon(token.trim_start_matches('@').to_string()));
        } else {
            children.push(Child::Label(token.to_string()));
        }
    }
    children
}

fn children_hash(children: &[Child]) -> String {
    let mut words = Vec::new();
    for child in children {
        match child {
            Child::Label(value) => words.push(value.clone()),
            Child::Icon(name) => words.push(format!("@{name}")),
            Child::Input(input) => {
                if input.shape == "dropdown" && input.value.contains('⚑') {
                    words.push("@greenFlag".to_string())
                } else {
                    words.push("_".to_string())
                }
            }
        }
    }
    words.join(" ")
}

fn children_typed_hash(children: &[Child]) -> String {
    let mut words = Vec::new();
    for child in children {
        match child {
            Child::Label(value) => words.push(value.clone()),
            Child::Icon(name) => words.push(format!("@{name}")),
            Child::Input(input) => {
                if input.shape == "dropdown" && input.value.contains('⚑') {
                    words.push("@greenFlag".to_string())
                } else {
                    let kind = input_shape_code(&input.shape);
                    words.push(kind)
                }
            }
        }
    }
    words.join(" ")
}

fn input_shape_code(shape: &str) -> String {
    format!("_{}_", match shape {
        "dropdown" | "number-dropdown" => "d",
        "number" => "n",
        "boolean" => "b",
        "string" => "s",
        "color" => "c",
        _ => "x",
    })
}

fn spec_placeholder_code(placeholder: &str) -> &'static str {
    if placeholder.starts_with("%b") { "_b_" }
    else if placeholder.starts_with("%n") { "_n_" }
    else if placeholder.starts_with("%c") { "_c_" }
    else if placeholder.starts_with("%m") || placeholder.starts_with("%d") { "_d_" }
    else { "_s_" }
}

fn build_typed_hash(spec: &str, inputs: &[String]) -> String {
    let mut out = String::new();
    let tokens = tokenize_spec(spec);
    let mut input_idx = 0;
    for token in tokens {
        if token.starts_with('%') && token.len() > 1 {
            let code = if input_idx < inputs.len() {
                spec_placeholder_code(&inputs[input_idx])
            } else {
                "_x_"
            };
            input_idx += 1;
            out.push_str(code);
            out.push(' ');
        } else if !matches!(token.as_str(), "," | "?" | ":" | ".") {
            out.push_str(&token);
            out.push(' ');
        }
    }
    minify_hash(&out)
}

fn hash_spec(spec: &str) -> String {
    let mut out = String::new();
    let tokens = tokenize_spec(spec);
    for token in tokens {
        if token.starts_with('%') && token.len() > 1 {
            out.push_str(" _ ");
        } else if !matches!(token.as_str(), "," | "?" | ":" | ".") {
            out.push_str(&token);
            out.push(' ');
        }
    }
    minify_hash(&out)
}

fn minify_hash(value: &str) -> String {
    // Replace hyphens only between word characters (e.g. 'mouse-pointer' -> 'mouse pointer')
    // Standalone '-' operators (e.g. '_ - _') must be preserved for OPERATORS_SUBTRACT matching
    let mut result = value.replace('_', " _ ");
    let bytes = result.as_bytes().to_vec();
    let mut out = String::with_capacity(result.len());
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'-' {
            let prev_word = i > 0 && (bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
            let next_word = i + 1 < bytes.len() && (bytes[i + 1].is_ascii_alphanumeric() || bytes[i + 1] == b'_');
            if prev_word && next_word {
                out.push(' ');
            } else {
                out.push('-');
            }
        } else {
            out.push(b as char);
        }
    }
    result = out;
    result
        .replace([',', '%', '?', ':'], "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace('ß', "ss")
        .replace('ä', "a")
        .replace('ö', "o")
        .replace('ü', "u")
        .replace(". . .", "...")
        .replace('…', "...")
        .trim()
        .to_lowercase()
}

fn extract_icon(spec: &str) -> Option<&str> {
    spec.split_whitespace().find(|part| part.starts_with('@'))
}

fn placeholder_shape(token: &str) -> &'static str {
    if token.starts_with("%b") {
        "boolean"
    } else if token.starts_with("%n") {
        "number"
    } else if token.starts_with("%c") {
        "color"
    } else if token.starts_with("%m") || token.starts_with("%d") {
        "dropdown"
    } else {
        "string"
    }
}

fn unicode_icon(icon: &str) -> &'static str {
    match icon {
        "@greenFlag" => "⚑",
        "@turnRight" => "↻",
        "@turnLeft" => "↺",
        "@addInput" => "▸",
        "@delInput" => "◂",
        _ => "",
    }
}

/// Reverse map: if a spec contains a unicode icon, return a version with @iconName.
/// This ensures user input with @ notation also matches specs with unicode icons.
fn unicode_to_at_spec(spec: &str) -> Option<String> {
    let unicode_to_at: &[(&str, &str)] = &[
        ("⚑", "@greenFlag"),
        ("↻", "@turnRight"),
        ("↺", "@turnLeft"),
        ("▸", "@addInput"),
        ("◂", "@delInput"),
    ];
    for (uni, at) in unicode_to_at {
        if spec.contains(uni) {
            return Some(spec.replace(uni, at));
        }
    }
    None
}

fn is_number_literal(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|ch| ch.is_ascii_digit() || matches!(ch, 'e' | 'E' | '.' | '-'))
}

fn is_hex_color(value: &str) -> bool {
    let bytes = value.as_bytes();
    if !(bytes.len() == 4 || bytes.len() == 7) || bytes.first() != Some(&b'#') {
        return false;
    }
    bytes[1..].iter().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sonst_shape() {
        // Parse a single line "sonst" in German
        let result = parse_internal("sonst", "de", false).expect("parse failed");
        // Should produce one script with one block
        assert_eq!(result.len(), 1);
        let script = &result[0];
        assert_eq!(script.len(), 1);
        let block = &script[0];
        assert_eq!(block.shape, "celse");
        assert_eq!(block.id, "CONTROL_ELSE");
    }

    #[test]
    fn test_if_else_parsing() {
        let code = r#"falls <> dann
  sage [Hallo]
sonst
  sage [Tschüss]
ende"#;
        let result = parse_internal(code, "de", false).expect("parse failed");
        assert_eq!(result.len(), 1);
        let script = &result[0];
        // Should have if block with else_body
        let if_block = &script[0];
        assert_eq!(if_block.id, "CONTROL_IF");
        assert!(!if_block.else_body.is_empty(), "else_body should not be empty");
        let else_block = &if_block.else_body[0];
        assert_eq!(else_block.id, "LOOKS_SAY");
    }

    #[test]
    fn test_sound_effect_pitch_prefers_sound_category() {
        let result = parse_internal("change [pitch v] effect by (10)", "en", false).expect("parse failed");
        let block = &result[0][0];
        assert_eq!(block.id, "SOUND_CHANGEEFFECTBY");
        assert_eq!(block.category, "sound");
    }

    #[test]
    fn test_looks_effect_color_prefers_looks_category() {
        let result = parse_internal("change [color v] effect by (10)", "en", false).expect("parse failed");
        let block = &result[0][0];
        assert_eq!(block.id, "LOOKS_CHANGEEFFECTBY");
        assert_eq!(block.category, "looks");
    }

    #[test]
    fn test_category_default_motion() {
        let result = parse_internal("@motion", "en", false).expect("parse failed");
        let block = &result[0][0];
        assert_eq!(block.id, "MOTION_MOVESTEPS");
        assert_eq!(block.category, "motion");
    }

    #[test]
    fn test_category_prefix_with_free_text_forces_category() {
        let result = parse_internal("@motion you are great", "en", false).expect("parse failed");
        let block = &result[0][0];
        assert_eq!(block.id, "unrecognized");
        assert_eq!(block.category, "motion");
        assert_eq!(block.shape, "stack");
        assert!(matches!(&block.children[0], Child::Label(v) if v == "you"));
    }

    #[test]
    fn test_category_prefix_with_free_text_and_label_suffix() {
        let result = parse_internal("@motion you are great #m1", "en", false).expect("parse failed");
        let block = &result[0][0];
        assert_eq!(block.category, "motion");
        assert_eq!(block.line_label.as_deref(), Some("m1"));
    }

    #[test]
    fn test_category_prefix_matches_list_block_when_text_fits() {
        let result = parse_internal("@list add (12) to [my list v]", "en", false).expect("parse failed");
        let block = &result[0][0];
        assert_eq!(block.id, "DATA_ADDTOLIST");
        assert_eq!(block.category, "list");
    }

    #[test]
    fn test_category_prefix_matches_variable_block_when_text_fits() {
        let result = parse_internal("@variable change [score v] by (1)", "en", false).expect("parse failed");
        let block = &result[0][0];
        assert_eq!(block.id, "DATA_CHANGEVARIABLEBY");
        assert_eq!(block.category, "variables");
    }

    #[test]
    fn test_line_label_suffix_and_line_numbers() {
        let result = parse_internal("repeat (2) #loop\nmove (10) steps #step\nend", "en", false).expect("parse failed");
        let block = &result[0][0];
        assert_eq!(block.id, "CONTROL_REPEAT");
        assert_eq!(block.line_label.as_deref(), Some("loop"));
        assert_eq!(block.line_number, Some(1));
        assert_eq!(block.body[0].line_label.as_deref(), Some("step"));
        assert_eq!(block.body[0].line_number, Some(2));
    }

    #[test]
    fn test_duplicate_line_label_fails() {
        let err = parse_internal("move (10) steps #dup\nturn cw (15) degrees #dup", "en", false)
            .expect_err("duplicate label should fail");
        assert!(err.contains("duplicate line label"));
    }
}
