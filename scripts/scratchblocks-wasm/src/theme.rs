#[derive(Clone, Copy)]
pub struct CategoryColors {
    pub fill: &'static str,
    pub stroke: &'static str,
    pub text: &'static str,
    pub alt: &'static str, // Secondary/darker fill for dropdowns etc.
}

pub fn colors_for(category: &str, theme: &str) -> CategoryColors {
    if theme == "print" {
        return CategoryColors {
            fill: "#ffffff",
            stroke: "#000000",
            text: "#000000",
            alt: "#e0e0e0",
        };
    }

    if theme == "grayscale" {
        return grayscale_colors(category);
    }

    match (theme, category) {
        ("high-contrast", "motion") => CategoryColors { fill: "#80B5FF", stroke: "#3373CC", text: "#000000", alt: "#B3D2FF" },
        ("high-contrast", "looks") => CategoryColors { fill: "#CCB3FF", stroke: "#774DCB", text: "#000000", alt: "#DDCCFF" },
        ("high-contrast", "sound") => CategoryColors { fill: "#E19DE1", stroke: "#BD42BD", text: "#000000", alt: "#FFB3FF" },
        ("high-contrast", "events") => CategoryColors { fill: "#FFD966", stroke: "#CC9900", text: "#000000", alt: "#FFECB3" },
        ("high-contrast", "control") => CategoryColors { fill: "#FFBE4C", stroke: "#CF8B17", text: "#000000", alt: "#FFDA99" },
        ("high-contrast", "sensing") => CategoryColors { fill: "#85C4E0", stroke: "#2E8EB8", text: "#000000", alt: "#AED8EA" },
        ("high-contrast", "operators") => CategoryColors { fill: "#7ECE7E", stroke: "#389438", text: "#000000", alt: "#B5E3B5" },
        ("high-contrast", "variables") => CategoryColors { fill: "#FFA54C", stroke: "#DB6E00", text: "#000000", alt: "#FFCC99" },
        ("high-contrast", "lists") => CategoryColors { fill: "#FF9966", stroke: "#E64D00", text: "#000000", alt: "#FFB380" },
        ("high-contrast", "custom") => CategoryColors { fill: "#FF99AA", stroke: "#FF3355", text: "#000000", alt: "#FFB3C2" },
        ("high-contrast", "pen") => CategoryColors { fill: "#13ECAF", stroke: "#0B8E69", text: "#000000", alt: "#45F0C2" },
        (_, "motion") => CategoryColors { fill: "#4C97FF", stroke: "#3373CC", text: "#ffffff", alt: "#4280D7" },
        (_, "looks") => CategoryColors { fill: "#9966FF", stroke: "#774DCB", text: "#ffffff", alt: "#855CD6" },
        (_, "sound") => CategoryColors { fill: "#CF63CF", stroke: "#BD42BD", text: "#ffffff", alt: "#C94FC9" },
        (_, "events") => CategoryColors { fill: "#FFBF00", stroke: "#CC9900", text: "#ffffff", alt: "#E6AC00" },
        (_, "control") => CategoryColors { fill: "#FFAB19", stroke: "#CF8B17", text: "#ffffff", alt: "#EC9C13" },
        (_, "sensing") => CategoryColors { fill: "#5CB1D6", stroke: "#2E8EB8", text: "#ffffff", alt: "#47A8D1" },
        (_, "operators") => CategoryColors { fill: "#59C059", stroke: "#389438", text: "#ffffff", alt: "#46B946" },
        (_, "variables") => CategoryColors { fill: "#FF8C1A", stroke: "#DB6E00", text: "#ffffff", alt: "#FF8000" },
        (_, "lists") => CategoryColors { fill: "#FF661A", stroke: "#E64D00", text: "#ffffff", alt: "#FF5500" },
        (_, "custom") => CategoryColors { fill: "#FF6680", stroke: "#FF3355", text: "#ffffff", alt: "#FF4D6A" },
        ("high-contrast", "custom-arg") => CategoryColors { fill: "#FF99AA", stroke: "#FF3355", text: "#000000", alt: "#FFB3C2" },
        (_, "custom-arg") => CategoryColors { fill: "#FF6680", stroke: "#FF3355", text: "#ffffff", alt: "#FF4D6A" },
        (_, "pen") => CategoryColors { fill: "#0FBD8C", stroke: "#0B8E69", text: "#ffffff", alt: "#0DA57A" },
        _ => CategoryColors { fill: "#bfbfbf", stroke: "#909090", text: "#ffffff", alt: "#b2b2b2" },
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
fn grayscale_colors(category: &str) -> CategoryColors {
    match category {
        "looks" => GRAY[0],
        "sound" => GRAY[1],
        "lists" => GRAY[2],
        "motion" => GRAY[3],
        "custom" | "custom-arg" => GRAY[4],
        "pen" => GRAY[5],
        "sensing" => GRAY[6],
        "variables" => GRAY[7],
        "operators" => GRAY[8],
        "control" => GRAY[9],
        "events" => GRAY[10],
        _ => CategoryColors { fill: "#f2f2f2", stroke: "#9a9a9a", text: "#000000", alt: "#e2e2e2" },
    }
}

/// Eleven steps from 96 to 232, ordered by the luminance of the colour each
/// one replaces. `alt` (dropdowns, inner fills) is one step darker than
/// `fill` so a menu still reads as inset.
const GRAY: [CategoryColors; 11] = [
    CategoryColors { fill: "#606060", stroke: "#3c3c3c", text: "#ffffff", alt: "#525252" },
    CategoryColors { fill: "#6e6e6e", stroke: "#444444", text: "#ffffff", alt: "#606060" },
    CategoryColors { fill: "#7b7b7b", stroke: "#4c4c4c", text: "#000000", alt: "#6d6d6d" },
    CategoryColors { fill: "#898989", stroke: "#555555", text: "#000000", alt: "#7b7b7b" },
    CategoryColors { fill: "#969696", stroke: "#5d5d5d", text: "#000000", alt: "#888888" },
    CategoryColors { fill: "#a4a4a4", stroke: "#666666", text: "#000000", alt: "#969696" },
    CategoryColors { fill: "#b2b2b2", stroke: "#6e6e6e", text: "#000000", alt: "#a4a4a4" },
    CategoryColors { fill: "#bfbfbf", stroke: "#767676", text: "#000000", alt: "#b1b1b1" },
    CategoryColors { fill: "#cdcdcd", stroke: "#7f7f7f", text: "#000000", alt: "#bfbfbf" },
    CategoryColors { fill: "#dadada", stroke: "#878787", text: "#000000", alt: "#cccccc" },
    CategoryColors { fill: "#e8e8e8", stroke: "#909090", text: "#000000", alt: "#dadada" },
];
