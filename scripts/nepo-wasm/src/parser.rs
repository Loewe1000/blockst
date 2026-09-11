//! Text -> NEPO blocks.
//!
//! The surface syntax is indentation-based, as in the prototype brief:
//!
//! ```text
//! Start
//!   Zeige Text "Hallo"
//!   Wiederhole unendlich oft
//!     Schalte RGB LED an (#ff0000)
//!   Ende
//! ```
//!
//! A line is matched against the localised pattern of every block the selected
//! platform offers. `Ende` is optional — the indentation already says where a
//! mouth closes — but accepted, because teaching material tends to write it.

use crate::catalog::{self, Binding, Language, Pattern, Token};
use crate::model::{BlockSpec, DocumentSpec, ScriptSpec};
use crate::render::render_document;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub code: String,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_platform")]
    pub platform: String,
    pub theme: Option<String>,
    pub scale: Option<f32>,
    #[serde(default)]
    pub font: String,
    #[serde(default)]
    pub widths: Option<HashMap<String, f32>>,
}

fn default_language() -> String {
    "de".to_string()
}

fn default_platform() -> String {
    "calliope".to_string()
}

struct Line {
    indent: usize,
    text: String,
    number: usize,
}

fn scan(code: &str) -> Vec<Line> {
    let mut lines = Vec::new();
    for (index, raw) in code.lines().enumerate() {
        let without_comment = match raw.find("//") {
            Some(at) => &raw[..at],
            None => raw,
        };
        if without_comment.trim().is_empty() {
            continue;
        }
        // A tab counts as two columns, which is enough to keep relative depth
        // consistent for any file that does not mix tabs and spaces mid-line.
        let indent = without_comment
            .chars()
            .take_while(|ch| ch.is_whitespace())
            .map(|ch| if ch == '\t' { 2 } else { 1 })
            .sum();
        lines.push(Line {
            indent,
            text: without_comment.trim().to_string(),
            number: index + 1,
        });
    }
    lines
}

fn is_closer(text: &str) -> bool {
    matches!(
        text.trim().to_lowercase().as_str(),
        "ende" | "end" | "}" | "einde" | "fin"
    )
}

pub fn parse(
    code: &str,
    language_code: &str,
    platform: &str,
) -> Result<Vec<Vec<BlockSpec>>, String> {
    let catalog = catalog::catalog();
    let language = catalog
        .languages
        .get(language_code)
        .ok_or_else(|| format!("nepo: unknown language '{language_code}'"))?;

    let lines = scan(code);
    let mut index = 0usize;
    let blocks = parse_stack(&lines, &mut index, 0, language, platform)?;

    if index < lines.len() {
        return Err(format!(
            "nepo: line {}: '{}' is indented less than the block it follows.",
            lines[index].number, lines[index].text
        ));
    }

    // Every top-level stack is one script. A Start block always begins a new
    // one; anything before it is a loose stack, which teaching material uses
    // to show a single block on its own.
    let mut scripts: Vec<Vec<BlockSpec>> = Vec::new();
    for block in blocks {
        let starts_script = block.shape == "start" || scripts.is_empty();
        if starts_script {
            scripts.push(vec![block]);
        } else {
            scripts.last_mut().expect("script").push(block);
        }
    }

    crate::variables::apply_types(&mut scripts);

    Ok(scripts)
}

fn parse_stack(
    lines: &[Line],
    index: &mut usize,
    indent: usize,
    language: &Language,
    platform: &str,
) -> Result<Vec<BlockSpec>, String> {
    let mut blocks = Vec::new();

    while *index < lines.len() {
        let line = &lines[*index];
        if line.indent < indent {
            break;
        }
        if is_closer(&line.text) {
            *index += 1;
            if line.indent < indent {
                break;
            }
            continue;
        }

        *index += 1;
        let (mut id, mut bindings) = match_line(&line.text, language, platform)
            .ok_or_else(|| unknown_line_error(line, language, platform))?;

        // Anything indented further belongs in this block's mouth.
        let child_indent = indented_after(lines, *index, line.indent);
        let mut mouths = statement_fields_of(&id, language);
        let mut continuation = Vec::new();
        let mut filled = 0usize;

        if let Some(child_indent) = child_indent {
            let mut nested = parse_stack(lines, index, child_indent, language, platform)?;
            match mouths.first() {
                // A block with a mouth swallows what is indented under it —
                // but a mouth with a named connection only accepts the blocks
                // whose notch carries that name. The Start block's `ST` is the
                // one that does: it takes the leading variable declarations
                // and lets the program itself follow below, which is exactly
                // how Open Roberta draws it.
                Some((name, _)) => {
                    if let Some(accepted) = mouth_check(&id, name) {
                        let taken = nested
                            .iter()
                            .position(|block| !connects_as(&block.id, &accepted))
                            .unwrap_or(nested.len());
                        continuation = nested.split_off(taken);
                    }
                    bindings.insert(name.clone(), Binding::Statement(nested));
                    filled = 1;
                }
                // Everything else reads the indent as "these follow me", which
                // is how a bare stack is written in teaching material.
                None => continuation = nested,
            }
        } else if !mouths.is_empty() {
            filled = 1;
        }

        // `robControls_if` and `robControls_ifElse` share the same header
        // (`wenn %IF0`). The otherwise optional `sonst` line disambiguates
        // them only after the first mouth has been read. Upgrade the base if
        // block at that point, retaining its already-parsed DO0 body.
        if id == "robControls_if" {
            let if_else_mouths = statement_fields_of("robControls_ifElse", language);
            let has_else = if_else_mouths
                .get(1)
                .and_then(|(_, label)| label.as_deref())
                .and_then(|label| lines.get(*index).map(|next| (label, next)))
                .map(|(label, next)| {
                    next.indent == line.indent && next.text.eq_ignore_ascii_case(label)
                })
                .unwrap_or(false);
            if has_else {
                id = "robControls_ifElse".to_string();
                mouths = if_else_mouths;
            }
        }

        // A second mouth is introduced by its own label, at the header's
        // indent — "sonst" for robControls_ifElse. The keyword is not
        // hard-coded: it is the literal that opens that row in the locale, so
        // a translation gets it for free.
        while filled < mouths.len() {
            let (name, label) = &mouths[filled];
            let Some(label) = label.as_deref() else { break };
            let Some(next) = lines.get(*index) else { break };
            if next.indent != line.indent || !next.text.eq_ignore_ascii_case(label) {
                break;
            }
            *index += 1;
            let body = match indented_after(lines, *index, line.indent) {
                Some(inner) => parse_stack(lines, index, inner, language, platform)?,
                None => Vec::new(),
            };
            bindings.insert(name.clone(), Binding::Statement(body));
            filled += 1;
        }

        blocks.push(catalog::build_block(&id, language, &bindings)?);
        blocks.extend(continuation);
    }

    Ok(blocks)
}

/// The connection name a mouth insists on, if it has one.
fn mouth_check(id: &str, field: &str) -> Option<String> {
    catalog::catalog()
        .blocks
        .get(id)?
        .fields
        .get(field)?
        .check
        .first()
        .cloned()
}

/// Does this block's notch carry that name? `setPreviousStatement(true, name)`
/// in Blockly, `connection = "..."` in the catalog.
fn connects_as(id: &str, connection: &str) -> bool {
    catalog::catalog()
        .blocks
        .get(id)
        .and_then(|def| def.connection.as_deref())
        .map(|name| name == connection)
        .unwrap_or(false)
}

fn unknown_line_error(line: &Line, language: &Language, platform: &str) -> String {
    let mut known: Vec<&str> = language
        .specs
        .keys()
        .filter(|id| catalog::supports(id, platform))
        .map(|id| id.as_str())
        .collect();
    known.sort_unstable();
    format!(
        "nepo: line {}: no block on platform '{}' matches '{}'. Known blocks: {}.",
        line.number,
        platform,
        line.text,
        known.join(", ")
    )
}

fn indented_after(lines: &[Line], index: usize, indent: usize) -> Option<usize> {
    lines
        .get(index)
        .filter(|next| next.indent > indent)
        .map(|next| next.indent)
}

/// Every mouth the block has, in reading order, each with the label that opens
/// its row. The label is what a source file writes to start that mouth.
fn statement_fields_of(id: &str, language: &Language) -> Vec<(String, Option<String>)> {
    let Some(def) = catalog::catalog().blocks.get(id) else {
        return Vec::new();
    };
    let Some(pattern) = language.specs.get(id) else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for row in &pattern.rows {
        let label = row.iter().find_map(|token| match token {
            Token::Literal(text) => Some(text.clone()),
            _ => None,
        });
        for token in row {
            if let Token::Field(name) = token {
                let is_statement = def
                    .fields
                    .get(name)
                    .map(|field| field.kind == "statement")
                    .unwrap_or(false);
                if is_statement {
                    out.push((name.clone(), label.clone()));
                }
            }
        }
    }
    out
}

/// Tokens a source line is expected to supply: everything except the rows that
/// only exist to label a mouth.
fn parse_tokens<'a>(pattern: &'a Pattern, id: &str) -> Vec<&'a Token> {
    let def = catalog::catalog().blocks.get(id);
    pattern
        .rows
        .iter()
        .filter(|row| {
            !row.iter().any(|token| match token {
                Token::Field(name) => def
                    .and_then(|def| def.fields.get(name))
                    .map(|field| field.kind == "statement")
                    .unwrap_or(false),
                _ => false,
            })
        })
        .flatten()
        .collect()
}

fn candidates<'a>(language: &'a Language, platform: &str) -> Vec<(&'a str, &'a Pattern, bool)> {
    let mut out: Vec<(&str, &Pattern, bool)> = Vec::new();
    for (id, pattern) in &language.specs {
        if catalog::supports(id, platform) {
            out.push((id.as_str(), pattern, true));
        }
    }
    for (id, pattern) in &language.aliases {
        if catalog::supports(id, platform) {
            out.push((id.as_str(), pattern, false));
        }
    }
    out
}

/// How much a matching pattern is preferred, most significant first.
///
/// Everything here is a tie-break, and every one of them has to be decided:
/// the candidates are iterated out of a hash map, so a rule left to chance
/// would make the same document render differently from run to run.
type Rank<'a> = (
    // A bare variable getter is a catch-all for any identifier. Every
    // concrete Blockly block wins against it, so `wahr`, `Herz` and aliases
    // such as `Programmstart` keep their own meaning.
    bool,
    // Open Roberta's own wording beats an alias.
    bool,
    // The pattern that explains more of the line.
    usize,
    // The simpler block: `wenn` on its own is `robControls_if`, and becomes
    // `robControls_ifElse` only when a `sonst` line turns up. Both patterns
    // match the header, so without this the else-mouth would appear at
    // random.
    std::cmp::Reverse<usize>,
    // Nothing is left to hash order.
    std::cmp::Reverse<&'a str>,
);

/// Match a whole line. Ties are broken towards the canonical wording.
fn match_line(
    text: &str,
    language: &Language,
    platform: &str,
) -> Option<(String, HashMap<String, Binding>)> {
    let mut best: Option<(Rank, String, HashMap<String, Binding>)> = None;

    for (id, pattern, canonical) in candidates(language, platform) {
        let tokens = parse_tokens(pattern, id);
        let mut bindings = HashMap::new();
        if let Some(consumed) =
            match_tokens(&tokens, text, 0, id, language, platform, &mut bindings)
        {
            if text[consumed..].trim().is_empty() {
                let rank: Rank<'_> = (
                    id != "variables_get",
                    canonical,
                    tokens.len(),
                    std::cmp::Reverse(pattern.rows.len()),
                    std::cmp::Reverse(id),
                );
                if best.as_ref().map(|(best, _, _)| rank > *best).unwrap_or(true) {
                    best = Some((rank, id.to_string(), bindings));
                }
            }
        }
    }

    best.map(|(_, id, bindings)| (id, bindings))
}

fn skip_space(text: &str, mut pos: usize) -> usize {
    while let Some(ch) = text[pos..].chars().next() {
        if ch.is_whitespace() {
            pos += ch.len_utf8();
        } else {
            break;
        }
    }
    pos
}

fn match_tokens(
    tokens: &[&Token],
    text: &str,
    start: usize,
    id: &str,
    language: &Language,
    platform: &str,
    bindings: &mut HashMap<String, Binding>,
) -> Option<usize> {
    let def = catalog::catalog().blocks.get(id)?;
    let mut pos = start;

    for token in tokens {
        pos = skip_space(text, pos);
        match token {
            Token::Literal(literal) => {
                pos = match_literal(text, pos, literal)?;
            }
            Token::Field(name) => {
                let field = def.fields.get(name)?;
                // A spacer is pure layout: Open Roberta paints it, nobody
                // types it.
                if matches!(field.kind.as_str(), "spacer" | "icon") {
                    continue;
                }
                let matched = match field.kind.as_str() {
                    "dropdown" | "mode" | "image" => match_dropdown(text, pos, id, name, language),
                    "variable" | "variable_name" => match_variable(text, pos),
                    "text" => match_text(text, pos),
                    "number" => match_number(text, pos),
                    "colour" => match_colour(text, pos),
                    "matrix" => match_matrix(text, pos),
                    "value" | "inline_value" => {
                        match_value(text, pos, language, platform, inline_budget(id, language))
                    }
                    _ => return None,
                };
                match matched {
                    Some((binding, consumed)) => {
                        bindings.insert(name.clone(), binding);
                        pos = consumed;
                    }
                    // An empty socket is a legitimate thing to write: it is how
                    // a worksheet shows which type belongs in a hole. Every
                    // other field kind has to match.
                    None if matches!(field.kind.as_str(), "value" | "inline_value") => {}
                    None => return None,
                }
            }
        }
    }

    Some(pos)
}

/// The expression-precedence ladder.
///
/// A block whose pattern *starts* with an inline socket is left-recursive: on
/// its own it would happily parse itself as its own first operand, forever.
/// Blockly never has this problem, because there the nesting is done by hand
/// with the mouse. A text syntax has to say what binds tighter than what, and
/// the levels below are the usual answer:
///
///   0  plain values — a number, a sensor, a variable
///   1  arithmetic:  `Punkte + 5`
///   2  comparison:  `Punkte + 5 > 10`
///   3  logic:       `Punkte > 10 und Taste A gedrückt?`
///
/// A block of level *n* may appear where the budget is at least *n*, and its
/// own operands are parsed with a budget of *n - 1*. Brackets reset the
/// budget to the top, so anything can be written in any position.
const TOP_LEVEL: u8 = 3;

fn precedence_level(id: &str) -> u8 {
    match id {
        "math_arithmetic" => 1,
        "logic_compare" => 2,
        "logic_operation" => 3,
        // Everything else that leads with a socket — "%VALUE ist leer?" —
        // binds as tightly as arithmetic and takes plain values.
        _ => 1,
    }
}

fn inline_budget(id: &str, language: &Language) -> u8 {
    if leads_with_inline(id, language) {
        precedence_level(id).saturating_sub(1)
    } else {
        TOP_LEVEL
    }
}

fn leads_with_inline(id: &str, language: &Language) -> bool {
    let Some(def) = catalog::catalog().blocks.get(id) else {
        return false;
    };
    let Some(pattern) = language.specs.get(id) else {
        return false;
    };
    matches!(
        pattern.rows.first().and_then(|row| row.first()),
        Some(Token::Field(name))
            if def
                .fields
                .get(name)
                .map(|field| field.kind == "inline_value")
                .unwrap_or(false)
    )
}

/// Case-insensitive literal match that is safe on multi-byte characters and
/// forgiving about how much whitespace the author typed.
///
/// Comparing `text[pos..literal.len()]` would be neither: it panics in the
/// middle of a "ü", and it insists on exactly one space where the pattern has
/// one.
fn match_literal(text: &str, pos: usize, literal: &str) -> Option<usize> {
    let mut cursor = pos;
    let mut expected = literal.chars().peekable();

    while let Some(want) = expected.next() {
        if want.is_whitespace() {
            let skipped = skip_space(text, cursor);
            if skipped == cursor {
                return None;
            }
            cursor = skipped;
            while expected
                .peek()
                .map(|ch| ch.is_whitespace())
                .unwrap_or(false)
            {
                expected.next();
            }
            continue;
        }
        let got = text[cursor..].chars().next()?;
        if !got.eq_ignore_ascii_case(&want) && got.to_lowercase().ne(want.to_lowercase()) {
            return None;
        }
        cursor += got.len_utf8();
    }

    Some(cursor)
}

fn match_dropdown(
    text: &str,
    pos: usize,
    id: &str,
    field: &str,
    language: &Language,
) -> Option<(Binding, usize)> {
    let mut options: Vec<&String> = language.dropdown_options(id, field).iter().collect();
    // Longest first, so "A+B" wins over "A".
    options.sort_by_key(|option| std::cmp::Reverse(option.chars().count()));
    for option in options {
        if let Some(end) = match_literal(text, pos, option) {
            return Some((Binding::Dropdown((*option).clone()), end));
        }
    }
    None
}

/// Blockly's FieldVariable is a dropdown whose entries are made by the user,
/// not a fixed toolbox list. In source, a variable is an identifier such as
/// `Punkte` or `zaehler_1`. Restricting this to identifiers keeps a variable
/// distinct from the quoted text reporter in an otherwise ambiguous socket.
fn match_variable(text: &str, pos: usize) -> Option<(Binding, usize)> {
    let rest = &text[pos..];
    let mut end = 0usize;
    for (offset, ch) in rest.char_indices() {
        if ch.is_alphanumeric() || ch == '_' {
            end = offset + ch.len_utf8();
        } else {
            break;
        }
    }
    let name = &rest[..end];
    let valid_start = name
        .chars()
        .next()
        .map(|ch| ch.is_alphabetic() || ch == '_')
        .unwrap_or(false);
    valid_start.then_some((Binding::Variable(name.to_string()), pos + end))
}

/// A quoted or bracketed literal: `"Hallo"`, `'Hallo'` or `[Hallo]`.
fn match_text(text: &str, pos: usize) -> Option<(Binding, usize)> {
    let rest = &text[pos..];
    let (open, close) = match rest.chars().next()? {
        '"' => ('"', '"'),
        '\'' => ('\'', '\''),
        '[' => ('[', ']'),
        _ => return None,
    };
    let inner_start = pos + open.len_utf8();
    let end = text[inner_start..].find(close)? + inner_start;
    Some((
        Binding::Text(text[inner_start..end].to_string()),
        end + close.len_utf8(),
    ))
}

/// A bare numeric literal, as `math_number` is written: `500`, `-3`, `0.5`.
fn match_number(text: &str, pos: usize) -> Option<(Binding, usize)> {
    let rest = &text[pos..];
    let mut end = 0;
    let mut seen_digit = false;
    let mut seen_dot = false;

    for (offset, ch) in rest.char_indices() {
        match ch {
            '-' | '+' if offset == 0 => {}
            '.' if !seen_dot && seen_digit => seen_dot = true,
            c if c.is_ascii_digit() => seen_digit = true,
            _ => break,
        }
        end = offset + ch.len_utf8();
    }

    if !seen_digit {
        return None;
    }
    Some((Binding::Number(rest[..end].to_string()), pos + end))
}

fn match_colour(text: &str, pos: usize) -> Option<(Binding, usize)> {
    let rest = &text[pos..];
    if !rest.starts_with('#') {
        return None;
    }
    let digits: String = rest[1..].chars().take(6).collect();
    if digits.len() != 6 || !digits.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return None;
    }
    Some((Binding::Colour(format!("#{digits}")), pos + 7))
}

/// `.#.#.|.#.#.|.....|#...#|.###.` — one string per LED row.
fn match_matrix(text: &str, pos: usize) -> Option<(Binding, usize)> {
    let rest = &text[pos..];
    let end = rest
        .find(|ch: char| !matches!(ch, '.' | '#' | '|' | '/' | ' ') && !ch.is_ascii_digit())
        .unwrap_or(rest.len());
    let body = &rest[..end];
    let rows: Vec<String> = body
        .split(['|', '/'])
        .map(|row| row.trim().to_string())
        .collect();
    if rows.len() != crate::matrix::SIZE {
        return None;
    }
    if rows
        .iter()
        .any(|row| row.chars().count() != crate::matrix::SIZE)
    {
        return None;
    }
    Some((Binding::Matrix(rows), pos + end))
}

/// Parse a reporting block at this position, optionally in parentheses.
fn match_value(
    text: &str,
    pos: usize,
    language: &Language,
    platform: &str,
    inline_budget: u8,
) -> Option<(Binding, usize)> {
    let rest = &text[pos..];

    let mut best: Option<(BlockSpec, usize)> = None;

    // Brackets say where an expression ends, so the ladder has nothing left
    // to disambiguate inside them: anything may be written there.
    if rest.starts_with('(') {
        if let Some(close) = find_closing(rest) {
            let inner = &rest[1..close];
            if let Some((block, consumed)) =
                match_value_body(inner, 0, language, platform, TOP_LEVEL)
            {
                if inner[consumed..].trim().is_empty() {
                    best = Some((block, pos + close + 1));
                }
            }
        }
    }

    // The bracketed group may only have been the first operand — as in
    // "(a oder b) und c" — so the unbracketed reading is tried as well and
    // the longer of the two wins. It terminates because every step down the
    // ladder lowers the budget.
    if let Some((block, consumed)) = match_value_body(text, pos, language, platform, inline_budget)
    {
        if best
            .as_ref()
            .map(|(_, best_consumed)| consumed > *best_consumed)
            .unwrap_or(true)
        {
            best = Some((block, consumed));
        }
    }

    best.map(|(block, consumed)| (Binding::Value(block), consumed))
}

fn find_closing(text: &str) -> Option<usize> {
    let mut depth = 0usize;
    for (offset, ch) in text.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(offset);
                }
            }
            _ => {}
        }
    }
    None
}

fn match_value_body(
    text: &str,
    pos: usize,
    language: &Language,
    platform: &str,
    inline_budget: u8,
) -> Option<(BlockSpec, usize)> {
    let mut best: Option<((usize, bool, std::cmp::Reverse<&str>), BlockSpec)> = None;

    for (id, pattern, _) in candidates(language, platform) {
        let def = catalog::catalog().blocks.get(id)?;
        if def.shape != "value" {
            continue;
        }
        let tokens = parse_tokens(pattern, id);
        // Inline reporter patterns cannot recurse into themselves as their
        // first operand. The budget encodes raw values -> comparisons ->
        // logical operations and removes the ambiguity without flat greed.
        let leading_inline = matches!(tokens.first(), Some(Token::Field(name)) if def.fields.get(name).map(|field| field.kind == "inline_value").unwrap_or(false));
        if leading_inline && precedence_level(id) > inline_budget {
            continue;
        }
        let mut bindings = HashMap::new();
        if let Some(consumed) =
            match_tokens(&tokens, text, pos, id, language, platform, &mut bindings)
        {
            // Longest match first, then the same tie-breaks as a whole line.
            let rank = (consumed, id != "variables_get", std::cmp::Reverse(id));
            if best
                .as_ref()
                .map(|(best, _)| rank > *best)
                .unwrap_or(true)
            {
                if let Ok(block) = catalog::build_block(id, language, &bindings) {
                    best = Some((rank, block));
                }
            }
        }
    }

    best.map(|((consumed, _, _), block)| (block, consumed))
}

pub fn render_request(input: &str) -> Result<String, String> {
    let request: Request = serde_json::from_str(input)
        .map_err(|err| format!("nepo: invalid render request: {err}"))?;
    let scripts = parse(&request.code, &request.language, &request.platform)?;

    let document = DocumentSpec {
        dialect: crate::model::Dialect::Nepo,
        scale: request.scale,
        theme: request.theme,
        font: if request.font.is_empty() {
            "Helvetica Neue, Helvetica, Arial, sans-serif".to_string()
        } else {
            request.font
        },
        scripts: scripts
            .into_iter()
            .map(|blocks| ScriptSpec { blocks })
            .collect(),
    };

    if let Some(widths) = request.widths {
        if !widths.is_empty() {
            crate::measure::set_widths(widths);
            let svg = render_document(&document);
            crate::measure::clear_widths();
            return Ok(svg);
        }
    }
    Ok(render_document(&document))
}

/// Every string the renderer will paint, so the host can measure them first.
pub fn extract_texts(input: &str) -> Result<String, String> {
    let request: Request = serde_json::from_str(input)
        .map_err(|err| format!("nepo: invalid extract request: {err}"))?;
    let scripts = parse(&request.code, &request.language, &request.platform)?;
    let mut texts = std::collections::BTreeSet::new();
    for blocks in &scripts {
        let mut expanded = blocks.clone();
        for block in expanded.iter_mut() {
            crate::matrix::expand(block);
        }
        crate::measure::collect_texts(&expanded, &mut texts);
    }
    let list: Vec<String> = texts.into_iter().collect();
    serde_json::to_string(&list).map_err(|err| format!("nepo: failed to encode texts: {err}"))
}

/// The parsed structure, for tests and for callers that want the AST.
pub fn parse_request(input: &str) -> Result<String, String> {
    let request: Request =
        serde_json::from_str(input).map_err(|err| format!("nepo: invalid parse request: {err}"))?;
    let scripts = parse(&request.code, &request.language, &request.platform)?;
    serde_json::to_string(&crate::debug::describe(&scripts))
        .map_err(|err| format!("nepo: failed to encode parse result: {err}"))
}
