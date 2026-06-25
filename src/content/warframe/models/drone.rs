use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

pub type Root = Vec<Drone>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drone {
    pub bin_capacity: i64,
    pub bin_count: i64,
    pub capacity_multiplier: Vec<i64>,
    pub codex_secret: bool,
    pub description: String,
    pub durability: i64,
    pub fill_rate: f64,
    pub name: String,
    pub repair_rate: i64,
    pub specialities: Vec<Value>,
    pub unique_name: String,
}
