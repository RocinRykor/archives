use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Weapon>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
    pub accuracy: Option<f64>,
    pub codex_secret: bool,
    pub critical_chance: f64,
    pub critical_multiplier: f64,
    pub damage_per_shot: Vec<f64>,
    pub description: String,
    pub fire_rate: f64,
    pub magazine_size: Option<i64>,
    pub mastery_req: i64,
    pub multishot: Option<i64>,
    pub name: String,
    pub noise: Option<String>,
    pub omega_attenuation: f64,
    pub proc_chance: f64,
    pub product_category: String,
    pub reload_time: Option<f64>,
    pub slot: Option<i64>,
    pub total_damage: f64,
    pub trigger: Option<String>,
    pub unique_name: String,
    pub blocking_angle: Option<i64>,
    pub combo_duration: Option<i64>,
    pub follow_through: Option<f64>,
    pub heavy_attack_damage: Option<i64>,
    pub heavy_slam_attack: Option<i64>,
    pub heavy_slam_radial_damage: Option<i64>,
    pub heavy_slam_radius: Option<i64>,
    pub range: Option<f64>,
    pub slam_attack: Option<i64>,
    pub slam_radial_damage: Option<i64>,
    pub slam_radius: Option<i64>,
    pub slide_attack: Option<i64>,
    pub wind_up: Option<f64>,
    pub max_level_cap: Option<i64>,
    pub sentinel: Option<bool>,
    pub exclude_from_codex: Option<bool>,
    pub prime_omega_attenuation: Option<f64>,
}
