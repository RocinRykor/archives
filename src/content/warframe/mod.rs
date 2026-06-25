use serde::{Deserialize, Serialize};

pub mod public_export;
pub mod language_code;
pub mod export_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Warframe {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,

    pub name: String,
    pub description: String,

    #[serde(rename = "longDescription")]
    pub long_description: Option<String>,

    #[serde(rename = "passiveDescription")]
    pub passive_description: Option<String>,

    #[serde(rename = "parentName")]
    pub parent_name: String,

    #[serde(rename = "productCategory")]
    pub product_category: String,

    #[serde(rename = "masteryReq")]
    pub mastery_req: i32,

    pub health: i32,
    pub shield: i32,
    pub armor: i32,
    pub power: i32,
    pub stamina: i32,

    #[serde(rename = "sprintSpeed")]
    pub sprint_speed: f64,

    #[serde(rename = "codexSecret")]
    pub codex_secret: bool,

    #[serde(rename = "excludeFromCodex")]
    pub exclude_from_codex: Option<bool>,

    pub abilities: Vec<serde_json::Value>,

    pub exalted: Option<Vec<serde_json::Value>>,
}