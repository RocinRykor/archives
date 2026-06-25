use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Custom>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Custom {
    pub codex_secret: bool,
    pub description: Option<String>,
    pub name: String,
    pub unique_name: String,
    pub exclude_from_codex: Option<bool>,
}
