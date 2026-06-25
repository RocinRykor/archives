use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nightwave {
    pub affiliation_tag: String,
    pub challenges: Vec<Challenge>,
    pub rewards: Vec<Reward>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub description: String,
    pub name: String,
    pub required: i64,
    pub standing: i64,
    pub unique_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    pub item_count: Option<i64>,
    pub unique_name: String,
    #[serde(default)]
    pub components: Vec<String>,
    pub description: Option<String>,
    pub name: Option<String>,
}
