#![cfg(feature = "async")]
#![cfg(feature = "json")]

use async_trait::async_trait;

use config::{AsyncSource, Config, ConfigError, FileFormat, Format, Map, Value};

#[derive(Debug)]
struct AsyncJson(&'static str);

#[async_trait]
impl AsyncSource for AsyncJson {
    async fn collect(&self) -> Result<Map<String, Value>, ConfigError> {
        let text = self.0;

        FileFormat::Json
            .parse(None, text)
            .map_err(ConfigError::Foreign)
    }
}

#[tokio::test]
async fn test_single_async_file_source() {
    let config = Config::builder()
        .add_async_source(AsyncJson(
            r#"
{
    "debug": true
}
"#,
        ))
        .build()
        .await
        .unwrap();

    assert!(config.get::<bool>("debug").unwrap());
}

#[tokio::test]
async fn test_two_async_file_sources() {
    let config = Config::builder()
        .add_async_source(AsyncJson(
            r#"
{
  "debug_json": true,
  "place": {
    "name": "Torre di Pisa"
  }
}
"#,
        ))
        .add_async_source(AsyncJson(
            r#"
{
  "place": {
    "name": "Torre di Pisa",
    "number": 1
  }
}
"#,
        ))
        .build()
        .await
        .unwrap();

    assert_eq!(config.get::<String>("place.name").unwrap(), "Torre di Pisa");
    assert_eq!(config.get::<i32>("place.number").unwrap(), 1);
    assert!(config.get::<bool>("debug_json").unwrap());
}

#[tokio::test]
async fn test_sync_to_async_file_sources() {
    let config = Config::builder()
        .add_source(config::File::from_str(
            r#"
{
  "debug_json": true,
  "place": {
    "name": "Torre di Pisa"
  }
}
"#,
            FileFormat::Json,
        ))
        .add_async_source(AsyncJson(
            r#"
{
  "place": {
    "name": "Torre di Pisa",
    "number": 1
  }
}
"#,
        ))
        .build()
        .await
        .unwrap();

    assert_eq!(config.get::<String>("place.name").unwrap(), "Torre di Pisa",);
    assert_eq!(config.get::<i32>("place.number").unwrap(), 1);
}

#[tokio::test]
async fn test_async_to_sync_file_sources() {
    let config = Config::builder()
        .add_async_source(AsyncJson(
            r#"
{
  "place": {
    "name": "Torre di Pisa",
    "number": 1
  }
}
"#,
        ))
        .add_source(config::File::from_str(
            r#"
{
  "debug_json": true,
  "place": {
    "name": "Torre di Pisa"
  }
}
"#,
            FileFormat::Json,
        ))
        .build()
        .await
        .unwrap();

    assert_eq!(config.get::<String>("place.name").unwrap(), "Torre di Pisa",);
    assert_eq!(config.get::<i32>("place.number").unwrap(), 1,);
}

#[tokio::test]
async fn test_async_file_sources_with_defaults() {
    let config = Config::builder()
        .set_default("place.name", "Tower of London")
        .unwrap()
        .set_default("place.sky", "blue")
        .unwrap()
        .add_async_source(AsyncJson(
            r#"
{
  "place": {
    "name": "Torre di Pisa",
    "number": 1
  }
}
"#,
        ))
        .build()
        .await
        .unwrap();

    assert_eq!(config.get::<String>("place.name").unwrap(), "Torre di Pisa",);
    assert_eq!(config.get::<String>("place.sky").unwrap(), "blue",);
    assert_eq!(config.get::<i32>("place.number").unwrap(), 1);
}

#[tokio::test]
async fn test_async_file_sources_with_overrides() {
    let config = Config::builder()
        .set_override("place.name", "Tower of London")
        .unwrap()
        .add_async_source(AsyncJson(
            r#"
{
  "place": {
    "name": "Torre di Pisa",
    "number": 1
  }
}
"#,
        ))
        .build()
        .await
        .unwrap();

    assert_eq!(
        config.get::<String>("place.name").unwrap(),
        "Tower of London",
    );
    assert_eq!(config.get::<i32>("place.number").unwrap(), 1);
}
