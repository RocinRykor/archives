use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Gear>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gear {
    pub codex_secret: bool,
    pub description: String,
    pub name: String,
    pub parent_name: String,
    pub unique_name: String,
}
