use std::error::Error;
use std::fmt;
use std::result;

use serde::de;
use serde::ser;

#[allow(unnameable_types)] // Unsure if/how to expose this
#[derive(Debug)]
pub enum Unexpected {
    Bool(bool),
    I64(i64),
    I128(i128),
    U64(u64),
    U128(u128),
    Float(f64),
    Str(String),
    Unit,
    Seq,
    Map,
}

impl fmt::Display for Unexpected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        match *self {
            Unexpected::Bool(b) => write!(f, "boolean `{b}`"),
            Unexpected::I64(i) => write!(f, "64-bit integer `{i}`"),
            Unexpected::I128(i) => write!(f, "128-bit integer `{i}`"),
            Unexpected::U64(i) => write!(f, "64-bit unsigned integer `{i}`"),
            Unexpected::U128(i) => write!(f, "128-bit unsigned integer `{i}`"),
            Unexpected::Float(v) => write!(f, "floating point `{v}`"),
            Unexpected::Str(ref s) => write!(f, "string {s:?}"),
            Unexpected::Unit => write!(f, "unit value"),
            Unexpected::Seq => write!(f, "sequence"),
            Unexpected::Map => write!(f, "map"),
        }
    }
}

/// Represents all possible errors that can occur when working with
/// configuration.
#[non_exhaustive]
pub enum ConfigError {
    /// Configuration is frozen and no further mutations can be made.
    Frozen,

    /// Configuration property was not found
    NotFound(String),

    /// Configuration path could not be parsed.
    PathParse { cause: Box<dyn Error + Send + Sync> },

    /// Configuration could not be parsed from file.
    FileParse {
        /// The URI used to access the file (if not loaded from a string).
        /// Example: `/path/to/config.json`
        uri: Option<String>,

        /// The captured error from attempting to parse the file in its desired format.
        /// This is the actual error object from the library used for the parsing.
        cause: Box<dyn Error + Send + Sync>,
    },

    /// Value could not be converted into the requested type.
    Type {
        /// The URI that references the source that the value came from.
        /// Example: `/path/to/config.json` or `Environment` or `etcd://localhost`
        // TODO: Why is this called Origin but FileParse has a uri field?
        origin: Option<String>,

        /// What we found when parsing the value
        unexpected: Unexpected,

        /// What was expected when parsing the value
        expected: &'static str,

        /// The key in the configuration hash of this value (if available where the
        /// error is generated).
        key: Option<String>,
    },

    /// Custom message
    At {
        /// Error being extended with a path
        error: Box<ConfigError>,

        /// The URI that references the source that the value came from.
        /// Example: `/path/to/config.json` or `Environment` or `etcd://localhost`
        // TODO: Why is this called Origin but FileParse has a uri field?
        origin: Option<String>,

        /// The key in the configuration hash of this value (if available where the
        /// error is generated).
        key: Option<String>,
    },

    /// Custom message
    Message(String),

    /// Unadorned error from a foreign origin.
    Foreign(Box<dyn Error + Send + Sync>),
}

impl ConfigError {
    // FIXME: pub(crate)
    #[doc(hidden)]
    pub fn invalid_type(
        origin: Option<String>,
        unexpected: Unexpected,
        expected: &'static str,
    ) -> Self {
        Self::Type {
            origin,
            unexpected,
            expected,
            key: None,
        }
    }

    // Have a proper error fire if the root of a file is ever not a Table
    // TODO: for now only json5 checked, need to finish others
    #[doc(hidden)]
    pub fn invalid_root(origin: Option<&String>, unexpected: Unexpected) -> Box<Self> {
        Box::new(Self::Type {
            origin: origin.cloned(),
            unexpected,
            expected: "a map",
            key: None,
        })
    }

    // FIXME: pub(crate)
    #[doc(hidden)]
    #[must_use]
    pub fn extend_with_key(self, key: &str) -> Self {
        match self {
            Self::Type {
                origin,
                unexpected,
                expected,
                ..
            } => Self::Type {
                origin,
                unexpected,
                expected,
                key: Some(key.into()),
            },

            Self::At { origin, error, .. } => Self::At {
                error,
                origin,
                key: Some(key.into()),
            },

            other => Self::At {
                error: Box::new(other),
                origin: None,
                key: Some(key.into()),
            },
        }
    }

    #[must_use]
    fn prepend(self, segment: &str, add_dot: bool) -> Self {
        let concat = |key: Option<String>| {
            let key = key.unwrap_or_default();
            let dot = if add_dot && key.as_bytes().first().unwrap_or(&b'[') != &b'[' {
                "."
            } else {
                ""
            };
            format!("{segment}{dot}{key}")
        };
        match self {
            Self::Type {
                origin,
                unexpected,
                expected,
                key,
            } => Self::Type {
                origin,
                unexpected,
                expected,
                key: Some(concat(key)),
            },
            Self::At { error, origin, key } => Self::At {
                error,
                origin,
                key: Some(concat(key)),
            },
            Self::NotFound(key) => Self::NotFound(concat(Some(key))),
            other => Self::At {
                error: Box::new(other),
                origin: None,
                key: Some(concat(None)),
            },
        }
    }

    #[must_use]
    pub(crate) fn prepend_key(self, key: &str) -> Self {
        self.prepend(key, true)
    }

    #[must_use]
    pub(crate) fn prepend_index(self, idx: usize) -> Self {
        self.prepend(&format!("[{idx}]"), false)
    }
}

/// Alias for a `Result` with the error type set to `ConfigError`.
pub(crate) type Result<T, E = ConfigError> = result::Result<T, E>;

// Forward Debug to Display for readable panic! messages
impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self)
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ConfigError::Frozen => write!(f, "configuration is frozen"),

            ConfigError::PathParse { ref cause } => write!(f, "{cause}"),

            ConfigError::Message(ref s) => write!(f, "{s}"),

            ConfigError::Foreign(ref cause) => write!(f, "{cause}"),

            ConfigError::NotFound(ref key) => {
                write!(f, "configuration property {key:?} not found")
            }

            ConfigError::Type {
                ref origin,
                ref unexpected,
                expected,
                ref key,
            } => {
                write!(f, "invalid type: {unexpected}, expected {expected}")?;

                if let Some(ref key) = *key {
                    write!(f, " for key `{key}`")?;
                }

                if let Some(ref origin) = *origin {
                    write!(f, " in {origin}")?;
                }

                Ok(())
            }

            ConfigError::At {
                ref error,
                ref origin,
                ref key,
            } => {
                write!(f, "{error}")?;

                if let Some(ref key) = *key {
                    write!(f, " for key `{key}`")?;
                }

                if let Some(ref origin) = *origin {
                    write!(f, " in {origin}")?;
                }

                Ok(())
            }

            ConfigError::FileParse { ref cause, ref uri } => {
                write!(f, "{cause}")?;

                if let Some(ref uri) = *uri {
                    write!(f, " in {uri}")?;
                }

                Ok(())
            }
        }
    }
}

impl Error for ConfigError {}

impl de::Error for ConfigError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}

impl ser::Error for ConfigError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}
