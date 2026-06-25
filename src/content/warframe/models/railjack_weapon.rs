use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<RailjackWeapon>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RailjackWeapon {
    pub accuracy: f64,
    pub codex_secret: bool,
    pub critical_chance: f64,
    pub critical_multiplier: f64,
    pub damage_per_shot: Vec<f64>,
    pub description: String,
    pub exclude_from_codex: bool,
    pub fire_rate: f64,
    pub magazine_size: i64,
    pub mastery_req: i64,
    pub multishot: i64,
    pub name: String,
    pub noise: String,
    pub omega_attenuation: f64,
    pub proc_chance: f64,
    pub product_category: String,
    pub reload_time: f64,
    pub slot: i64,
    pub total_damage: f64,
    pub trigger: String,
    pub unique_name: String,
}
