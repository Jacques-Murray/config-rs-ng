// Author: Jacques Murray

use super::Source;
use crate::Error;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

/// A configuration source that reads from a file (e.g, TOML, YAML, JSON).
///
/// The format is automatically detected from the file extension.
pub struct File {
    path: PathBuf,
    // In a future enhancement, we could add:
    // format: Option<Format>
}

impl File {
    /// Creates a new `File` source from a path.
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl Source for File {
    fn collect(&self) -> Result<Value, Error> {
        let path_str = self.path.to_string_lossy().to_string();
        log::debug!("Collecting configuration from file: {}", path_str);

        // Read file contents
        let contents = fs::read_to_string(&self.path)?;

        // Detect format from extension
        let ext = self
            .path
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| Error::UnknownFileFormat(path_str.clone()))?;

        match ext {
            "toml" => {
                let value: Value = toml::from_str(&contents)?;
                Ok(value)
            }
            "yaml" | "yml" => {
                let value: Value = serde_yaml::from_str(&contents)?;
                Ok(value)
            }
            "json" => {
                let value: Value = serde_json::from_str(&contents)?;
                Ok(value)
            }
            _ => Err(Error::UnknownFileFormat(path_str)),
        }
    }
}
