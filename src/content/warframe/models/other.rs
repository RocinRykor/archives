use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Other>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Other {
    pub description: String,
    pub exclude_from_codex: Option<bool>,
    pub name: String,
    pub unique_name: String,
}
