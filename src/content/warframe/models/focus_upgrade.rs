use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<FocusUpgrade>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusUpgrade {
    pub base_drain: i64,
    pub codex_secret: bool,
    pub exclude_from_codex: bool,
    pub fusion_limit: i64,
    pub level_stats: Vec<LevelStat>,
    pub name: String,
    pub polarity: String,
    pub rarity: String,
    pub unique_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelStat {
    pub stats: Vec<String>,
}
