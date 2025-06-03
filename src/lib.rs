//! [`Config`] organizes hierarchical or layered configurations for Rust applications.
//!
//! [`Config`] lets you set a set of [default parameters][ConfigBuilder::set_default] and then extend them via merging in
//! configuration from a variety of sources:
//!
//!  - [Environment variables][Environment]
//!  - [String literals][FileSourceString] in [well-known formats][FileFormat]
//!  - Another [`Config`] instance
//!  - [Files][FileSourceFile] in [well known formats][FileFormat] and custom ones defined with [`Format`] trait
//!  - Manual, programmatic [overrides][ConfigBuilder::set_override]
//!
//! Additionally, [`Config`] supports:
//!
//!  - Live watching and re-reading of configuration files
//!  - Deep access into the merged configuration via a path syntax
//!  - Deserialization via `serde` of the configuration or any subset defined via a path
//!
//! # Example
//!
//! ```rust
//! # #[cfg(feature = "toml")] {
#![doc = include_str!("../examples/simple/main.rs")]
//! # }
//! ```
//!
//! See more [examples](https://github.com/mehcode/config-rs/tree/master/examples) for
//! general usage information.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

pub mod builder;
mod config;
mod de;
mod env;
mod error;
mod file;
mod format;
mod map;
mod path;
mod ser;
mod source;
mod value;

// Re-export
#[cfg(feature = "convert-case")]
pub use convert_case::Case;

pub use crate::builder::ConfigBuilder;
pub use crate::config::Config;
pub use crate::env::Environment;
pub use crate::error::ConfigError;
pub use crate::file::source::FileSource;
pub use crate::file::{File, FileFormat, FileSourceFile, FileSourceString, FileStoredFormat};
pub use crate::format::Format;
pub use crate::map::Map;
#[cfg(feature = "async")]
pub use crate::source::AsyncSource;
pub use crate::source::Source;
pub use crate::value::{Value, ValueKind};

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
