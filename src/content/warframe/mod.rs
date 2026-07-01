pub mod export_json;
pub mod index;
pub mod language_code;
pub mod listable;
pub mod models;

use crate::index::NameIndex;
use crate::listable::Listable;
use crate::relic::Relic;
pub use export_json::*;
pub use language_code::*;
pub use models::*;
use std::fs;
use std::path::Path;

pub fn list_file(path: &Path, index: &NameIndex) -> Result<String, std::io::Error> {
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let contents = fs::read_to_string(path)?;

    let mut sections = Vec::new();

    match stem {
        "Relics" => {
            let items: Vec<Relic> =
                serde_json::from_str(&contents).expect("failed to parse Relics");

            // Deduplicate by name — keep only Bronze (Intact)
            let mut seen = std::collections::HashSet::new();
            for item in &items {
                if item.unique_name.contains("Bronze") && seen.insert(&item.name) {
                    sections.push(item.list(index));
                }
            }
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("no Listable impl for {}", stem),
            ));
        }
    }

    Ok(sections.join("\n"))
}
