use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub(crate) struct TagValue {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub(crate) struct TagVariant {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub(crate) struct Frame {
    pub(crate) title: String,
    pub(crate) toggles: Vec<String>,
}

pub(crate) fn default_to_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub(crate) struct AnimateData {
    pub frames: Vec<Frame>,
    pub tags: Vec<String>,
    #[serde(rename = "tag-values", default = "Vec::new")]
    pub tag_values: Vec<TagValue>,
    pub variants: Vec<TagVariant>,
    pub delay: u16,
    #[serde(rename = "auto-start", default = "default_to_true")]
    pub auto_start: bool,
    #[serde(rename = "min-frame")]
    pub min_frame: Option<usize>,
    #[serde(rename = "max-frame")]
    pub max_frame: Option<usize>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub(crate) struct DiagramMeta {
    // information for verbatim mermaid, hope there is no other tag
    pub config: Option<serde_yaml::Value>,
    pub title: Option<String>,
    pub animate: Option<AnimateData>,
    #[serde(rename = "animate-yml-file")]
    pub animate_yml_file: Option<String>,
}
