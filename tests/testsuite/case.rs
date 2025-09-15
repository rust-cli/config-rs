use serde::Deserialize;

use config::{Config, File, FileFormat};

#[test]
#[cfg(feature = "json")]
fn respect_field_case() {
    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    struct Kafka {
        broker: String,
        topic: String,
        pollSleep: u64, //<---
    }

    let c = Config::builder()
        .add_source(File::from_str(
            r#"
{
  "broker": "localhost:29092",
  "topic": "rust",
  "pollSleep": 1000
}
"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();

    c.try_deserialize::<Kafka>().unwrap();
}

#[test]
#[cfg(feature = "json")]
fn respect_renamed_field() {
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct MyConfig {
        #[serde(rename = "FooBar")]
        foo_bar: String,
    }

    let c = Config::builder()
        .add_source(File::from_str(
            r#"
{
  "FooBar": "Hello, world!"
}
"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();

    c.try_deserialize::<MyConfig>().unwrap();
}

#[test]
#[cfg(feature = "json")]
fn respect_path_case() {
    let c = Config::builder()
        .add_source(File::from_str(
            r#"
{
  "Student": [
    { "Name": "1" },
    { "Name": "2" }
  ]
}
"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();

    c.get_string("Student[0].Name").unwrap();
}
