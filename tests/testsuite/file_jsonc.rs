#![cfg(feature = "jsonc")]

use serde_derive::Deserialize;

use config::{Config, File, FileFormat, Map, Value};
use float_cmp::ApproxEqUlps;
use snapbox::{assert_data_eq, str};

/// Returns the path to a test config file with an optional suffix.
///
/// # Example
/// If the current file path is `/workspace/config-rs/tests/testsuite/file_jsonc.rs`:
/// ```
/// let path = get_config_file_path("");
/// assert_eq!(path, "/workspace/config-rs/tests/testsuite/file_jsonc.jsonc");
///
/// let path = get_config_file_path(".extra");
/// assert_eq!(path, "/workspace/config-rs/tests/testsuite/file_jsonc.extra.jsonc");
/// ```
fn get_config_file_path(suffix: &str) -> String {
    let path = std::path::Path::new(file!());
    format!(
        "{}/{}/{}{}.jsonc",
        env!("CARGO_MANIFEST_DIR"),
        path.parent().unwrap().to_str().unwrap(),
        path.file_stem().unwrap().to_str().unwrap(),
        suffix
    )
}

#[test]
fn test_file() {
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

    #[derive(Debug, Deserialize)]
    struct Settings {
        debug: f64,
        production: Option<String>,
        place: Place,
        #[serde(rename = "arr")]
        elements: Vec<String>,
    }

    let c = Config::builder()
        .add_source(File::new(&get_config_file_path(""), FileFormat::Jsonc))
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
}

#[test]
fn test_error_parse() {
    let f = get_config_file_path(".error");
    let res = Config::builder()
        .add_source(File::new(&f, FileFormat::Jsonc))
        .build();

    assert!(res.is_err());
    let err = res.unwrap_err().to_string();
    let expected_prefix =
        "Expected colon after the string or word in object property on line 4 column 1 in ";
    assert!(
        err.starts_with(expected_prefix),
        "Error message does not start with expected prefix. Got: {}",
        err
    );
}

#[derive(Debug, Deserialize, PartialEq)]
#[allow(non_snake_case)]
struct OverrideSettings {
    FOO: String,
    foo: String,
}

#[test]
fn test_override_uppercase_value_for_struct() {
    std::env::set_var("APP_FOO", "I HAVE BEEN OVERRIDDEN_WITH_UPPER_CASE");

    let cfg = Config::builder()
        .add_source(File::new(&get_config_file_path(""), FileFormat::Jsonc))
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .build()
        .unwrap();

    let settings: OverrideSettings = cfg.try_deserialize().unwrap();
    assert_eq!(settings.FOO, "FOO should be overridden");
    assert_eq!(
        settings.foo,
        "I HAVE BEEN OVERRIDDEN_WITH_UPPER_CASE".to_owned()
    );
}

#[test]
fn test_override_lowercase_value_for_struct() {
    std::env::set_var("config_foo", "I have been overridden_with_lower_case");

    let cfg = Config::builder()
        .add_source(File::new(&get_config_file_path(""), FileFormat::Jsonc))
        .add_source(config::Environment::with_prefix("config").separator("_"))
        .build()
        .unwrap();

    let settings: OverrideSettings = cfg.try_deserialize().unwrap();
    assert_eq!(settings.FOO, "FOO should be overridden");
    assert_eq!(
        settings.foo,
        "I have been overridden_with_lower_case".to_owned()
    );
}

#[derive(Debug, Deserialize, PartialEq)]
enum EnumSettings {
    Bar(String),
}

#[test]
fn test_override_uppercase_value_for_enums() {
    std::env::set_var("APPS_BAR", "I HAVE BEEN OVERRIDDEN_WITH_UPPER_CASE");

    let cfg = Config::builder()
        .add_source(File::new(&get_config_file_path(".enum"), FileFormat::Jsonc))
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
    std::env::set_var("test_bar", "I have been overridden_with_lower_case");

    let cfg = Config::builder()
        .add_source(File::new(&get_config_file_path(".enum"), FileFormat::Jsonc))
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
fn test_nothing() {
    let res = Config::builder()
        .add_source(File::from_str("", FileFormat::Jsonc))
        .build();
    assert!(res.is_err());
    assert_data_eq!(
        res.unwrap_err().to_string(),
        format!("invalid type: unit value, expected a map")
    );
}
