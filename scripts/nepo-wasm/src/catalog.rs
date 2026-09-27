//! The block catalog: structure from `data/blocks.toml`, text from
//! `data/locales/<lang>.toml`, both embedded at compile time.

use crate::model::{deserialize_checks, BlockSpec, RowSpec, SegmentSpec};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

const BLOCKS_TOML: &str = include_str!("../data/blocks.toml");

const LOCALES: &[(&str, &str)] = &[
    ("de", include_str!("../data/locales/de.toml")),
    ("en", include_str!("../data/locales/en.toml")),
];

#[derive(Debug, Deserialize)]
struct BlocksToml {
    blocks: HashMap<String, BlockDef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlockDef {
    #[serde(default)]
    pub platforms: Vec<String>,
    pub shape: String,
    pub category: String,
    /// Output data types for `value` blocks.
    #[serde(default, deserialize_with = "deserialize_checks")]
    pub check: Vec<String>,
    /// Type this block's output from the id a dropdown selected, rather than
    /// fixing it in the catalog: a list block reports `Array_Number` or
    /// `Array_String` depending on what it was told to hold.
    #[serde(default)]
    pub check_from: Option<String>,
    #[serde(default)]
    pub check_prefix: Option<String>,
    /// The name Blockly's `setPreviousStatement(true, name)` gives this
    /// block's notch. A mouth that names the same one accepts nothing else —
    /// that is how a variable declaration can only sit inside `Start`.
    #[serde(default)]
    pub connection: Option<String>,
    #[serde(default)]
    pub fields: HashMap<String, FieldDef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FieldDef {
    pub kind: String,
    #[serde(default, deserialize_with = "deserialize_checks")]
    pub check: Vec<String>,
    #[serde(default)]
    pub default: Option<String>,
    /// For a dropdown: the ids Blockly pairs with the locale's labels, in the
    /// same order — `[[Blockly.Msg.VARIABLES_TYPE_NUMBER, 'Number'], ...]`.
    /// A label is what the block prints; the id is what other fields reason
    /// about, which is why the two lists are kept side by side.
    #[serde(default)]
    pub values: Vec<String>,
    /// Take this socket's accepted type from the id another field selected.
    /// `robGlobalVariables_declare` does exactly this: changing the type
    /// dropdown re-checks the initial-value socket.
    #[serde(default)]
    pub check_from: Option<String>,
    /// Drop the whole row when nothing is bound to this field. Blockly models
    /// the same thing as a mutation that appends the input only when it is
    /// needed.
    #[serde(default)]
    pub optional: bool,
    /// `setAlign(Blockly.ALIGN_RIGHT)` on the row this field owns.
    #[serde(default)]
    pub align: Option<String>,
    /// For a `label`: the dropdown whose selection picks the text. A sensor's
    /// unit is one — `updateShape_` swaps "%" for "°" when the mode changes —
    /// so the locale lists one label per option of that dropdown, in step.
    /// Without it the label is the locale's first entry.
    #[serde(default)]
    pub label_from: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LocaleToml {
    #[serde(default)]
    dir: Option<String>,
    specs: HashMap<String, String>,
    #[serde(default)]
    dropdowns: HashMap<String, HashMap<String, Vec<String>>>,
    #[serde(default)]
    aliases: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Literal(String),
    Field(String),
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub rows: Vec<Vec<Token>>,
}

impl Pattern {
    fn compile(spec: &str) -> Pattern {
        let rows = spec
            .split('|')
            .map(|row| compile_row(row.trim()))
            .filter(|row| !row.is_empty())
            .collect();
        Pattern { rows }
    }

    /// Names of the placeholders in the pattern, in reading order.
    pub fn fields(&self) -> Vec<&str> {
        self.rows
            .iter()
            .flatten()
            .filter_map(|token| match token {
                Token::Field(name) => Some(name.as_str()),
                _ => None,
            })
            .collect()
    }
}

fn compile_row(row: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut literal = String::new();
    let mut chars = row.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '%' {
            literal.push(ch);
            continue;
        }
        let mut name = String::new();
        while let Some(&next) = chars.peek() {
            if next.is_ascii_alphanumeric() || next == '_' {
                name.push(next);
                chars.next();
            } else {
                break;
            }
        }
        if name.is_empty() {
            literal.push('%');
            continue;
        }
        let trimmed = literal.trim();
        if !trimmed.is_empty() {
            tokens.push(Token::Literal(trimmed.to_string()));
        }
        literal.clear();
        tokens.push(Token::Field(name));
    }

    let trimmed = literal.trim();
    if !trimmed.is_empty() {
        tokens.push(Token::Literal(trimmed.to_string()));
    }
    tokens
}

#[derive(Debug)]
pub struct Language {
    pub code: String,
    pub rtl: bool,
    /// Canonical pattern per block, used for rendering.
    pub specs: HashMap<String, Pattern>,
    /// Extra patterns accepted when parsing, never rendered.
    pub aliases: Vec<(String, Pattern)>,
    /// block id -> field name -> options, first entry is the default.
    pub dropdowns: HashMap<String, HashMap<String, Vec<String>>>,
}

impl Language {
    pub fn dropdown_options(&self, block_id: &str, field: &str) -> &[String] {
        self.dropdowns
            .get(block_id)
            .and_then(|fields| fields.get(field))
            .map(|options| options.as_slice())
            .unwrap_or(&[])
    }
}

#[derive(Debug)]
pub struct Catalog {
    pub blocks: HashMap<String, BlockDef>,
    pub languages: HashMap<String, Language>,
}

static CATALOG: OnceLock<Catalog> = OnceLock::new();

pub fn catalog() -> &'static Catalog {
    CATALOG.get_or_init(|| {
        let blocks: BlocksToml = toml::from_str(BLOCKS_TOML).expect("data/blocks.toml");
        let mut languages = HashMap::new();

        for (code, source) in LOCALES {
            let locale: LocaleToml =
                toml::from_str(source).unwrap_or_else(|err| panic!("locales/{code}.toml: {err}"));

            let specs: HashMap<String, Pattern> = locale
                .specs
                .iter()
                .map(|(id, spec)| (id.clone(), Pattern::compile(spec)))
                .collect();

            let mut aliases = Vec::new();
            for (id, patterns) in &locale.aliases {
                for spec in patterns {
                    aliases.push((id.clone(), Pattern::compile(spec)));
                }
            }

            languages.insert(
                code.to_string(),
                Language {
                    code: code.to_string(),
                    rtl: locale
                        .dir
                        .as_deref()
                        .map(|dir| dir.eq_ignore_ascii_case("rtl"))
                        .unwrap_or(false),
                    specs,
                    aliases,
                    dropdowns: locale.dropdowns,
                },
            );
        }

        Catalog {
            blocks: blocks.blocks,
            languages,
        }
    })
}

/// What a parsed line supplies for one placeholder.
#[derive(Debug, Clone)]
pub enum Binding {
    Dropdown(String),
    Variable(String),
    Text(String),
    Number(String),
    Colour(String),
    Matrix(Vec<String>),
    Value(BlockSpec),
    Statement(Vec<BlockSpec>),
}

/// The label a dropdown-shaped field is showing: what the source selected, or
/// the first option, which is the value a freshly dragged Blockly field has.
fn selected_label(
    id: &str,
    field: &str,
    language: &Language,
    bindings: &HashMap<String, Binding>,
) -> String {
    match bindings.get(field) {
        Some(Binding::Dropdown(text)) => text.clone(),
        _ => default_option(id, field, language),
    }
}

/// What a dropdown shows when the source did not choose: the catalog's
/// `default` if it names one — the toolbox hands out the radio block at full
/// strength, `<field name="POWER">7</field>` — and otherwise the first option,
/// which is what a freshly created Blockly field holds.
fn default_option(id: &str, field: &str, language: &Language) -> String {
    let options = language.dropdown_options(id, field);
    catalog()
        .blocks
        .get(id)
        .and_then(|def| def.fields.get(field))
        .and_then(|def| def.default.as_ref())
        .filter(|wanted| options.contains(wanted))
        .or_else(|| options.first())
        .cloned()
        .unwrap_or_default()
}

/// Select a type dropdown the source left out from the value it types.
///
/// `Sende Nachricht "Hallo"` names no type, but the text block plugged in
/// says `String`, and a socket whose check follows that dropdown would
/// otherwise demand a `Number` it was never given. Only a dropdown that
/// carries type ids (`values`) is inferred, and only from a value that
/// reports exactly one type.
fn infer_selections(
    id: &str,
    def: &BlockDef,
    language: &Language,
    bindings: &HashMap<String, Binding>,
) -> HashMap<String, Binding> {
    let mut inferred = bindings.clone();
    for (name, field) in &def.fields {
        let Some(source) = field.check_from.as_deref() else {
            continue;
        };
        if inferred.contains_key(source) {
            continue;
        }
        let Some(Binding::Value(child)) = bindings.get(name) else {
            continue;
        };
        let [only] = child.check.as_slice() else {
            continue;
        };
        let Some(index) = def
            .fields
            .get(source)
            .and_then(|dropdown| dropdown.values.iter().position(|value| value == only))
        else {
            continue;
        };
        if let Some(label) = language.dropdown_options(id, source).get(index) {
            inferred.insert(source.to_string(), Binding::Dropdown(label.clone()));
        }
    }
    inferred
}

/// The *id* behind that label — `'Number'` behind "Zahl". Blockly stores the
/// pair; the catalog stores the ids and the locale the labels, in the same
/// order, so the lookup is by position.
fn selected_value(
    id: &str,
    field: &str,
    def: &FieldDef,
    language: &Language,
    bindings: &HashMap<String, Binding>,
) -> Option<String> {
    if def.values.is_empty() {
        return None;
    }
    let label = selected_label(id, field, language, bindings);
    let options = language.dropdown_options(id, field);
    let index = options.iter().position(|option| *option == label)?;
    def.values.get(index).cloned()
}

/// Assemble a renderable block from the catalog plus what the parser found.
///
/// Rendering always uses the *canonical* pattern, never the alias that matched,
/// so a document written with a colloquial wording still prints Open Roberta's
/// own words.
pub fn build_block(
    id: &str,
    language: &Language,
    bindings: &HashMap<String, Binding>,
) -> Result<BlockSpec, String> {
    let catalog = catalog();
    let def = catalog
        .blocks
        .get(id)
        .ok_or_else(|| format!("nepo: unknown block '{id}'"))?;
    let pattern = language
        .specs
        .get(id)
        .ok_or_else(|| format!("nepo: block '{id}' has no {} pattern", language.code))?;
    let bindings = &infer_selections(id, def, language, bindings);

    // Every id a dropdown in this block has selected, so that a socket with
    // `check_from` can be typed no matter which row it sits in.
    let selected: HashMap<&str, String> = def
        .fields
        .iter()
        .filter_map(|(name, field)| {
            selected_value(id, name, field, language, bindings)
                .map(|value| (name.as_str(), value))
        })
        .collect();

    let mut rows = Vec::new();

    for tokens in &pattern.rows {
        let mut fields: Vec<SegmentSpec> = Vec::new();
        let mut kind = "dummy".to_string();
        let mut check = Vec::new();
        let mut value = None;
        let mut body = Vec::new();
        let mut drop_row = false;
        let mut align = "left".to_string();

        for token in tokens {
            match token {
                Token::Literal(text) => fields.push(SegmentSpec::Text {
                    value: text.clone(),
                    monospace: false,
                }),
                Token::Field(name) => {
                    let field = def.fields.get(name).ok_or_else(|| {
                        format!("nepo: block '{id}' has no field '{name}' but its pattern uses it")
                    })?;
                    if let Some(wanted) = field.align.as_deref() {
                        align = wanted.to_string();
                    }
                    match field.kind.as_str() {
                        "dropdown" => {
                            let selected = selected_label(id, name, language, bindings);
                            fields.push(SegmentSpec::Dropdown { value: selected });
                        }
                        // A sensor port that comes from the robot configuration.
                        // Blockly fills the menu with the names the
                        // configuration gave its blocks (`getConfigPorts`), so
                        // like a variable its entries belong to the document;
                        // the locale supplies the name the lab gives a freshly
                        // added sensor.
                        "config_port" => {
                            let selected = match bindings.get(name) {
                                Some(Binding::Variable(text)) => text.clone(),
                                _ => default_option(id, name, language),
                            };
                            fields.push(SegmentSpec::Dropdown { value: selected });
                        }
                        // A label whose text follows another dropdown: the
                        // unit of a sensor with several modes. Blockly never
                        // lets a label disappear — an empty one still paints a
                        // no-break space and takes its separator — so the
                        // locale writes " " where a mode has no unit.
                        "label" => {
                            let index = field
                                .label_from
                                .as_deref()
                                .and_then(|source| {
                                    let chosen = selected_label(id, source, language, bindings);
                                    language
                                        .dropdown_options(id, source)
                                        .iter()
                                        .position(|option| *option == chosen)
                                })
                                .unwrap_or(0);
                            let text = language
                                .dropdown_options(id, name)
                                .get(index)
                                .cloned()
                                .unwrap_or_else(|| " ".to_string());
                            fields.push(SegmentSpec::Text {
                                value: text,
                                monospace: false,
                            });
                        }
                        // `FieldVariable` inherits from FieldDropdown in
                        // Blockly, but its entries belong to the document
                        // rather than to a fixed catalogue. The source parser
                        // therefore supplies its current name directly.
                        "variable" => {
                            let selected = match bindings.get(name) {
                                Some(Binding::Variable(text)) => text.clone(),
                                _ => field.default.clone().unwrap_or_default(),
                            };
                            fields.push(SegmentSpec::Dropdown { value: selected });
                        }
                        // The declaration names the variable in a plain
                        // `FieldTextInput`: it is where the name is written,
                        // not where an existing one is picked, so it has no
                        // dropdown arrow.
                        "variable_name" => {
                            let selected = match bindings.get(name) {
                                Some(Binding::Variable(text)) => text.clone(),
                                _ => field.default.clone().unwrap_or_default(),
                            };
                            fields.push(SegmentSpec::Input { value: selected });
                        }
                        // Generic sensors with only one available mode still
                        // use Blockly's editable FieldDropdown, but it has no
                        // arrow. Model it as an input-shaped fixed choice.
                        "mode" => {
                            let selected = match bindings.get(name) {
                                Some(Binding::Dropdown(text)) => text.clone(),
                                _ => language
                                    .dropdown_options(id, name)
                                    .first()
                                    .cloned()
                                    .unwrap_or_default(),
                            };
                            fields.push(SegmentSpec::Input { value: selected });
                        }
                        "text" | "number" => {
                            let text = match bindings.get(name) {
                                Some(Binding::Text(text)) | Some(Binding::Number(text)) => {
                                    text.clone()
                                }
                                _ => field.default.clone().unwrap_or_default(),
                            };
                            fields.push(SegmentSpec::Input { value: text });
                        }
                        // A label used purely as a gap. `robControls_start`
                        // has one: `appendField('  ')` between the title and
                        // the hidden debug field, and it is worth two spaces
                        // plus a separator of block width.
                        "spacer" => {
                            fields.push(SegmentSpec::Text {
                                value: field.default.clone().unwrap_or_else(|| "  ".to_string()),
                                monospace: false,
                            });
                        }
                        "icon" => fields.push(SegmentSpec::Icon {
                            name: field.default.clone().unwrap_or_default(),
                        }),
                        "colour" => {
                            let colour = match bindings.get(name) {
                                Some(Binding::Colour(colour)) => colour.clone(),
                                _ => field
                                    .default
                                    .clone()
                                    .unwrap_or_else(|| "#ff0000".to_string()),
                            };
                            fields.push(SegmentSpec::ColourField { colour });
                        }
                        "matrix" => {
                            let cells = match bindings.get(name) {
                                Some(Binding::Matrix(cells)) => cells.clone(),
                                _ => vec![String::new(); crate::matrix::SIZE],
                            };
                            fields.push(SegmentSpec::PixelMatrix { rows: cells });
                        }
                        "image" => {
                            let selected = match bindings.get(name) {
                                Some(Binding::Dropdown(text)) => text.clone(),
                                _ => language
                                    .dropdown_options(id, name)
                                    .first()
                                    .cloned()
                                    .unwrap_or_default(),
                            };
                            fields.push(SegmentSpec::Image { value: selected });
                        }
                        "inline_value" => {
                            let value = match bindings.get(name) {
                                Some(Binding::Value(block)) => Some(Box::new(block.clone())),
                                _ => None,
                            };
                            fields.push(SegmentSpec::InlineValue {
                                check: resolve_check(field, &selected),
                                value,
                            });
                        }
                        "value" => {
                            kind = "value".to_string();
                            check = resolve_check(field, &selected);
                            if let Some(Binding::Value(block)) = bindings.get(name) {
                                value = Some(Box::new(block.clone()));
                            }
                            // An optional socket that stayed empty is a row
                            // Blockly's mutator never appended — the sixth
                            // item of a three-item list.
                            if field.optional && value.is_none() {
                                drop_row = true;
                            }
                        }
                        "statement" => {
                            kind = "statement".to_string();
                            if let Some(Binding::Statement(blocks)) = bindings.get(name) {
                                body = blocks.clone();
                            }
                            // An optional mouth that stayed empty was never
                            // appended in the first place.
                            if field.optional && body.is_empty() {
                                drop_row = true;
                            }
                        }
                        other => {
                            return Err(format!(
                                "nepo: block '{id}' field '{name}' has unknown kind '{other}'"
                            ))
                        }
                    }
                }
            }
        }

        if drop_row {
            continue;
        }

        rows.push(RowSpec {
            kind,
            align,
            fields,
            check,
            value,
            body,
        });
    }

    Ok(BlockSpec {
        dialect: Some("nepo".to_string()),
        id: id.to_string(),
        shape: def.shape.clone(),
        category: def.category.clone(),
        check: match def.check_from.as_deref().and_then(|from| selected.get(from)) {
            Some(value) => vec![format!(
                "{}{value}",
                def.check_prefix.as_deref().unwrap_or_default()
            )],
            None => def.check.clone(),
        },
        segments: Vec::new(),
        rows,
    })
}

/// A socket's accepted types: the fixed `check` from the catalog, unless the
/// field takes its type from a dropdown, in which case the selection wins.
fn resolve_check(field: &FieldDef, selected: &HashMap<&str, String>) -> Vec<String> {
    match field.check_from.as_deref() {
        Some(source) => match selected.get(source) {
            Some(value) => vec![value.clone()],
            None => field.check.clone(),
        },
        None => field.check.clone(),
    }
}

/// Is this block offered on the selected platform?
pub fn supports(id: &str, platform: &str) -> bool {
    catalog()
        .blocks
        .get(id)
        .map(|def| def.platforms.is_empty() || def.platforms.iter().any(|p| p == platform))
        .unwrap_or(false)
}
