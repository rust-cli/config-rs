use std::error::Error;

use crate::map::Map;
use crate::{Format, file::FileStoredFormat, value::Value};

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
    pub(crate) fn all() -> &'static [Self] {
        &[
            #[cfg(feature = "toml")]
            Self::Toml,
            #[cfg(feature = "json")]
            Self::Json,
            #[cfg(feature = "yaml")]
            Self::Yaml,
            #[cfg(feature = "ini")]
            Self::Ini,
            #[cfg(feature = "ron")]
            Self::Ron,
            #[cfg(feature = "json5")]
            Self::Json5,
            #[cfg(feature = "corn")]
            Self::Corn,
        ]
    }

    pub(crate) fn extensions(&self) -> &'static [&'static str] {
        match self {
            #[cfg(feature = "toml")]
            Self::Toml => &["toml"],

            #[cfg(feature = "json")]
            Self::Json => &["json"],

            #[cfg(feature = "yaml")]
            Self::Yaml => &["yaml", "yml"],

            #[cfg(feature = "ini")]
            Self::Ini => &["ini"],

            #[cfg(feature = "ron")]
            Self::Ron => &["ron"],

            #[cfg(feature = "json5")]
            Self::Json5 => &["json5"],

            #[cfg(feature = "corn")]
            Self::Corn => &["corn"],

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
            Self::Toml => toml::parse(uri, text),

            #[cfg(feature = "json")]
            Self::Json => json::parse(uri, text),

            #[cfg(feature = "yaml")]
            Self::Yaml => yaml::parse(uri, text),

            #[cfg(feature = "ini")]
            Self::Ini => ini::parse(uri, text),

            #[cfg(feature = "ron")]
            Self::Ron => ron::parse(uri, text),

            #[cfg(feature = "json5")]
            Self::Json5 => json5::parse(uri, text),

            #[cfg(feature = "corn")]
            Self::Corn => corn::parse(uri, text),

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
