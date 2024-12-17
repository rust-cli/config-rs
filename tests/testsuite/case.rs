use serde_derive::Deserialize;

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

    c.try_deserialize::<Kafka>().unwrap_err();
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

    c.get_string("Student[0].Name").unwrap_err();
}
