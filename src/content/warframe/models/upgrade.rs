use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Upgrade>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upgrade {
    pub base_drain: i64,
    pub codex_secret: bool,
    pub compat_name: Option<String>,
    #[serde(default)]
    pub description: Vec<String>,
    pub fusion_limit: i64,
    #[serde(default)]
    pub level_stats: Vec<LevelStat>,
    pub name: String,
    pub polarity: String,
    pub rarity: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub unique_name: String,
    pub is_utility: Option<bool>,
    pub mod_set: Option<String>,
    pub mod_set_values: Option<Vec<f64>>,
    pub subtype: Option<String>,
    pub exclude_from_codex: Option<bool>,
    #[serde(default)]
    pub available_challenges: Vec<AvailableChallenge>,
    #[serde(default)]
    pub upgrade_entries: Vec<UpgradeEntry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelStat {
    pub stats: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableChallenge {
    pub complications: Vec<Complication>,
    pub description: String,
    pub full_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Complication {
    pub description: String,
    pub full_name: String,
    pub override_tag: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeEntry {
    pub prefix_tag: String,
    pub suffix_tag: String,
    pub tag: String,
    pub upgrade_values: Vec<UpgradeValue>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeValue {
    pub value: f64,
    pub loc_tag: Option<String>,
    pub reverse_value_symbol: Option<bool>,
}
