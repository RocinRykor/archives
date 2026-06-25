use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Key>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    pub codex_secret: bool,
    pub description: String,
    pub name: String,
    pub parent_name: String,
    pub unique_name: String,
    pub exclude_from_codex: Option<bool>,
}
