use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DocumentSpec {
    pub scale: Option<f32>,
    pub theme: Option<String>,
    #[serde(default)]
    pub line_numbers: bool,
    #[serde(default = "default_line_number_start")]
    pub line_number_start: u32,
    #[serde(default = "default_line_number_first_block")]
    pub line_number_first_block: u32,
    #[serde(default = "default_line_number_gutter")]
    pub line_number_gutter: f32,
    #[serde(default = "default_inset_scale")]
    pub inset_scale: f32,
    #[serde(default = "default_font")]
    pub font: String,
    pub scripts: Vec<ScriptSpec>,
}

fn default_font() -> String {
    "Helvetica Neue, Helvetica, sans-serif".to_string()
}

fn default_line_number_start() -> u32 {
    1
}

fn default_line_number_first_block() -> u32 {
    1
}

fn default_line_number_gutter() -> f32 {
    24.0
}

fn default_inset_scale() -> f32 {
    1.0
}

#[derive(Debug, Deserialize)]
pub struct ScriptSpec {
    pub blocks: Vec<BlockSpec>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BlockSpec {
    pub shape: String,
    pub category: String,
    #[serde(default)]
    pub line_number: Option<u32>,
    #[serde(default)]
    pub segments: Vec<SegmentSpec>,
    #[serde(default)]
    pub body: Vec<BlockSpec>,
    #[serde(default)]
    pub else_body: Vec<BlockSpec>,
    #[serde(default)]
    pub else_segments: Vec<SegmentSpec>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum SegmentSpec {
    #[serde(rename = "text")]
    Text { value: String },
    #[serde(rename = "icon")]
    Icon { name: String },
    #[serde(rename = "input")]
    Input {
        input: String,
        #[serde(default)]
        value: String,
        #[serde(default)]
        color: String,
        #[serde(default)]
        nested: Option<Box<BlockSpec>>,
    },
    #[serde(rename = "block")]
    Block { block: Box<BlockSpec> },
}
