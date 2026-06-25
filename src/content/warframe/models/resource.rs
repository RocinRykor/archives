use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Resource>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub codex_secret: bool,
    pub description: String,
    pub exclude_from_codex: Option<bool>,
    pub name: String,
    pub parent_name: String,
    pub unique_name: String,
    pub show_in_inventory: Option<bool>,
    pub long_description: Option<String>,
    pub prime_selling_price: Option<i64>,
}
