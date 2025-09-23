use std::error::Error;

use crate::map::Map;
use crate::{file::FileStoredFormat, value::Value, Format};

#[cfg(feature = "toml")]
mod toml;

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "yaml")]
mod yaml;

#[cfg(feature = "ini")]
mod ini;

#[cfg(feature = "ron")]
mod ron;

#[cfg(feature = "json5")]
mod json5;

#[cfg(feature = "corn")]
mod corn;

/// File formats provided by the library.
///
/// Although it is possible to define custom formats using [`Format`] trait it is recommended to use `FileFormat` if possible.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum FileFormat {
    /// TOML (parsed with toml)
    #[cfg(feature = "toml")]
    Toml,

    /// JSON (parsed with `serde_json`)
    #[cfg(feature = "json")]
    Json,

    /// YAML (parsed with `yaml_rust2`)
    #[cfg(feature = "yaml")]
    Yaml,

    /// INI (parsed with `rust_ini`)
    #[cfg(feature = "ini")]
    Ini,

    /// RON (parsed with ron)
    #[cfg(feature = "ron")]
    Ron,

    /// JSON5 (parsed with json5)
    #[cfg(feature = "json5")]
    Json5,

    /// Corn (parsed with `libcorn`)
    #[cfg(feature = "corn")]
    Corn,
}

impl FileFormat {
    pub(crate) fn all() -> &'static [FileFormat] {
        &[
            #[cfg(feature = "toml")]
            FileFormat::Toml,
            #[cfg(feature = "json")]
            FileFormat::Json,
            #[cfg(feature = "yaml")]
            FileFormat::Yaml,
            #[cfg(feature = "ini")]
            FileFormat::Ini,
            #[cfg(feature = "ron")]
            FileFormat::Ron,
            #[cfg(feature = "json5")]
            FileFormat::Json5,
            #[cfg(feature = "corn")]
            FileFormat::Corn,
        ]
    }

    pub(crate) fn extensions(&self) -> &'static [&'static str] {
        match self {
            #[cfg(feature = "toml")]
            FileFormat::Toml => &["toml"],

            #[cfg(feature = "json")]
            FileFormat::Json => &["json"],

            #[cfg(feature = "yaml")]
            FileFormat::Yaml => &["yaml", "yml"],

            #[cfg(feature = "ini")]
            FileFormat::Ini => &["ini"],

            #[cfg(feature = "ron")]
            FileFormat::Ron => &["ron"],

            #[cfg(feature = "json5")]
            FileFormat::Json5 => &["json5"],

            #[cfg(feature = "corn")]
            FileFormat::Corn => &["corn"],

            #[cfg(all(
                not(feature = "toml"),
                not(feature = "json"),
                not(feature = "yaml"),
                not(feature = "ini"),
                not(feature = "ron"),
                not(feature = "json5"),
            ))]
            _ => unreachable!("No features are enabled, this library won't work without features"),
        }
    }

    pub(crate) fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
        match self {
            #[cfg(feature = "toml")]
            FileFormat::Toml => toml::parse(uri, text),

            #[cfg(feature = "json")]
            FileFormat::Json => json::parse(uri, text),

            #[cfg(feature = "yaml")]
            FileFormat::Yaml => yaml::parse(uri, text),

            #[cfg(feature = "ini")]
            FileFormat::Ini => ini::parse(uri, text),

            #[cfg(feature = "ron")]
            FileFormat::Ron => ron::parse(uri, text),

            #[cfg(feature = "json5")]
            FileFormat::Json5 => json5::parse(uri, text),

            #[cfg(feature = "corn")]
            FileFormat::Corn => corn::parse(uri, text),

            #[cfg(all(
                not(feature = "toml"),
                not(feature = "json"),
                not(feature = "yaml"),
                not(feature = "ini"),
                not(feature = "ron"),
                not(feature = "json5"),
            ))]
            _ => unreachable!("No features are enabled, this library won't work without features"),
        }
    }
}

impl Format for FileFormat {
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
        self.parse(uri, text)
    }
}

impl FileStoredFormat for FileFormat {
    fn file_extensions(&self) -> &'static [&'static str] {
        self.extensions()
    }
}
