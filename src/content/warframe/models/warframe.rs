use serde_derive::{Deserialize, Serialize};
use crate::ability::Ability;

pub type Root = Vec<Warframe>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Warframe {
    pub abilities: Vec<Ability>,
    pub armor: i64,
    pub codex_secret: bool,
    pub description: String,
    pub health: i64,
    pub mastery_req: i64,
    pub name: String,
    pub parent_name: String,
    pub power: i64,
    pub product_category: String,
    pub shield: i64,
    pub sprint_speed: f64,
    pub stamina: i64,
    pub unique_name: String,
    pub passive_description: Option<String>,
    #[serde(default)]
    pub exalted: Vec<String>,
    pub long_description: Option<String>,
    pub exclude_from_codex: Option<bool>,
}