use std::path::Path;
use cortex::primitives::JsonFile;

#[derive(Debug)]
pub enum ExportJsonError {
    Io(std::io::Error),
    InvalidJson(serde_json::Error),
    WrongExtension(String),
}

impl From<std::io::Error> for ExportJsonError {
    fn from(e: std::io::Error) -> Self {
        ExportJsonError::Io(e)
    }
}

pub struct ExportJson {
    pub original_file: JsonFile,
}

impl ExportJson {
    pub fn new(file: JsonFile) -> Result<ExportJson, ExportJsonError> {
        // Even pre-built files get validated
        file.parse().map_err(ExportJsonError::InvalidJson)?;
        Ok(ExportJson { original_file: file })
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ExportJson, ExportJsonError> {
        let path = path.as_ref();

        // Reject anything that doesn't at least claim to be JSON
        match path.extension().and_then(|e| e.to_str()) {
            Some("json") => {}
            Some(ext) => return Err(ExportJsonError::WrongExtension(ext.to_string())),
            None => return Err(ExportJsonError::WrongExtension("(none)".to_string())),
        }

        let data = std::fs::read(path)?;

        let title = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let file = JsonFile::new(title, data);

        // Validate contents before handing back a usable struct
        file.parse().map_err(ExportJsonError::InvalidJson)?;

        Ok(ExportJson { original_file: file })
    }
}