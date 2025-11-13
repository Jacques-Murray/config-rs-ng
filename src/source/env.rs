// Author: Jacques Murray

use super::Source;
use crate::Error;
use serde_json::{Map, Value};
use std::env;

/// A configuration source that reads from environment variables.
///
/// Supports a prefix and a delimiter for nested values.
/// For example, with prefix `APP_` and delimiter `_`:
/// `APP_DATABASE_HOST` becomes `{"database":{"host":"..."}}`
pub struct Env {
    prefix: String,
    delimiter: String,
}

impl Env {
    /// Creates a new `Env` source with default settings.
    pub fn new() -> Self {
        Self {
            prefix: "".to_string(),
            delimiter: "__".to_string(), // Default to double-underscore
        }
    }

    /// Sets the prefix for environment variables.
    /// Only variables starting with this prefix will be considered.
    /// The prefix is stripped from the key.
    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = prefix.to_string();
        self
    }

    /// Sets the delimiter for nested keys.
    /// For example, `DATABASE_HOST` with delimiter `_` becomes `{"database":{"host":"..."}}`.
    pub fn delimiter(mut self, delimiter: &str) -> Self {
        self.delimiter = delimiter.to_string();
        self
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

impl Source for Env {
    fn collect(&self) -> Result<Value, Error> {
        log::debug!(
            "Collecting configuration from environment with prefix '{}'",
            self.prefix
        );
        let mut root = Value::Object(Map::new());

        for (key, value) in env::vars() {
            if !key.starts_with(&self.prefix) {
                continue;
            }

            // Strip prefix and convert to lowercase
            let key_without_prefix = key.trim_start_matches(&self.prefix).to_lowercase();

            // Split by delimiter to create nested structure
            let parts: Vec<&str> = key_without_prefix.split(&self.delimiter).collect();

            if parts.is_empty() {
                continue;
            }

            // Build nested Value
            let mut current_map = &mut root;
            for (i, part) in parts.iter().enumerate() {
                if i == parts.len() - 1 {
                    // Last part, insert the value
                    if let Some(obj) = current_map.as_object_mut() {
                        obj.insert(part.to_string(), Value::String(value.clone()));
                    }
                } else {
                    // Not the last part, ensure nested object exists
                    if let Some(obj) = current_map.as_object_mut() {
                        current_map = obj
                            .entry(part.to_string())
                            .or_insert_with(|| Value::Object(Map::new()));
                    }
                }
            }
        }
        Ok(root)
    }
}
