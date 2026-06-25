use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Arcane>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arcane {
    pub codex_secret: bool,
    pub level_stats: Vec<LevelStat>,
    pub name: String,
    pub rarity: Option<String>,
    pub unique_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelStat {
    pub stats: Vec<String>,
}
