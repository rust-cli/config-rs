use std::env;

use crate::error::Result;
use crate::map::Map;
use crate::source::Source;
use crate::value::{Value, ValueKind};
use crate::ConfigError;

#[cfg(feature = "convert-case")]
use convert_case::{Case, Casing};

/// An environment source collects a dictionary of environment variables values into a hierarchical
/// config Value type. We have to be aware how the config tree is created from the environment
/// dictionary, therefore we are mindful about prefixes for the environment keys, level separators,
/// encoding form (kebab, snake case) etc.
///
/// Environment variables are specified using the path to the key they correspond to. When
/// specifying a value within a list, the index should be specified in the key immediately after
/// the list type.
///
/// ## Example
///
/// ```rust
/// # use config::{Environment, Config};
/// # use serde::Deserialize;
/// # use std::collections::HashMap;
/// # use std::convert::TryInto;
/// #
/// # #[test]
/// # fn test_env_list_of_structs() {
///     #[derive(Deserialize, Debug, PartialEq)]
///     struct Struct {
///         a: Vec<Option<u32>>,
///         b: u32,
///     }
///
///     #[derive(Deserialize, Debug)]
///     struct ListOfStructs {
///         list: Vec<Struct>,
///     }
///
///     let values = vec![
///         ("LIST_0_A_0".to_owned(), "1".to_owned()),
///         ("LIST_0_A_2".to_owned(), "2".to_owned()),
///         ("LIST_0_B".to_owned(), "3".to_owned()),
///         ("LIST_1_A_1".to_owned(), "4".to_owned()),
///         ("LIST_1_B".to_owned(), "5".to_owned()),
///     ];
///
///     let environment = Environment::default()
///         .separator("_")
///         .try_parsing(true)
///         .source(Some(values.into_iter().collect()));
///
///     let config = Config::builder().add_source(environment).build().unwrap();
///
///     let config: ListOfStructs = config.try_deserialize().unwrap();
///     assert_eq!(
///         config.list,
///         vec![
///             Struct {
///                 a: vec![Some(1), None, Some(2)],
///                 b: 3
///             },
///             Struct {
///                 a: vec![None, Some(4)],
///                 b: 5
///             },
///         ]
///     );
/// # }
/// ```
#[must_use]
#[derive(Clone, Debug, Default)]
pub struct Environment {
    /// Optional prefix that will limit access to the environment to only keys that
    /// begin with the defined prefix.
    ///
    /// A prefix with a separator of `_` is tested to be present on each key before its considered
    /// to be part of the source environment.
    ///
    /// For example, the key `CONFIG_DEBUG` would become `DEBUG` with a prefix of `config`.
    prefix: Option<String>,

    /// Optional character sequence that separates the prefix from the rest of the key
    prefix_separator: Option<String>,

    /// Optional character sequence that separates each key segment in an environment key pattern.
    /// Consider a nested configuration such as `redis.password`, a separator of `_` would allow
    /// an environment key of `REDIS_PASSWORD` to match.
    separator: Option<String>,

    /// Optional directive to translate collected keys into a form that matches what serializers
    /// that the configuration would expect. For example if you have the `kebab-case` attribute
    /// for your serde config types, you may want to pass `Case::Kebab` here.
    #[cfg(feature = "convert-case")]
    convert_case: Option<Case>,

    /// Optional character sequence that separates each env value into a vector. only works when `try_parsing` is set to true
    /// Once set, you cannot have type String on the same environment, unless you set `list_parse_keys`.
    list_separator: Option<String>,
    /// A list of keys which should always be parsed as a list. If not set you can have only `Vec<String>` or `String` (not both) in one environment.
    list_parse_keys: Option<Vec<String>>,

    /// Ignore empty env values (treat as unset).
    ignore_empty: bool,

    /// Parses booleans, integers and floats if they're detected (can be safely parsed).
    try_parsing: bool,

    // Preserve the prefix while parsing
    keep_prefix: bool,

    /// Alternate source for the environment. This can be used when you want to test your own code
    /// using this source, without the need to change the actual system environment variables.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use config::{Environment, Config};
    /// # use serde::Deserialize;
    /// # use std::collections::HashMap;
    /// # use std::convert::TryInto;
    /// #
    /// #[test]
    /// fn test_config() -> Result<(), config::ConfigError> {
    ///   #[derive(Clone, Debug, Deserialize)]
    ///   struct MyConfig {
    ///     pub my_string: String,
    ///   }
    ///
    ///   let source = Environment::default()
    ///     .source(Some({
    ///       let mut env = HashMap::new();
    ///       env.insert("MY_STRING".into(), "my-value".into());
    ///       env
    ///   }));
    ///
    ///   let config: MyConfig = Config::builder()
    ///     .add_source(source)
    ///     .build()?
    ///     .try_into()?;
    ///   assert_eq!(config.my_string, "my-value");
    ///
    ///   Ok(())
    /// }
    /// ```
    source: Option<Map<String, String>>,
}

impl Environment {
    #[deprecated(since = "0.12.0", note = "please use 'Environment::default' instead")]
    pub fn new() -> Self {
        Self::default()
    }

    /// Optional prefix that will limit access to the environment to only keys that
    /// begin with the defined prefix.
    ///
    /// A prefix with a separator of `_` is tested to be present on each key before its considered
    /// to be part of the source environment.
    ///
    /// For example, the key `CONFIG_DEBUG` would become `DEBUG` with a prefix of `config`.
    pub fn with_prefix(s: &str) -> Self {
        Self {
            prefix: Some(s.into()),
            ..Self::default()
        }
    }

    /// See [`Environment::with_prefix`]
    pub fn prefix(mut self, s: &str) -> Self {
        self.prefix = Some(s.into());
        self
    }

    #[cfg(feature = "convert-case")]
    pub fn with_convert_case(tt: Case) -> Self {
        Self::default().convert_case(tt)
    }

    #[cfg(feature = "convert-case")]
    pub fn convert_case(mut self, tt: Case) -> Self {
        self.convert_case = Some(tt);
        self
    }

    /// Optional character sequence that separates the prefix from the rest of the key
    pub fn prefix_separator(mut self, s: &str) -> Self {
        self.prefix_separator = Some(s.into());
        self
    }

    /// Optional character sequence that separates each key segment in an environment key pattern.
    /// Consider a nested configuration such as `redis.password`, a separator of `_` would allow
    /// an environment key of `REDIS_PASSWORD` to match.
    pub fn separator(mut self, s: &str) -> Self {
        self.separator = Some(s.into());
        self
    }

    /// When set and `try_parsing` is true, then all environment variables will be parsed as [`Vec<String>`] instead of [`String`].
    /// See
    /// [`with_list_parse_key`](Self::with_list_parse_key)
    /// when you want to use [`Vec<String>`] in combination with [`String`].
    pub fn list_separator(mut self, s: &str) -> Self {
        self.list_separator = Some(s.into());
        self
    }

    /// Add a key which should be parsed as a list when collecting [`Value`]s from the environment.
    /// Once `list_separator` is set, the type for string is [`Vec<String>`].
    /// To switch the default type back to type Strings you need to provide the keys which should be [`Vec<String>`] using this function.
    pub fn with_list_parse_key(mut self, key: &str) -> Self {
        if self.list_parse_keys.is_none() {
            self.list_parse_keys = Some(vec![key.to_lowercase()]);
        } else {
            self.list_parse_keys = self.list_parse_keys.map(|mut keys| {
                keys.push(key.to_lowercase());
                keys
            });
        }
        self
    }

    /// Ignore empty env values (treat as unset).
    pub fn ignore_empty(mut self, ignore: bool) -> Self {
        self.ignore_empty = ignore;
        self
    }

    /// Note: enabling `try_parsing` can reduce performance it will try and parse
    /// each environment variable 3 times (bool, i64, f64)
    pub fn try_parsing(mut self, try_parsing: bool) -> Self {
        self.try_parsing = try_parsing;
        self
    }

    // Preserve the prefix while parsing
    pub fn keep_prefix(mut self, keep: bool) -> Self {
        self.keep_prefix = keep;
        self
    }

    /// Alternate source for the environment. This can be used when you want to test your own code
    /// using this source, without the need to change the actual system environment variables.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use config::{Environment, Config};
    /// # use serde::Deserialize;
    /// # use std::collections::HashMap;
    /// # use std::convert::TryInto;
    /// #
    /// #[test]
    /// fn test_config() -> Result<(), config::ConfigError> {
    ///   #[derive(Clone, Debug, Deserialize)]
    ///   struct MyConfig {
    ///     pub my_string: String,
    ///   }
    ///
    ///   let source = Environment::default()
    ///     .source(Some({
    ///       let mut env = HashMap::new();
    ///       env.insert("MY_STRING".into(), "my-value".into());
    ///       env
    ///   }));
    ///
    ///   let config: MyConfig = Config::builder()
    ///     .add_source(source)
    ///     .build()?
    ///     .try_into()?;
    ///   assert_eq!(config.my_string, "my-value");
    ///
    ///   Ok(())
    /// }
    /// ```
    pub fn source(mut self, source: Option<Map<String, String>>) -> Self {
        self.source = source;
        self
    }
}

impl Source for Environment {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<Map<String, Value>> {
        let uri: String = "the environment".into();
        let mut m = Value::new(Some(&uri), ValueKind::Table(Map::new()));

        let separator = self.separator.as_deref().unwrap_or("");
        #[cfg(feature = "convert-case")]
        let convert_case = &self.convert_case;
        let prefix_separator = match (self.prefix_separator.as_deref(), self.separator.as_deref()) {
            (Some(pre), _) => pre,
            (None, Some(sep)) => sep,
            (None, None) => "_",
        };

        // Define a prefix pattern to test and exclude from keys
        let prefix_pattern = self
            .prefix
            .as_ref()
            .map(|prefix| format!("{prefix}{prefix_separator}").to_lowercase());

        let collector = |(key, value): (String, String)| -> Result<()> {
            // Treat empty environment variables as unset
            if self.ignore_empty && value.is_empty() {
                return Ok(());
            }

            let mut key = key.to_lowercase();

            // Check for prefix
            let mut kept_prefix = String::new();
            if let Some(ref prefix_pattern) = prefix_pattern {
                if key.starts_with(prefix_pattern) {
                    key = key[prefix_pattern.len()..].to_string();
                    if self.keep_prefix {
                        kept_prefix = prefix_pattern.clone();
                    }
                } else {
                    // Skip this key
                    return Ok(());
                }
            }

            // If separator is given replace with `.`
            if !separator.is_empty() {
                key = key.replace(separator, ".");
            }

            #[cfg(feature = "convert-case")]
            if let Some(convert_case) = convert_case {
                key = key.to_case(*convert_case);
                kept_prefix = kept_prefix.to_case(*convert_case);
            }

            let value_kind = if self.try_parsing {
                // convert to lowercase because bool parsing expects all lowercase
                if let Ok(parsed) = value.to_lowercase().parse::<bool>() {
                    ValueKind::Boolean(parsed)
                } else if let Ok(parsed) = value.parse::<i64>() {
                    ValueKind::I64(parsed)
                } else if let Ok(parsed) = value.parse::<f64>() {
                    ValueKind::Float(parsed)
                } else if let Some(separator) = &self.list_separator {
                    if let Some(parse_keys) = &self.list_parse_keys {
                        #[cfg(feature = "convert-case")]
                        let key = key.to_lowercase();
                        if parse_keys.contains(&key) {
                            let v: Vec<Value> = value
                                .split(separator)
                                .map(|s| Value::new(Some(&uri), ValueKind::String(s.to_owned())))
                                .collect();
                            ValueKind::Array(v)
                        } else {
                            ValueKind::String(value)
                        }
                    } else {
                        let v: Vec<Value> = value
                            .split(separator)
                            .map(|s| Value::new(Some(&uri), ValueKind::String(s.to_owned())))
                            .collect();
                        ValueKind::Array(v)
                    }
                } else {
                    ValueKind::String(value)
                }
            } else {
                ValueKind::String(value)
            };

            let value = Value::new(Some(&uri), value_kind);
            let mut key_parts = key.split('.');
            let key = key_parts
                .next()
                .ok_or(ConfigError::Message("No key after prefix".to_owned()))?;

            let value = key_parts
                .rev()
                .fold(value, |value, key| match key.parse::<usize>() {
                    Ok(index) => {
                        let nil = Value::new(Some(&uri), ValueKind::Nil);
                        let mut array = vec![nil; index + 1];
                        array[index] = value;
                        Value::new(Some(&uri), ValueKind::Array(array))
                    }
                    Err(_) => {
                        let mut map = Map::new();
                        map.insert(key.to_owned(), value);
                        Value::new(Some(&uri), ValueKind::Table(map))
                    }
                });

            let mut map = Map::new();
            kept_prefix.push_str(key);
            map.insert(kept_prefix, value);
            let table = Value::new(Some(&uri), ValueKind::Table(map));

            m.merge(table);
            Ok(())
        };

        match &self.source {
            Some(source) => source.clone().into_iter().try_for_each(collector)?,
            None => env::vars().try_for_each(collector)?,
        }

        match m.kind {
            ValueKind::Table(m) => Ok(m),
            _ => unreachable!(),
        }
    }
}
