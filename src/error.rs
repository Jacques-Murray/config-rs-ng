// Author: Jacques Murray

use thiserror::Error;

/// Enumerates all possible errors for the configuration library.
#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error for path: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON parsing/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Validation error(s): {0}")]
    Validation(#[from] validator::ValidationErrors),

    #[error("Source collection error: {0}")]
    Source(String),

    #[error("Failed to determine file format for: {0}")]
    UnknownFileFormat(String),

    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("Could not build config: {0}")]
    Build(String),
}
