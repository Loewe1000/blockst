use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DocumentSpec {
    pub scale: Option<f32>,
    pub theme: Option<String>,
    #[serde(default = "default_font")]
    pub font: String,
    pub scripts: Vec<ScriptSpec>,
}

fn default_font() -> String {
    "Helvetica Neue, Helvetica, sans-serif".to_string()
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
