#![cfg(feature = "async")]
#![cfg(feature = "json")]
#![cfg(feature = "toml")]

use std::{env, fs, path, str::FromStr};

use async_trait::async_trait;
use config::{AsyncSource, Config, ConfigError, FileFormat, Format, Map, Value};
use tokio::fs::read_to_string;

#[derive(Debug)]
struct AsyncFile {
    path: String,
    format: FileFormat,
}

/// This is a test only implementation to be used in tests
impl AsyncFile {
    pub(crate) fn new(path: String, format: FileFormat) -> Self {
        Self { path, format }
    }
}

#[async_trait]
impl AsyncSource for AsyncFile {
    async fn collect(&self) -> Result<Map<String, Value>, ConfigError> {
        let mut path = env::current_dir().unwrap();
        let local = path::PathBuf::from_str(&self.path).unwrap();

        path.extend(local.iter());
        let path = fs::canonicalize(path).map_err(|e| ConfigError::Foreign(Box::new(e)))?;

        let text = read_to_string(path)
            .await
            .map_err(|e| ConfigError::Foreign(Box::new(e)))?;

        self.format
            .parse(Some(&self.path), &text)
            .map_err(ConfigError::Foreign)
    }
}

#[tokio::test]
async fn test_single_async_file_source() {
    let config = Config::builder()
        .add_async_source(AsyncFile::new(
            "tests/Settings.json".to_owned(),
            FileFormat::Json,
        ))
        .build()
        .await
        .unwrap();

    assert!(config.get::<bool>("debug").unwrap());
}

#[tokio::test]
async fn test_two_async_file_sources() {
    let config = Config::builder()
        .add_async_source(AsyncFile::new(
            "tests/Settings.json".to_owned(),
            FileFormat::Json,
        ))
        .add_async_source(AsyncFile::new(
            "tests/Settings.toml".to_owned(),
            FileFormat::Toml,
        ))
        .build()
        .await
        .unwrap();

    assert_eq!("Torre di Pisa", config.get::<String>("place.name").unwrap());
    assert!(config.get::<bool>("debug_json").unwrap());
    assert_eq!(1, config.get::<i32>("place.number").unwrap());
}

#[tokio::test]
async fn test_sync_to_async_file_sources() {
    let config = Config::builder()
        .add_source(config::File::new("tests/Settings", FileFormat::Json))
        .add_async_source(AsyncFile::new(
            "tests/Settings.toml".to_owned(),
            FileFormat::Toml,
        ))
        .build()
        .await
        .unwrap();

    assert_eq!("Torre di Pisa", config.get::<String>("place.name").unwrap());
    assert_eq!(1, config.get::<i32>("place.number").unwrap());
}

#[tokio::test]
async fn test_async_to_sync_file_sources() {
    let config = Config::builder()
        .add_async_source(AsyncFile::new(
            "tests/Settings.toml".to_owned(),
            FileFormat::Toml,
        ))
        .add_source(config::File::new("tests/Settings", FileFormat::Json))
        .build()
        .await
        .unwrap();

    assert_eq!("Torre di Pisa", config.get::<String>("place.name").unwrap());
    assert_eq!(1, config.get::<i32>("place.number").unwrap());
}

#[tokio::test]
async fn test_async_file_sources_with_defaults() {
    let config = Config::builder()
        .set_default("place.name", "Tower of London")
        .unwrap()
        .set_default("place.sky", "blue")
        .unwrap()
        .add_async_source(AsyncFile::new(
            "tests/Settings.toml".to_owned(),
            FileFormat::Toml,
        ))
        .build()
        .await
        .unwrap();

    assert_eq!("Torre di Pisa", config.get::<String>("place.name").unwrap());
    assert_eq!("blue", config.get::<String>("place.sky").unwrap());
    assert_eq!(1, config.get::<i32>("place.number").unwrap());
}

#[tokio::test]
async fn test_async_file_sources_with_overrides() {
    let config = Config::builder()
        .set_override("place.name", "Tower of London")
        .unwrap()
        .add_async_source(AsyncFile::new(
            "tests/Settings.toml".to_owned(),
            FileFormat::Toml,
        ))
        .build()
        .await
        .unwrap();

    assert_eq!(
        "Tower of London",
        config.get::<String>("place.name").unwrap()
    );
    assert_eq!(1, config.get::<i32>("place.number").unwrap());
}
