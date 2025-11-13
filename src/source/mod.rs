// Author: Jacques Murray

use crate::Error;
use serde_json::Value;

mod env;
mod file;

pub use env::Env;
pub use file::File;

/// A trait for any struct that can provide configuration values.
///
/// Implement this trait to add support for custom configuration sources,
/// such as remote HTTP endpoints, databases, or cloud-specific services.
///
/// The `collect` method should return a `serde_json::Value`, which is
/// a generic, map-like structure that can be easily and deeply merged.
pub trait Source {
    /// Collects configuration values from the source.
    ///
    /// # Returns
    /// A `Result` containing a `serde_json::Value` on success,
    /// or an `Error` on failure.
    fn collect(&self) -> Result<Value, Error>;
}
