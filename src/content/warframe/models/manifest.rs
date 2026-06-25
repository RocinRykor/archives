use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Manifest>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub texture_location: String,
    pub unique_name: String,
}
