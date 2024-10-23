#[test]
fn test_rename_attr() {
    use config::{Config, File, FileFormat};

    #[allow(unused)]
    #[derive(serde::Deserialize, Debug)]
    struct MyConfig {
        #[serde(rename = "FooBar")]
        foo_bar: String,
    }

    const MY_CONFIG: &str = r#"{
        "FooBar": "Hello, world!"
    }"#;

    let cfg = Config::builder()
        .add_source(File::from_str(MY_CONFIG, FileFormat::Json))
        .build()
        .unwrap();

    let desered: Result<MyConfig, _> = cfg.try_deserialize();
    assert!(desered.is_ok(), "Not Ok(_): {}", desered.unwrap_err());
}
