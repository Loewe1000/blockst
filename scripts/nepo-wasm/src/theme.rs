//! Open Roberta's palette, copied from the official constants.
//!
//! Source: OpenRoberta/blockly `core/constants.js` (`Blockly.CAT_*_RGB` and
//! `Blockly.DATA_TYPE`). These are taken verbatim rather than approximated,
//! because "the colours are exactly right" is one of the things the prototype
//! is supposed to demonstrate.

#[derive(Clone, Copy)]
pub struct BlockColours {
    /// Body fill. Open Roberta draws blocks as a flat fill: its stylesheet has
    /// no `.blocklyPath` stroke rule, and the fork comments out the highlight
    /// and shadow paths that stock Blockly draws.
    pub fill: &'static str,
    /// Only used by the `print` theme, which needs an outline to survive on
    /// paper. `None` means "no stroke", which is the faithful rendering.
    pub stroke: Option<&'static str>,
    /// `.blocklyText`
    pub text: &'static str,
    /// `.blocklyEditableText > rect`: white at 60% over the block fill.
    pub field_fill: &'static str,
    pub field_fill_opacity: &'static str,
    /// `.blocklyEditableText > text`
    pub field_text: &'static str,
}

/// `Blockly.CAT_*_RGB`
pub fn category_colour(category: &str) -> &'static str {
    match category {
        "activity" => "#E2001A",
        "action" => "#F29400",
        "control" => "#EB6A0A",
        "sensor" => "#8FA402",
        "colour" => "#EBC300",
        "image" => "#DF01D7",
        "text" => "#BACC1E",
        "math" => "#005A94",
        "logic" => "#33B8CA",
        "list" => "#39378B",
        "variable" => "#9085BA",
        "procedure" => "#179C7D",
        "communication" => "#FF69B4",
        "neural-network" | "nn" => "#00B6AC",
        "add-on" => "#666666",
        _ => "#666666",
    }
}

/// `Blockly.DATA_TYPE[...]` — the colour a typed, *empty* connection is drawn
/// in. This is what makes a NEPO socket announce what belongs in it.
pub fn data_type_colour(data_type: &str) -> &'static str {
    match data_type {
        "Number" => "#005A94",
        "String" => "#BACC1E",
        "Boolean" => "#33B8CA",
        "Colour" => "#EBC300",
        "Connection" => "#FF69B4",
        "Sensor" => "#8FA402",
        "Image" => "#DF01D7",
        "Actor" => "#F29400",
        t if t.starts_with("Array") => "#39378B",
        _ => "#666666",
    }
}

pub fn colours_for(category: &str, theme: &str) -> BlockColours {
    if theme == "print" {
        return BlockColours {
            fill: "#ffffff",
            stroke: Some("#000000"),
            text: "#000000",
            field_fill: "#ffffff",
            field_fill_opacity: "1",
            field_text: "#000000",
        };
    }

    BlockColours {
        fill: category_colour(category),
        stroke: None,
        text: "#ffffff",
        field_fill: "#ffffff",
        field_fill_opacity: "0.6",
        field_text: "#000000",
    }
}

/// In `print` the typed connections lose their colour coding, so they are
/// drawn black like everything else.
pub fn connection_colour(data_type: &str, theme: &str) -> &'static str {
    if theme == "print" {
        "#000000"
    } else {
        data_type_colour(data_type)
    }
}
