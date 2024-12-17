#![cfg(feature = "ini")]

use chrono::{DateTime, TimeZone, Utc};
use serde_derive::Deserialize;
use snapbox::{assert_data_eq, str};

use config::{Config, File, FileFormat};

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
        .add_source(File::from_str(
            r#"
debug = true
production = false
FOO = FOO should be overridden
bar = I am bar
[place]
name = Torre di Pisa
longitude = 43.7224985
latitude = 10.3970522
favorite = false
reviews = 3866
rating = 4.5
"#,
            FileFormat::Ini,
        ))
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
        .add_source(File::from_str(
            r#"
ok : true,
error
"#,
            FileFormat::Ini,
        ))
        .build();

    assert!(res.is_err());
    assert_data_eq!(
        res.unwrap_err().to_string(),
        str![[r#"4:1 expecting "[Some('='), Some(':')]" but found EOF."#]]
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
        .add_source(File::from_str(
            r#"
debug = true
production = false
FOO = FOO should be overridden
bar = I am bar
[place]
name = Torre di Pisa
longitude = 43.7224985
latitude = 10.3970522
favorite = false
reviews = 3866
rating = 4.5
"#,
            FileFormat::Ini,
        ))
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
        .add_source(File::from_str(
            r#"
debug = true
production = false
FOO = FOO should be overridden
bar = I am bar
[place]
name = Torre di Pisa
longitude = 43.7224985
latitude = 10.3970522
favorite = false
reviews = 3866
rating = 4.5
"#,
            FileFormat::Ini,
        ))
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
        .add_source(File::from_str(
            r#"
bar = "bar is a lowercase param"
"#,
            FileFormat::Ini,
        ))
        .add_source(config::Environment::with_prefix("APPS").separator("_"))
        .build()
        .unwrap();

    let param = cfg.try_deserialize::<EnumSettings>();
    assert!(param.is_err());
    assert_data_eq!(
        param.unwrap_err().to_string(),
        str!["enum EnumSettings does not have variant constructor bar"]
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
        .add_source(File::from_str(
            r#"
bar = "bar is a lowercase param"
"#,
            FileFormat::Ini,
        ))
        .add_source(config::Environment::with_prefix("test").separator("_"))
        .build()
        .unwrap();

    let param = cfg.try_deserialize::<EnumSettings>();
    assert!(param.is_err());
    assert_data_eq!(
        param.unwrap_err().to_string(),
        str!["enum EnumSettings does not have variant constructor bar"]
    );
}

#[test]
fn ini() {
    let s = Config::builder()
        .add_source(File::from_str(
            r#"
                ini_datetime = 2017-05-10T02:14:53Z
            "#,
            FileFormat::Ini,
        ))
        .build()
        .unwrap();

    let date: String = s.get("ini_datetime").unwrap();
    assert_eq!(&date, "2017-05-10T02:14:53Z");
    let date: DateTime<Utc> = s.get("ini_datetime").unwrap();
    assert_eq!(date, Utc.with_ymd_and_hms(2017, 5, 10, 2, 14, 53).unwrap());
}
