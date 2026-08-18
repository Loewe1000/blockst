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
    fn new(fill: &str, stroke: &str, text: &str, alt: &str) -> Self {
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
