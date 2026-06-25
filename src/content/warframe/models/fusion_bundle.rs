use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<FusionBundle>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FusionBundle {
    pub codex_secret: bool,
    pub description: String,
    pub fusion_points: i64,
    pub unique_name: String,
}
