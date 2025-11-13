// Author: Jacques Murray

//! # config-rs-ng
//!
//! A modern, flexible, and ergonomic configuration library for Rust,
//! built on top of `serde` and `validator`.
//!
//! This library provides a unified API for loading configuration from
//! multiple sources, merging them with a clear precedence, and
//! deserializing them into strongly-typed Rust structs.
//!
//! ## Quick Start
//!
//! See the API example in `examples/simple.rs` for a demonstration of
//! how to use the builder.

// Public modules
pub mod builder;
pub mod config;
pub mod error;
pub mod source;

// Public API exports
pub use builder::ConfigBuilder;
pub use config::Config;
pub use error::Error;
