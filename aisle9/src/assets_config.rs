use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Assets {
    #[serde(rename = "Models")]
    pub models: HashMap<String, ModelEntry>,
    #[serde(rename = "Textures")]
    pub textures: HashMap<String, TextureEntry>,
}

#[derive(Deserialize, Debug)]
pub struct ModelEntry {
    #[serde(rename = "File")]
    pub file: String,
}

#[derive(Deserialize, Debug)]
pub struct TextureEntry {
    #[serde(rename = "File")]
    pub file: String,
}
