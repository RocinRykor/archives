use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Avionic>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avionic {
    pub base_drain: i64,
    pub codex_secret: bool,
    pub fusion_limit: i64,
    #[serde(default)]
    pub level_stats: Vec<LevelStat>,
    pub name: String,
    pub polarity: String,
    pub rarity: String,
    pub unique_name: String,
    pub exclude_from_codex: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelStat {
    pub stats: Vec<String>,
}
