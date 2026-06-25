use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Recipe>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub build_price: i64,
    pub build_time: i64,
    pub codex_secret: bool,
    pub consume_on_use: bool,
    pub ingredients: Vec<Ingredient>,
    pub num: i64,
    pub result_type: String,
    pub secret_ingredients: Vec<SecretIngredient>,
    pub skip_build_time_price: i64,
    pub unique_name: String,
    pub exclude_from_codex: Option<bool>,
    pub prime_selling_price: Option<i64>,
    pub always_available: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    #[serde(rename = "ItemCount")]
    pub item_count: i64,
    #[serde(rename = "ItemType")]
    pub item_type: String,
    #[serde(rename = "ProductCategory")]
    pub product_category: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretIngredient {
    #[serde(rename = "ItemCount")]
    pub item_count: i64,
    #[serde(rename = "ItemType")]
    pub item_type: String,
}
