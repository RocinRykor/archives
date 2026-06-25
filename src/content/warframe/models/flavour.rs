use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Flavour>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flavour {
    pub codex_secret: bool,
    pub description: String,
    pub exclude_from_codex: Option<bool>,
    pub name: String,
    pub unique_name: String,
    #[serde(default)]
    pub hex_colours: Vec<HexColour>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HexColour {
    pub value: String,
}
