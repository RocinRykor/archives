use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::index::NameIndex;
use crate::listable::Listable;

pub type Root = Vec<Relic>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relic {
    pub codex_secret: bool,
    pub description: String,
    pub name: String,
    pub relic_rewards: Vec<RelicReward>,
    pub unique_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelicReward {
    pub item_count: i64,
    pub rarity: String,
    pub reward_name: String,
    pub tier: i64,
}

impl Listable for Relic {
    fn list(&self, index: &NameIndex) -> String {
        let mut lines = Vec::new();

        lines.push(format!("## {}", self.name));
        lines.push(String::new());

        let mut common = Vec::new();
        let mut uncommon = Vec::new();
        let mut rare = Vec::new();

        for reward in &self.relic_rewards {
            let name = index.resolve(&reward.reward_name)
                .unwrap_or(&reward.reward_name)
                .to_string();

            let entry = if reward.item_count > 1 {
                format!("  {}x {}", reward.item_count, name)
            } else {
                format!("  {}", name)
            };

            match reward.rarity.as_str() {
                "COMMON"   => common.push(entry),
                "UNCOMMON" => uncommon.push(entry),
                "RARE"     => rare.push(entry),
                _          => common.push(entry),
            }
        }

        if !common.is_empty() {
            lines.push("Common".to_string());
            lines.extend(common);
            lines.push(String::new());
        }
        if !uncommon.is_empty() {
            lines.push("Uncommon".to_string());
            lines.extend(uncommon);
            lines.push(String::new());
        }
        if !rare.is_empty() {
            lines.push("Rare".to_string());
            lines.extend(rare);
            lines.push(String::new());
        }

        lines.join("\n")
    }
}