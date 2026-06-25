use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Ability>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub ability_name: String,
    pub ability_unique_name: String,
    pub description: String,
}
