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
