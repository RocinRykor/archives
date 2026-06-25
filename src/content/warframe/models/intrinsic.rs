use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Intrinsic>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Intrinsic {
    pub name: String,
    pub ranks: Vec<Rank>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub description: String,
    pub name: String,
}
