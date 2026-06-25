use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Region>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    pub faction_index: i64,
    pub mastery_req: i64,
    pub max_enemy_level: i64,
    pub min_enemy_level: i64,
    pub mission_index: i64,
    pub name: String,
    pub node_type: i64,
    pub system_index: i64,
    pub system_name: String,
    pub unique_name: String,
}
