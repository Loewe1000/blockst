// Locale database for sb3-wasm — loads all 26 language templates at init.
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LocaleToml {
    specs: HashMap<String, String>,
}

static mut LOCALE_DB: Option<HashMap<String, HashMap<String, String>>> = None;

/// Initialize locale database from embedded TOML data.
/// Must be called once before any locale lookup.
pub(crate) fn init() {
    let mut db: HashMap<String, HashMap<String, String>> = HashMap::new();
    for &(code, locale_toml) in crate::generated::LOCALE_DATA {
        let locale: LocaleToml = toml::from_str(locale_toml)
            .unwrap_or_else(|e| panic!("locales/{code}.toml: {e}"));
        db.insert(code.to_string(), locale.specs);
    }
    unsafe { LOCALE_DB = Some(db); }
}

/// Get locale template for a block ID in the given language.
/// Returns None if the language or block ID is not found.
pub(crate) fn get_template(block_id: &str, lang: &str) -> Option<String> {
    let db = unsafe { LOCALE_DB.as_ref()? };
    // Try requested language
    if let Some(specs) = db.get(lang) {
        if let Some(tmpl) = specs.get(block_id) {
            return Some(tmpl.clone());
        }
    }
    // Fall back to English
    if lang != "en" {
        if let Some(specs) = db.get("en") {
            return specs.get(block_id).cloned();
        }
    }
    None
}

/// Fill a locale template by replacing %1, %2, ... with the given input strings.
pub(crate) fn fill_template(template: &str, inputs: &[String]) -> String {
    let mut result = template.to_string();
    for (i, input) in inputs.iter().enumerate() {
        let placeholder = format!("%{}", i + 1);
        result = result.replace(&placeholder, input);
    }
    result
}

/// Render a block using the locale template. Falls back to English fallback template.
pub(crate) fn fmt_locale(lang: &str, block_id: &str, inputs: &[String], en_fallback: &str) -> String {
    if let Some(tmpl) = get_template(block_id, lang) {
        fill_template(&tmpl, inputs)
    } else {
        // No locale found — use the English fallback template
        fill_template(en_fallback, inputs)
    }
}

/// Parse a block ID from opcode (e.g. "motion_movesteps" → "MOTION_MOVESTEPS")
pub(crate) fn opcode_to_block_id(opcode: &str) -> String {
    opcode.to_ascii_uppercase()
}
