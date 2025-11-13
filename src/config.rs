// Author: Jacques Murray

use crate::{builder::ConfigBuilder, Error};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// The main configuration object.
///
/// This struct holds the final, merged configuration `Value` from all
/// sources. It is created by the `ConfigBuilder`.
///
/// Use the `try_deserialize` method to parse this into your
/// strongly-typed configuration struct.
#[derive(Debug)]
pub struct Config {
    /// The merged configuration value.
    value: Value,
}

impl Config {
    /// Creates a new `ConfigBuilder` to start building the configuration.
    /// This is the main entry point for the library.
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Creates a new `Config` directly from a `serde_json::Value`.
    /// Used by the `ConfigBuilder`.
    pub(crate) fn new(value: Value) -> Self {
        Self { value }
    }

    /// Deserializes the merged configuration into a strongly-typed
    /// struct `T`.
    ///
    /// `T` must implement `serde::de::DeserializeOwned`.
    ///
    /// # Returns
    /// A `Result` containing the populated struct `T` on success,
    /// or an `Error` if deserialization fails.
    pub fn try_deserialize<T>(self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_value(self.value).map_err(Error::from)
    }

    /// Gets a reference to the inner `serde_json::Value`.
    pub fn get_value(&self) -> &Value {
        &self.value
    }
}
