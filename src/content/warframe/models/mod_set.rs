use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<ModSet>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModSet {
    pub num_upgrades_in_set: i64,
    pub stats: Vec<String>,
    pub unique_name: String,
    pub buff_set: Option<bool>,
}
