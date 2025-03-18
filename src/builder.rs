use std::str::FromStr;

use crate::error::Result;
use crate::map::Map;
#[cfg(feature = "async")]
use crate::source::AsyncSource;
use crate::{config::Config, path::Expression, source::Source, value::Value};

/// A configuration builder
///
/// It registers ordered sources of configuration to later build consistent [`Config`] from them.
/// Configuration sources it defines are defaults, [`Source`]s and overrides.
///
/// Defaults are always loaded first and can be overwritten by any of two other sources.
/// Overrides are always loaded last, thus cannot be overridden.
/// Both can be only set explicitly key by key in code
/// using [`set_default`](Self::set_default) or [`set_override`](Self::set_override).
///
/// An intermediate category, [`Source`], set groups of keys at once implicitly using data coming from external sources
/// like files, environment variables or others that one implements. Defining a [`Source`] is as simple as implementing
/// a trait for a struct.
///
/// Adding sources, setting defaults and overrides does not invoke any I/O nor builds a config.
/// It happens on demand when [`build`](Self::build) (or its alternative) is called.
/// Therefore all errors, related to any of the [`Source`] will only show up then.
///
/// # Sync and async builder
///
/// [`ConfigBuilder`] uses type parameter to keep track of builder state.
///
/// In [`DefaultState`] builder only supports [`Source`]s
///
/// In [`AsyncState`] it supports both [`Source`]s and [`AsyncSource`]s at the price of building using `async fn`.
///
/// # Examples
///
/// ```rust
/// # use config::*;
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// # #[cfg(feature = "json")]
/// # {
/// let mut builder = Config::builder()
///     .set_default("default", "1")?
///     .add_source(File::new("config/settings", FileFormat::Json))
/// //  .add_async_source(...)
///     .set_override("override", "1")?;
///
/// match builder.build() {
///     Ok(config) => {
///         // use your config
///     },
///     Err(e) => {
///         // something went wrong
///     }
/// }
/// # }
/// # Ok(())
/// # }
/// ```
///
/// If any [`AsyncSource`] is used, the builder will transition to [`AsyncState`].
/// In such case, it is required to _await_ calls to [`build`](Self::build) and its non-consuming sibling.
///
/// Calls can be not chained as well
/// ```rust
/// # use std::error::Error;
/// # use config::*;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// # #[cfg(feature = "json")]
/// # {
/// let mut builder = Config::builder();
/// builder = builder.set_default("default", "1")?;
/// builder = builder.add_source(File::new("config/settings", FileFormat::Json));
/// builder = builder.add_source(File::new("config/settings.prod", FileFormat::Json));
/// builder = builder.set_override("override", "1")?;
/// # }
/// # Ok(())
/// # }
/// ```
///
/// Calling [`Config::builder`](Config::builder) yields builder in the default state.
/// If having an asynchronous state as the initial state is desired, _turbofish_ notation needs to be used.
/// ```rust
/// # use config::{*, builder::AsyncState};
/// let mut builder = ConfigBuilder::<AsyncState>::default();
/// ```
///
/// If for some reason acquiring builder in default state is required without calling [`Config::builder`](Config::builder)
/// it can also be achieved.
/// ```rust
/// # use config::{*, builder::DefaultState};
/// let mut builder = ConfigBuilder::<DefaultState>::default();
/// ```
#[derive(Debug, Clone, Default)]
#[must_use]
pub struct ConfigBuilder<St: BuilderState> {
    defaults: Map<Expression, Value>,
    overrides: Map<Expression, Value>,
    state: St,
}

/// Represents [`ConfigBuilder`] state.
pub trait BuilderState {}

/// Represents data specific to builder in default, synchronous state, without support for async.
#[derive(Debug, Default, Clone)]
pub struct DefaultState {
    sources: Vec<Box<dyn Source + Send + Sync>>,
}

/// Represents data specific to builder in asynchronous state, with support for async.
#[derive(Debug, Default, Clone)]
pub struct AsyncState {
    sources: Vec<SourceType>,
}

#[derive(Debug, Clone)]
enum SourceType {
    Sync(Box<dyn Source + Send + Sync>),
    #[cfg(feature = "async")]
    Async(Box<dyn AsyncSource + Send + Sync>),
}

impl BuilderState for DefaultState {}
impl BuilderState for AsyncState {}

/// Operations allowed in any state
impl<St: BuilderState> ConfigBuilder<St> {
    /// Set a default `value` at `key`
    ///
    /// This value can be overwritten by any [`Source`], [`AsyncSource`] or override.
    ///
    /// # Errors
    ///
    /// Fails if `Expression::from_str(key)` fails.
    pub fn set_default<S, T>(mut self, key: S, value: T) -> Result<Self>
    where
        S: AsRef<str>,
        T: Into<Value>,
    {
        self.defaults
            .insert(Expression::from_str(key.as_ref())?, value.into());
        Ok(self)
    }

    /// Set an override
    ///
    /// This function sets an overwrite value. It will not be altered by any default, [`Source`] nor [`AsyncSource`]
    ///
    /// # Errors
    ///
    /// Fails if `Expression::from_str(key)` fails.
    pub fn set_override<S, T>(mut self, key: S, value: T) -> Result<Self>
    where
        S: AsRef<str>,
        T: Into<Value>,
    {
        self.overrides
            .insert(Expression::from_str(key.as_ref())?, value.into());
        Ok(self)
    }

    /// Sets an override if value is Some(_)
    ///
    /// This function sets an overwrite value if Some(_) is passed. If None is passed, this function does nothing.
    /// It will not be altered by any default, [`Source`] nor [`AsyncSource`]
    ///
    /// # Errors
    ///
    /// Fails if `Expression::from_str(key)` fails.
    pub fn set_override_option<S, T>(mut self, key: S, value: Option<T>) -> Result<Self>
    where
        S: AsRef<str>,
        T: Into<Value>,
    {
        if let Some(value) = value {
            self.overrides
                .insert(Expression::from_str(key.as_ref())?, value.into());
        }
        Ok(self)
    }
}

/// Operations allowed in sync state
impl ConfigBuilder<DefaultState> {
    /// Registers new [`Source`] in this builder.
    ///
    /// Calling this method does not invoke any I/O. [`Source`] is only saved in internal register for later use.
    pub fn add_source<T>(mut self, source: T) -> Self
    where
        T: Source + Send + Sync + 'static,
    {
        self.state.sources.push(Box::new(source));
        self
    }

    /// Registers new [`AsyncSource`] in this builder and forces transition to [`AsyncState`].
    ///
    /// Calling this method does not invoke any I/O. [`AsyncSource`] is only saved in internal register for later use.
    #[cfg(feature = "async")]
    pub fn add_async_source<T>(self, source: T) -> ConfigBuilder<AsyncState>
    where
        T: AsyncSource + Send + Sync + 'static,
    {
        let async_state = ConfigBuilder {
            state: AsyncState {
                sources: self
                    .state
                    .sources
                    .into_iter()
                    .map(SourceType::Sync)
                    .collect(),
            },
            defaults: self.defaults,
            overrides: self.overrides,
        };

        async_state.add_async_source(source)
    }

    /// Reads all registered [`Source`]s.
    ///
    /// This is the method that invokes all I/O operations.
    /// For a non consuming alternative see [`build_cloned`](Self::build_cloned)
    ///
    /// # Errors
    /// If source collection fails, be it technical reasons or related to inability to read data as `Config` for different reasons,
    /// this method returns error.
    pub fn build(self) -> Result<Config> {
        Self::build_internal(self.defaults, self.overrides, &self.state.sources)
    }

    /// Reads all registered [`Source`]s.
    ///
    /// Similar to [`build`](Self::build), but it does not take ownership of `ConfigBuilder` to allow later reuse.
    /// Internally it clones data to achieve it.
    ///
    /// # Errors
    /// If source collection fails, be it technical reasons or related to inability to read data as `Config` for different reasons,
    /// this method returns error.
    pub fn build_cloned(&self) -> Result<Config> {
        Self::build_internal(
            self.defaults.clone(),
            self.overrides.clone(),
            &self.state.sources,
        )
    }

    fn build_internal(
        defaults: Map<Expression, Value>,
        overrides: Map<Expression, Value>,
        sources: &[Box<dyn Source + Send + Sync>],
    ) -> Result<Config> {
        let mut cache: Value = Map::<String, Value>::new().into();

        // Add defaults
        for (key, val) in defaults {
            key.set(&mut cache, val);
        }

        // Add sources
        sources.collect_to(&mut cache)?;

        // Add overrides
        for (key, val) in overrides {
            key.set(&mut cache, val);
        }

        Ok(Config::new(cache))
    }
}

/// Operations allowed in async state
impl ConfigBuilder<AsyncState> {
    /// Registers new [`Source`] in this builder.
    ///
    /// Calling this method does not invoke any I/O. [`Source`] is only saved in internal register for later use.
    pub fn add_source<T>(mut self, source: T) -> Self
    where
        T: Source + Send + Sync + 'static,
    {
        self.state.sources.push(SourceType::Sync(Box::new(source)));
        self
    }

    /// Registers new [`AsyncSource`] in this builder.
    ///
    /// Calling this method does not invoke any I/O. [`AsyncSource`] is only saved in internal register for later use.
    #[cfg(feature = "async")]
    pub fn add_async_source<T>(mut self, source: T) -> Self
    where
        T: AsyncSource + Send + Sync + 'static,
    {
        self.state.sources.push(SourceType::Async(Box::new(source)));
        self
    }

    /// Reads all registered defaults, [`Source`]s, [`AsyncSource`]s and overrides.
    ///
    /// This is the method that invokes all I/O operations.
    /// For a non consuming alternative see [`build_cloned`](Self::build_cloned)
    ///
    /// # Errors
    /// If source collection fails, be it technical reasons or related to inability to read data as `Config` for different reasons,
    /// this method returns error.
    pub async fn build(self) -> Result<Config> {
        Self::build_internal(self.defaults, self.overrides, &self.state.sources).await
    }

    /// Reads all registered defaults, [`Source`]s, [`AsyncSource`]s and overrides.
    ///
    /// Similar to [`build`](Self::build), but it does not take ownership of `ConfigBuilder` to allow later reuse.
    /// Internally it clones data to achieve it.
    ///
    /// # Errors
    /// If source collection fails, be it technical reasons or related to inability to read data as `Config` for different reasons,
    /// this method returns error.
    pub async fn build_cloned(&self) -> Result<Config> {
        Self::build_internal(
            self.defaults.clone(),
            self.overrides.clone(),
            &self.state.sources,
        )
        .await
    }

    async fn build_internal(
        defaults: Map<Expression, Value>,
        overrides: Map<Expression, Value>,
        sources: &[SourceType],
    ) -> Result<Config> {
        let mut cache: Value = Map::<String, Value>::new().into();

        // Add defaults
        for (key, val) in defaults {
            key.set(&mut cache, val);
        }

        for source in sources.iter() {
            match source {
                SourceType::Sync(source) => source.collect_to(&mut cache)?,
                #[cfg(feature = "async")]
                SourceType::Async(source) => source.collect_to(&mut cache).await?,
            }
        }

        // Add overrides
        for (key, val) in overrides {
            key.set(&mut cache, val);
        }

        Ok(Config::new(cache))
    }
}
