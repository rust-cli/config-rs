#![cfg(feature = "ini")]

use std::path::PathBuf;

use config::{Config, File, FileFormat};
use serde_derive::Deserialize;

#[test]
fn test_file() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Settings {
        debug: f64,
        place: Place,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Place {
        name: String,
        longitude: f64,
        latitude: f64,
        favorite: bool,
        reviews: u64,
        rating: Option<f32>,
    }

    let c = Config::builder()
        .add_source(File::new("tests/Settings", FileFormat::Ini))
        .build()
        .unwrap();
    let s: Settings = c.try_deserialize().unwrap();
    assert_eq!(
        s,
        Settings {
            debug: 1.0,
            place: Place {
                name: String::from("Torre di Pisa"),
                longitude: 43.722_498_5,
                latitude: 10.397_052_2,
                favorite: false,
                reviews: 3866,
                rating: Some(4.5),
            },
        }
    );
}

#[test]
fn test_error_parse() {
    let res = Config::builder()
        .add_source(File::new("tests/Settings-invalid", FileFormat::Ini))
        .build();

    let path: PathBuf = ["tests", "Settings-invalid.ini"].iter().collect();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        format!(
            r#"3:1 expecting "[Some('='), Some(':')]" but found EOF. in {}"#,
            path.display()
        )
    );
}

#[test]
fn test_override_uppercase_value_for_struct() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct StructSettings {
        foo: String,
        bar: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    #[allow(non_snake_case)]
    struct CapSettings {
        FOO: String,
    }

    std::env::set_var("APP_FOO", "I HAVE BEEN OVERRIDDEN_WITH_UPPER_CASE");

    let cfg = Config::builder()
        .add_source(File::new("tests/Settings", FileFormat::Ini))
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .build()
        .unwrap();
    let cap_settings = cfg.clone().try_deserialize::<CapSettings>();
    let lower_settings = cfg.try_deserialize::<StructSettings>().unwrap();

    match cap_settings {
        Ok(v) => {
            // this assertion will ensure that the map has only lowercase keys
            assert_ne!(v.FOO, "FOO should be overridden");
            assert_eq!(
                lower_settings.foo,
                "I HAVE BEEN OVERRIDDEN_WITH_UPPER_CASE".to_owned()
            );
        }
        Err(e) => {
            if e.to_string().contains("missing field `FOO`") {
                assert_eq!(
                    lower_settings.foo,
                    "I HAVE BEEN OVERRIDDEN_WITH_UPPER_CASE".to_owned()
                );
            } else {
                panic!("{}", e);
            }
        }
    }
}

#[test]
fn test_override_lowercase_value_for_struct() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct StructSettings {
        foo: String,
        bar: String,
    }

    std::env::set_var("config_foo", "I have been overridden_with_lower_case");

    let cfg = Config::builder()
        .add_source(File::new("tests/Settings", FileFormat::Ini))
        .add_source(config::Environment::with_prefix("config").separator("_"))
        .build()
        .unwrap();

    let values: StructSettings = cfg.try_deserialize().unwrap();
    assert_eq!(
        values.foo,
        "I have been overridden_with_lower_case".to_owned()
    );
    assert_ne!(values.foo, "I am bar".to_owned());
}

#[test]
fn test_override_uppercase_value_for_enums() {
    #[derive(Debug, Deserialize, PartialEq)]
    enum EnumSettings {
        Bar(String),
    }

    std::env::set_var("APPS_BAR", "I HAVE BEEN OVERRIDDEN_WITH_UPPER_CASE");

    let cfg = Config::builder()
        .add_source(File::new("tests/Settings-enum-test", FileFormat::Ini))
        .add_source(config::Environment::with_prefix("APPS").separator("_"))
        .build()
        .unwrap();
    let val: EnumSettings = cfg.try_deserialize().unwrap();

    assert_eq!(
        val,
        EnumSettings::Bar("I HAVE BEEN OVERRIDDEN_WITH_UPPER_CASE".to_owned())
    );
}

#[test]
fn test_override_lowercase_value_for_enums() {
    #[derive(Debug, Deserialize, PartialEq)]
    enum EnumSettings {
        Bar(String),
    }

    std::env::set_var("test_bar", "I have been overridden_with_lower_case");

    let cfg = Config::builder()
        .add_source(File::new("tests/Settings-enum-test", FileFormat::Ini))
        .add_source(config::Environment::with_prefix("test").separator("_"))
        .build()
        .unwrap();

    let param: EnumSettings = cfg.try_deserialize().unwrap();

    assert_eq!(
        param,
        EnumSettings::Bar("I have been overridden_with_lower_case".to_owned())
    );
}
