use std::error::Error;

use crate::error::{ConfigError, Unexpected};
use crate::map::Map;
use crate::value::{Value, ValueKind};

/// Describes a format of configuration source data
///
/// Implementations of this trait can be used to convert [`File`](crate::File) sources to configuration data.
///
/// There can be various formats, some of them provided by this library, such as JSON, Yaml and other.
/// This trait enables users of the library to easily define their own, even proprietary formats without
/// the need to alter library sources.
///
/// What is more, it is recommended to use this trait with custom [`Source`](crate::Source)s and their async counterparts.
pub trait Format {
    /// Parses provided content into configuration values understood by the library.
    ///
    /// It also allows specifying optional URI of the source associated with format instance that can facilitate debugging.
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>>;
}

// Have a proper error fire if the root of a file is ever not a Table
pub(crate) fn extract_root_table(
    uri: Option<&String>,
    value: Value,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    match value.kind {
        ValueKind::Table(map) => Ok(map),
        ValueKind::Nil => Err(Unexpected::Unit),
        ValueKind::Array(_value) => Err(Unexpected::Seq),
        ValueKind::Boolean(value) => Err(Unexpected::Bool(value)),
        ValueKind::I64(value) => Err(Unexpected::I64(value)),
        ValueKind::I128(value) => Err(Unexpected::I128(value)),
        ValueKind::U64(value) => Err(Unexpected::U64(value)),
        ValueKind::U128(value) => Err(Unexpected::U128(value)),
        ValueKind::Float(value) => Err(Unexpected::Float(value)),
        ValueKind::String(value) => Err(Unexpected::Str(value)),
    }
    .map_err(|err| ConfigError::invalid_root(uri, err))
    .map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>)
}
