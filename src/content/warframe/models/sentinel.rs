use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Sentinel>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sentinel {
    pub armor: i64,
    pub codex_secret: bool,
    pub description: String,
    pub exclude_from_codex: Option<bool>,
    pub health: i64,
    pub name: String,
    pub power: i64,
    pub product_category: String,
    pub shield: i64,
    pub stamina: i64,
    pub unique_name: String,
}
