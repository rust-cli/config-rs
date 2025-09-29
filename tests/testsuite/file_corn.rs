#![cfg(feature = "corn")]

use chrono::{DateTime, TimeZone, Utc};
use float_cmp::ApproxEqUlps;
use serde::Deserialize;
use snapbox::{assert_data_eq, str};

use config::{Config, File, FileFormat, Map, Value};

#[test]
fn test_file() {
    #[derive(Debug, Deserialize)]
    struct Settings {
        debug: f64,
        production: Option<String>,
        place: Place,
        #[serde(rename = "arr")]
        elements: Vec<String>,
        nullable: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    struct Place {
        name: String,
        longitude: f64,
        latitude: f64,
        favorite: bool,
        telephone: Option<String>,
        reviews: u64,
        creator: Map<String, Value>,
        rating: Option<f32>,
    }

    let c = Config::builder()
        .add_source(File::from_str(
            r#"
{
  // c
  debug = true
  production = false
  arr = [1 2 3 4 5 6 7 8 9 10]
  place = {
    name = "Torre di Pisa"
    longitude = 43.7224985
    latitude =10.3970522
    favorite=  false
    reviews = 3866
    rating = 4.5
    creator = {
      name = "John Smith"
      username = "jsmith"
      email = "jsmith@localhost"
    }
  }
  FOO = "FOO should be overridden"
  bar = "I am bar"
  nullable = null
}
"#,
            FileFormat::Corn,
        ))
        .build()
        .unwrap();

    // Deserialize the entire file as single struct
    let s: Settings = c.try_deserialize().unwrap();

    assert!(s.debug.approx_eq_ulps(&1.0, 2));
    assert_eq!(s.production, Some("false".to_owned()));
    assert_eq!(s.place.name, "Torre di Pisa");
    assert!(s.place.longitude.approx_eq_ulps(&43.722_498_5, 2));
    assert!(s.place.latitude.approx_eq_ulps(&10.397_052_2, 2));
    assert!(!s.place.favorite);
    assert_eq!(s.place.reviews, 3866);
    assert_eq!(s.place.rating, Some(4.5));
    assert_eq!(s.place.telephone, None);
    assert_eq!(s.elements.len(), 10);
    assert_eq!(s.elements[3], "4".to_owned());
    if cfg!(feature = "preserve_order") {
        assert_eq!(
            s.place
                .creator
                .into_iter()
                .collect::<Vec<(String, Value)>>(),
            vec![
                ("name".to_owned(), "John Smith".into()),
                ("username".into(), "jsmith".into()),
                ("email".into(), "jsmith@localhost".into()),
            ]
        );
    } else {
        assert_eq!(
            s.place.creator["name"].clone().into_string().unwrap(),
            "John Smith".to_owned()
        );
    }
    assert_eq!(s.nullable, None);
}

#[test]
fn test_error_parse() {
    let res = Config::builder()
        .add_source(File::from_str(
            r#"
{
  ok = true
  error
}
"#,
            FileFormat::Corn,
        ))
        .build();

    assert!(res.is_err());
    assert_data_eq!(
        res.unwrap_err().to_string(),
        str![[r#"
 --> 4:3
  |
4 |   error
  |   ^---
  |
  = expected spread or quoted_path_seg
"#]]
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
{
  // c
  debug = true
  production = false
  arr = [1 2 3 4 5 6 7 8 9 10]
  place = {
    name = "Torre di Pisa"
    longitude = 43.7224985
    latitude = 10.3970522
    favorite = false
    reviews = 3866
    rating = 4.5
    creator = {
      name = "John Smith"
      username = "jsmith"
      email = "jsmith@localhost"
    }
  }
  FOO = "FOO should be overridden"
  bar = "I am bar"
}
"#,
            FileFormat::Corn,
        ))
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .build()
        .unwrap();

    let cap_settings = cfg.clone().try_deserialize::<CapSettings>();
    let lower_settings = cfg.try_deserialize::<StructSettings>().unwrap();

    match cap_settings {
        Ok(v) => {
            // this assertion will ensure that the map has only lowercase keys
            assert_eq!(v.FOO, "FOO should be overridden");
            assert_eq!(
                lower_settings.foo,
                "I HAVE BEEN OVERRIDDEN_WITH_UPPER_CASE".to_owned()
            );
        }
        Err(e) => {
            if matches!(e, config::ConfigError::NotFound(_)) {
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
{
  // c
  debug = true
  production = false
  arr = [1 2 3 4 5 6 7 8 9 10]
  place = {
    name = "Torre di Pisa"
    longitude = 43.7224985
    latitude = 10.3970522
    favorite = false
    reviews = 3866
    rating = 4.5
    creator = {
      name = "John Smith"
      username = "jsmith"
      email = "jsmith@localhost"
    }
  }
  FOO = "FOO should be overridden"
  bar = "I am bar"
}
"#,
            FileFormat::Corn,
        ))
        .add_source(config::Environment::with_prefix("config").separator("_"))
        .build()
        .unwrap();

    let values: StructSettings = cfg.try_deserialize().unwrap();
    assert_eq!(
        values.foo,
        "I have been overridden_with_lower_case".to_owned()
    );
    assert_eq!(values.bar, "I am bar".to_owned());
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
{
    bar = "bar is a lowercase param"
}
"#,
            FileFormat::Corn,
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
{
    bar = "bar is a lowercase param"
}
"#,
            FileFormat::Corn,
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
fn corn() {
    let s = Config::builder()
        .add_source(File::from_str(
            r#"
            {
                corn_datetime = "2017-05-10T02:14:53Z"
            }
            "#,
            FileFormat::Corn,
        ))
        .build()
        .unwrap();

    let date: String = s.get("corn_datetime").unwrap();
    assert_eq!(&date, "2017-05-10T02:14:53Z");
    let date: DateTime<Utc> = s.get("corn_datetime").unwrap();
    assert_eq!(date, Utc.with_ymd_and_hms(2017, 5, 10, 2, 14, 53).unwrap());
}
