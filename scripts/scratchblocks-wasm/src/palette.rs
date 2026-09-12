//! Category colours, as data rather than as a match arm per category.
//!
//! A block language decides two things about colour: which categories exist,
//! and what they look like. Scratch has ten; a Blockly profile brings its own
//! and adds world categories that Scratch never had. Keeping that in a `match`
//! meant every new language had to be spliced into the same expression, so it
//! lives in a `Palette` here instead — one table per theme, looked up by name.
//!
//! The palette is set once per document, like the inset scale and the writing
//! direction, because it is a property of the document and not of any one
//! block.

use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CategoryColors {
    pub fill: String,
    pub stroke: String,
    pub text: String,
    /// Secondary/darker fill for dropdowns and inner shapes.
    pub alt: String,
}

impl CategoryColors {
    pub(crate) fn new(fill: &str, stroke: &str, text: &str, alt: &str) -> Self {
        Self {
            fill: fill.to_string(),
            stroke: stroke.to_string(),
            text: text.to_string(),
            alt: alt.to_string(),
        }
    }
}

pub struct Palette {
    /// Full-colour categories.
    normal: HashMap<String, CategoryColors>,
    /// Lighter, higher-contrast variants. A category missing here falls back
    /// to its normal colours, which is what the previous match did.
    high_contrast: HashMap<String, CategoryColors>,
    /// Greys that keep the categories apart on a photocopy.
    grayscale: HashMap<String, CategoryColors>,
    unknown: CategoryColors,
    unknown_gray: CategoryColors,
}

fn table(entries: &[(&str, &str, &str, &str, &str)]) -> HashMap<String, CategoryColors> {
    entries
        .iter()
        .map(|(name, fill, stroke, text, alt)| {
            (name.to_string(), CategoryColors::new(fill, stroke, text, alt))
        })
        .collect()
}

impl Palette {
    /// The Scratch 3 palette, unchanged from what the renderer always drew.
    pub fn scratch() -> Self {
        Self {
            normal: table(&[
                ("motion", "#4C97FF", "#3373CC", "#ffffff", "#4280D7"),
                ("looks", "#9966FF", "#774DCB", "#ffffff", "#855CD6"),
                ("sound", "#CF63CF", "#BD42BD", "#ffffff", "#C94FC9"),
                ("events", "#FFBF00", "#CC9900", "#ffffff", "#E6AC00"),
                ("control", "#FFAB19", "#CF8B17", "#ffffff", "#EC9C13"),
                ("sensing", "#5CB1D6", "#2E8EB8", "#ffffff", "#47A8D1"),
                ("operators", "#59C059", "#389438", "#ffffff", "#46B946"),
                ("variables", "#FF8C1A", "#DB6E00", "#ffffff", "#FF8000"),
                ("lists", "#FF661A", "#E64D00", "#ffffff", "#FF5500"),
                ("custom", "#FF6680", "#FF3355", "#ffffff", "#FF4D6A"),
                ("custom-arg", "#FF6680", "#FF3355", "#ffffff", "#FF4D6A"),
                ("pen", "#0FBD8C", "#0B8E69", "#ffffff", "#0DA57A"),
            ]),
            high_contrast: table(&[
                ("motion", "#80B5FF", "#3373CC", "#000000", "#B3D2FF"),
                ("looks", "#CCB3FF", "#774DCB", "#000000", "#DDCCFF"),
                ("sound", "#E19DE1", "#BD42BD", "#000000", "#FFB3FF"),
                ("events", "#FFD966", "#CC9900", "#000000", "#FFECB3"),
                ("control", "#FFBE4C", "#CF8B17", "#000000", "#FFDA99"),
                ("sensing", "#85C4E0", "#2E8EB8", "#000000", "#AED8EA"),
                ("operators", "#7ECE7E", "#389438", "#000000", "#B5E3B5"),
                ("variables", "#FFA54C", "#DB6E00", "#000000", "#FFCC99"),
                ("lists", "#FF9966", "#E64D00", "#000000", "#FFB380"),
                ("custom", "#FF99AA", "#FF3355", "#000000", "#FFB3C2"),
                ("custom-arg", "#FF99AA", "#FF3355", "#000000", "#FFB3C2"),
                ("pen", "#13ECAF", "#0B8E69", "#000000", "#45F0C2"),
            ]),
            grayscale: gray_table(&[
                ("looks", 0),
                ("sound", 1),
                ("lists", 2),
                ("motion", 3),
                ("custom", 4),
                ("custom-arg", 4),
                ("pen", 5),
                ("sensing", 6),
                ("variables", 7),
                ("operators", 8),
                ("control", 9),
                ("events", 10),
            ]),
            unknown: CategoryColors::new("#bfbfbf", "#909090", "#ffffff", "#b2b2b2"),
            unknown_gray: CategoryColors::new("#f2f2f2", "#9a9a9a", "#000000", "#e2e2e2"),
        }
    }

    pub fn colors_for(&self, category: &str, theme: &str) -> CategoryColors {
        if theme == "print" {
            return CategoryColors::new("#ffffff", "#000000", "#000000", "#e0e0e0");
        }
        if theme == "grayscale" {
            return self
                .grayscale
                .get(category)
                .cloned()
                .unwrap_or_else(|| self.unknown_gray.clone());
        }
        if theme == "high-contrast" {
            if let Some(colors) = self.high_contrast.get(category) {
                return colors.clone();
            }
        }
        self.normal
            .get(category)
            .cloned()
            .unwrap_or_else(|| self.unknown.clone())
    }
}

/// A greyscale palette that keeps the categories apart.
///
/// The obvious implementation — convert each Scratch colour to its own
/// luminance — does not work. Scratch picks hues, not lightnesses, so the
/// eleven categories land almost on top of each other once the hue is gone:
/// `lists` #FF661A and `motion` #4C97FF both come out at grey 151, and `pen`
/// and `sensing` both at 167. A page printed that way is unreadable in a way
/// the colour version never is, because the reader has lost the one cue that
/// told a control block from an operator.
///
/// So the scale is assigned rather than computed. Categories are ordered by
/// their true luminance — which keeps the palette recognisable to anyone who
/// knows the colour version — then spread evenly from 96 to 232, giving a
/// constant 14-level step between neighbours.
///
/// Label colour is chosen by measured WCAG contrast rather than by eye, and
/// the crossover is earlier than it looks: white text on `pen` #a4a4a4 scores
/// 2.49:1 where black scores 8.42:1. Everything from `lists` #7b7b7b upwards
/// therefore takes black, leaving only the three darkest categories white.
///
/// This is a separate theme, not a change to `print`. `print` renders every
/// block white with a black outline, matching what ProfCollege's
/// `[Impression]` key does, and some users want exactly that.
fn gray_table(order: &[(&str, usize)]) -> HashMap<String, CategoryColors> {
    order
        .iter()
        .map(|(name, step)| (name.to_string(), gray(*step)))
        .collect()
}

/// Eleven steps from 96 to 232, ordered by the luminance of the colour each
/// one replaces. `alt` (dropdowns, inner fills) is one step darker than
/// `fill` so a menu still reads as inset.
const GRAY_VALUES: [(&str, &str, &str, &str); 11] = [
    ("#606060", "#3c3c3c", "#ffffff", "#525252"),
    ("#6e6e6e", "#444444", "#ffffff", "#606060"),
    ("#7b7b7b", "#4c4c4c", "#000000", "#6d6d6d"),
    ("#898989", "#555555", "#000000", "#7b7b7b"),
    ("#969696", "#5d5d5d", "#000000", "#888888"),
    ("#a4a4a4", "#666666", "#000000", "#969696"),
    ("#b2b2b2", "#6e6e6e", "#000000", "#a4a4a4"),
    ("#bfbfbf", "#767676", "#000000", "#b1b1b1"),
    ("#cdcdcd", "#7f7f7f", "#000000", "#bfbfbf"),
    ("#dadada", "#878787", "#000000", "#cccccc"),
    ("#e8e8e8", "#909090", "#000000", "#dadada"),
];

fn gray(step: usize) -> CategoryColors {
    let (fill, stroke, text, alt) = GRAY_VALUES[step];
    CategoryColors::new(fill, stroke, text, alt)
}

thread_local! {
    static CURRENT: RefCell<Option<Palette>> = const { RefCell::new(None) };
}

/// Install the palette for the document about to be rendered.
pub fn set_palette(palette: Palette) {
    CURRENT.with(|current| *current.borrow_mut() = Some(palette));
}

pub fn clear_palette() {
    CURRENT.with(|current| *current.borrow_mut() = None);
}

/// Look up a category in the current palette, falling back to Scratch's.
pub fn colors_for(category: &str, theme: &str) -> CategoryColors {
    CURRENT.with(|current| {
        let mut borrowed = current.borrow_mut();
        if borrowed.is_none() {
            *borrowed = Some(Palette::scratch());
        }
        borrowed.as_ref().unwrap().colors_for(category, theme)
    })
}

// ---------------------------------------------------------------------------
// Palettes from profiles
//
// A profile TOML names its categories and gives each a fill. Everything else
// a category needs — outline, dropdown fill, label colour, the high-contrast
// and greyscale variants — is derived here, so a profile stays a short list
// of colours rather than a table of twelve values per category.
// ---------------------------------------------------------------------------

#[derive(serde::Deserialize)]
struct ProfileToml {
    #[serde(default)]
    inherits: Option<String>,
    #[serde(default)]
    colors: HashMap<String, String>,
}

/// Resolve a profile name to its merged colour table, following `inherits`.
fn profile_colors(name: &str) -> Option<Vec<(String, String)>> {
    let source = crate::generated::PROFILE_DATA
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, toml)| *toml)?;
    let profile: ProfileToml = toml::from_str(source).ok()?;
    // Base first, so the profile's own entries override it while the base's
    // ordering — which fixes the greyscale ranking — is kept for the rest.
    let mut merged: Vec<(String, String)> = profile
        .inherits
        .as_deref()
        .and_then(profile_colors)
        .unwrap_or_default();
    // toml::from_str yields the table in file order only through a map with
    // preserve_order; sort by name for a stable order independent of that.
    let mut own: Vec<(String, String)> = profile.colors.into_iter().collect();
    own.sort();
    for (category, fill) in own {
        match merged.iter_mut().find(|(c, _)| *c == category) {
            Some(entry) => entry.1 = fill,
            None => merged.push((category, fill)),
        }
    }
    Some(merged)
}

fn parse_hex(hex: &str) -> Option<(f32, f32, f32)> {
    let hex = hex.trim().trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let channel = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).ok().map(|v| v as f32 / 255.0);
    Some((channel(0)?, channel(2)?, channel(4)?))
}

fn to_hex(r: f32, g: f32, b: f32) -> String {
    let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!("#{:02x}{:02x}{:02x}", byte(r), byte(g), byte(b))
}

/// Relative luminance as WCAG defines it, so the label colour is chosen by
/// measured contrast rather than by eye.
fn luminance(r: f32, g: f32, b: f32) -> f32 {
    let lin = |c: f32| if c <= 0.03928 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) };
    0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b)
}

fn contrast_text(r: f32, g: f32, b: f32) -> &'static str {
    // Contrast against white vs. against black; pick the better one.
    let l = luminance(r, g, b);
    if (1.05) / (l + 0.05) >= (l + 0.05) / 0.05 { "#ffffff" } else { "#000000" }
}

fn derive(fill: &str) -> Option<(CategoryColors, CategoryColors, f32)> {
    let (r, g, b) = parse_hex(fill)?;
    let normal = CategoryColors {
        fill: to_hex(r, g, b),
        // Blockly's tertiary colour, the outline of today's blocks: the
        // fill pulled three tenths towards white (#5b80a5 -> #8ca6c0).
        stroke: to_hex(r + (1.0 - r) * 0.3, g + (1.0 - g) * 0.3, b + (1.0 - b) * 0.3),
        // Blockly labels are always white, even on its lightest greens;
        // the contrast rule is kept for the greyscale variant, where it
        // matters.
        text: "#ffffff".to_string(),
        alt: to_hex(r * 0.9, g * 0.9, b * 0.9),
    };
    let _ = contrast_text(r, g, b);
    // High contrast: pull the fill towards white and switch to black labels,
    // as the Scratch high-contrast palette does.
    let mix = |c: f32| c + (1.0 - c) * 0.45;
    let high = CategoryColors {
        fill: to_hex(mix(r), mix(g), mix(b)),
        stroke: normal.stroke.clone(),
        text: "#000000".to_string(),
        alt: to_hex(mix(r) * 0.95, mix(g) * 0.95, mix(b) * 0.95),
    };
    Some((normal, high, luminance(r, g, b)))
}

impl Palette {
    /// Build a palette from a profile's colour list.
    ///
    /// The greyscale variant follows the same rule as Scratch's: converting
    /// each colour to its own luminance lets categories collapse onto one
    /// grey, so the steps are *assigned* — categories ranked by luminance,
    /// then spread evenly over the eleven-step scale. Neighbours therefore
    /// always differ by at least one step, whatever the profile's colours.
    pub fn from_colors(colors: &[(String, String)]) -> Self {
        let mut normal = HashMap::new();
        let mut high_contrast = HashMap::new();
        let mut ranked: Vec<(String, f32)> = Vec::new();
        for (category, fill) in colors {
            if let Some((n, h, lum)) = derive(fill) {
                normal.insert(category.clone(), n);
                high_contrast.insert(category.clone(), h);
                ranked.push((category.clone(), lum));
            }
        }
        ranked.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        let count = ranked.len().max(1);
        let grayscale = ranked
            .iter()
            .enumerate()
            .map(|(rank, (category, _))| {
                let step = if count == 1 { 5 } else { (rank * (GRAY_VALUES.len() - 1)) / (count - 1) };
                (category.clone(), gray(step))
            })
            .collect();
        Self {
            normal,
            high_contrast,
            grayscale,
            unknown: CategoryColors::new("#bfbfbf", "#909090", "#ffffff", "#b2b2b2"),
            unknown_gray: CategoryColors::new("#f2f2f2", "#9a9a9a", "#000000", "#e2e2e2"),
        }
    }
}

/// The palette for a profile with the document's own colours laid over it.
/// A Blockly profile is rebuilt from its merged colour list, so the
/// greyscale ranking takes the overrides into account; Scratch keeps its
/// hand-made tables and only the overridden categories are re-derived.
pub fn palette_with(profile: Option<&str>, overrides: &[(String, String)]) -> Palette {
    let name = match profile {
        Some("blockly") | Some("blockly-modern") => Some("blockly-modern"),
        Some("blockly-classic") | Some("blockly-klassisch") => Some("blockly-klassisch"),
        Some("scratch") | None => None,
        Some(other) => Some(other),
    };
    let colors = name.and_then(profile_colors);
    match colors {
        Some(mut colors) => {
            for (category, fill) in overrides {
                match colors.iter_mut().find(|(c, _)| c == category) {
                    Some(entry) => entry.1 = fill.clone(),
                    None => colors.push((category.clone(), fill.clone())),
                }
            }
            Palette::from_colors(&colors)
        }
        None => {
            let mut palette = Palette::scratch();
            for (category, fill) in overrides {
                if let Some((normal, high, lum)) = derive(fill) {
                    palette.normal.insert(category.clone(), normal);
                    palette.high_contrast.insert(category.clone(), high);
                    // A category Scratch has keeps its grey; a new one gets
                    // the grey nearest its own luminance.
                    if !palette.grayscale.contains_key(category) {
                        let step = ((1.0 - lum.clamp(0.0, 1.0)) * (GRAY_VALUES.len() - 1) as f32).round() as usize;
                        palette.grayscale.insert(category.clone(), gray(step));
                    }
                }
            }
            palette
        }
    }
}
