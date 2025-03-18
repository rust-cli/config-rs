use std::fmt::Debug;

use serde::de::Deserialize;
use serde::ser::Serialize;

use crate::builder::{ConfigBuilder, DefaultState};
use crate::error::{ConfigError, Result};
use crate::map::Map;
use crate::path;
use crate::ser::ConfigSerializer;
use crate::source::Source;
use crate::value::{Table, Value};

/// A prioritized configuration repository.
///
/// It maintains a set of configuration sources, fetches values to populate those, and provides
/// them according to the source's priority.
#[derive(Clone, Debug)]
pub struct Config {
    defaults: Map<path::Expression, Value>,
    overrides: Map<path::Expression, Value>,
    sources: Vec<Box<dyn Source + Send + Sync>>,

    /// Root of the cached configuration.
    pub cache: Value,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            defaults: Default::default(),
            overrides: Default::default(),
            sources: Default::default(),
            cache: Value::new(None, Table::new()),
        }
    }
}

impl Config {
    pub(crate) fn new(value: Value) -> Self {
        Self {
            cache: value,
            ..Self::default()
        }
    }

    /// Creates new [`ConfigBuilder`] instance
    pub fn builder() -> ConfigBuilder<DefaultState> {
        ConfigBuilder::<DefaultState>::default()
    }

    /// Refresh the configuration cache with fresh
    /// data from added sources.
    ///
    /// Configuration is automatically refreshed after a mutation
    /// operation (`set`, `merge`, `set_default`, etc.).
    fn refresh(&mut self) -> Result<&mut Self> {
        self.cache = {
            let mut cache: Value = Map::<String, Value>::new().into();

            // Add defaults
            for (key, val) in &self.defaults {
                key.set(&mut cache, val.clone());
            }

            // Add sources
            self.sources.collect_to(&mut cache)?;

            // Add overrides
            for (key, val) in &self.overrides {
                key.set(&mut cache, val.clone());
            }

            cache
        };

        Ok(self)
    }

    /// Set an overwrite
    ///
    /// This function sets an overwrite value.
    /// The overwrite `value` is written to the `key` location on every `refresh()`
    ///
    /// # Warning
    ///
    /// Errors if config is frozen
    pub(crate) fn set<T>(&mut self, key: &str, value: T) -> Result<&mut Self>
    where
        T: Into<Value>,
    {
        self.overrides.insert(key.parse()?, value.into());

        self.refresh()
    }

    fn get_value(&self, key: &str) -> Result<Value> {
        // Parse the key into a path expression
        let expr: path::Expression = key.parse()?;

        // Traverse the cache using the path to (possibly) retrieve a value
        let value = expr.get(&self.cache).cloned();

        value.ok_or_else(|| ConfigError::NotFound(key.into()))
    }

    pub fn get<'de, T: Deserialize<'de>>(&self, key: &str) -> Result<T> {
        self.get_value(key).and_then(|value| {
            // Deserialize the received value into the requested type
            T::deserialize(value).map_err(|e| e.extend_with_key(key))
        })
    }

    pub fn get_string(&self, key: &str) -> Result<String> {
        self.get_value(key)
            .and_then(|value| value.into_string().map_err(|e| e.extend_with_key(key)))
    }

    pub fn get_int(&self, key: &str) -> Result<i64> {
        self.get_value(key)
            .and_then(|value| value.into_int().map_err(|e| e.extend_with_key(key)))
    }

    pub fn get_float(&self, key: &str) -> Result<f64> {
        self.get_value(key)
            .and_then(|value| value.into_float().map_err(|e| e.extend_with_key(key)))
    }

    pub fn get_bool(&self, key: &str) -> Result<bool> {
        self.get_value(key)
            .and_then(|value| value.into_bool().map_err(|e| e.extend_with_key(key)))
    }

    pub fn get_table(&self, key: &str) -> Result<Map<String, Value>> {
        self.get_value(key)
            .and_then(|value| value.into_table().map_err(|e| e.extend_with_key(key)))
    }

    pub fn get_array(&self, key: &str) -> Result<Vec<Value>> {
        self.get_value(key)
            .and_then(|value| value.into_array().map_err(|e| e.extend_with_key(key)))
    }

    /// Attempt to deserialize the entire configuration into the requested type.
    pub fn try_deserialize<'de, T: Deserialize<'de>>(self) -> Result<T> {
        T::deserialize(self)
    }

    /// Attempt to serialize the entire configuration from the given type.
    pub fn try_from<T: Serialize>(from: &T) -> Result<Self> {
        let mut serializer = ConfigSerializer::default();
        from.serialize(&mut serializer)?;
        Ok(serializer.output)
    }
}

impl Source for Config {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<Map<String, Value>> {
        self.cache.clone().into_table()
    }
}
