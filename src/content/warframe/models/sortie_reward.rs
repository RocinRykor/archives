use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<SortieReward>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortieReward {
    pub item_count: i64,
    pub probability: f64,
    pub rarity: String,
    pub reward_name: String,
    pub tier: i64,
}
