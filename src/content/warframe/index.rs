use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct NameIndex {
    entries: HashMap<String, String>,
}

impl NameIndex {
    pub fn get(&self, path: &str) -> Option<&str> {
        self.entries.get(path).map(|s| s.as_str())
    }

    pub fn resolve(&self, path: &str) -> Option<&str> {
        self.get(path)
            .or_else(|| self.get(&path.replace("/Lotus/StoreItems/", "/Lotus/")))
            .or_else(|| self.get(&path.replace("/Lotus/", "/Lotus/StoreItems/")))
    }

    fn collect_json_files(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut files = Vec::new();
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                files.extend(Self::collect_json_files(&path)?);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                files.push(path);
            }
        }
        Ok(files)
    }

    pub fn build(split_dir: &Path) -> Result<Self, std::io::Error> {
        let mut entries: HashMap<String, String> = HashMap::new();
        let mut recipe_files: Vec<PathBuf> = Vec::new();

        let all_files = Self::collect_json_files(split_dir)?;

        // First pass — direct uniqueName + name entries
        for path in &all_files {
            let contents = fs::read_to_string(path)?;
            let items: Vec<Value> = match serde_json::from_str(&contents) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let is_recipe = items
                .first()
                .and_then(|i| i.as_object())
                .map(|o| o.contains_key("resultType") && !o.contains_key("name"))
                .unwrap_or(false);

            if is_recipe {
                recipe_files.push(path.clone());
                continue;
            }

            for item in &items {
                if let (Some(unique_name), Some(name)) = (
                    item.get("uniqueName").and_then(|v| v.as_str()),
                    item.get("name").and_then(|v| v.as_str()),
                ) {
                    entries.insert(unique_name.to_string(), name.to_string());
                }
            }
        }

        // Second pass — recipes resolved against first pass
        for path in recipe_files {
            let contents = fs::read_to_string(&path)?;
            let items: Vec<Value> = match serde_json::from_str(&contents) {
                Ok(v) => v,
                Err(_) => continue,
            };

            for item in &items {
                let unique_name = match item.get("uniqueName").and_then(|v| v.as_str()) {
                    Some(u) => u,
                    None => continue,
                };
                let result_type = match item.get("resultType").and_then(|v| v.as_str()) {
                    Some(r) => r,
                    None => continue,
                };

                let result_name = entries
                    .get(result_type)
                    .or_else(|| entries.get(&result_type.replace("/Lotus/StoreItems/", "/Lotus/")))
                    .or_else(|| entries.get(&result_type.replace("/Lotus/", "/Lotus/StoreItems/")))
                    .cloned();

                if let Some(name) = result_name {
                    let display = format!("{} Blueprint", name);
                    entries.insert(unique_name.to_string(), display.clone());
                    entries.insert(
                        unique_name.replace("/Lotus/", "/Lotus/StoreItems/"),
                        display.clone(),
                    );
                    entries.insert(
                        unique_name.replace("/Lotus/StoreItems/", "/Lotus/"),
                        display,
                    );
                }
            }
        }

        Ok(Self { entries })
    }
}
