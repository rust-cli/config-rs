#![cfg(feature = "toml")]

use chrono::{DateTime, TimeZone, Utc};
use float_cmp::ApproxEqUlps;
use serde_derive::Deserialize;
use snapbox::{assert_data_eq, str};

use config::{Config, File, FileFormat, Map, Value};

#[test]
fn test_file() {
    #[derive(Debug, Deserialize)]
    struct Settings {
        debug: f64,
        production: Option<String>,
        code: AsciiCode,
        place: Place,
        #[serde(rename = "arr")]
        elements: Vec<String>,
    }

    #[derive(Debug, Deserialize)]
    struct Place {
        number: PlaceNumber,
        name: String,
        longitude: f64,
        latitude: f64,
        favorite: bool,
        telephone: Option<String>,
        reviews: u64,
        creator: Map<String, Value>,
        rating: Option<f32>,
    }

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct PlaceNumber(u8);

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct AsciiCode(i8);

    let c = Config::builder()
        .add_source(File::from_str(
            r#"
debug = true
debug_s = "true"
production = false
production_s = "false"

code = 53

# errors
boolean_s_parse = "fals"

# For override tests
FOO="FOO should be overridden"
bar="I am bar"

arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
quarks = ["up", "down", "strange", "charm", "bottom", "top"]

[diodes]
green = "off"

[diodes.red]
brightness = 100

[diodes.blue]
blinking = [300, 700]

[diodes.white.pattern]
name = "christmas"
inifinite = true

[[items]]
name = "1"

[[items]]
name = "2"

[place]
number = 1
name = "Torre di Pisa"
longitude = 43.7224985
latitude = 10.3970522
favorite = false
reviews = 3866
rating = 4.5

[place.creator]
name = "John Smith"
username = "jsmith"
email = "jsmith@localhost"

[proton]
up = 2
down = 1

[divisors]
1 = 1
2 = 2
4 = 3
5 = 2
"#,
            FileFormat::Toml,
        ))
        .build()
        .unwrap();

    // Deserialize the entire file as single struct
    let s: Settings = c.try_deserialize().unwrap();

    assert!(s.debug.approx_eq_ulps(&1.0, 2));
    assert_eq!(s.production, Some("false".to_owned()));
    assert_eq!(s.code, AsciiCode(53));
    assert_eq!(s.place.number, PlaceNumber(1));
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
    let res = Config::builder()
        .add_source(File::from_str(
            r#"
ok = true
error = tru
"#,
            FileFormat::Toml,
        ))
        .build();

    assert!(res.is_err());
    assert_data_eq!(
        res.unwrap_err().to_string(),
        str![[r#"
TOML parse error at line 3, column 9
  |
3 | error = tru
  |         ^^^
invalid boolean, expected `true`

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
debug = true
debug_s = "true"
production = false
production_s = "false"

code = 53

# errors
boolean_s_parse = "fals"

# For override tests
FOO="FOO should be overridden"
bar="I am bar"

arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
quarks = ["up", "down", "strange", "charm", "bottom", "top"]

[diodes]
green = "off"

[diodes.red]
brightness = 100

[diodes.blue]
blinking = [300, 700]

[diodes.white.pattern]
name = "christmas"
inifinite = true

[[items]]
name = "1"

[[items]]
name = "2"

[place]
number = 1
name = "Torre di Pisa"
longitude = 43.7224985
latitude = 10.3970522
favorite = false
reviews = 3866
rating = 4.5

[place.creator]
name = "John Smith"
username = "jsmith"
email = "jsmith@localhost"

[proton]
up = 2
down = 1

[divisors]
1 = 1
2 = 2
4 = 3
5 = 2
"#,
            FileFormat::Toml,
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
debug_s = "true"
production = false
production_s = "false"

code = 53

# errors
boolean_s_parse = "fals"

# For override tests
FOO="FOO should be overridden"
bar="I am bar"

arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
quarks = ["up", "down", "strange", "charm", "bottom", "top"]

[diodes]
green = "off"

[diodes.red]
brightness = 100

[diodes.blue]
blinking = [300, 700]

[diodes.white.pattern]
name = "christmas"
inifinite = true

[[items]]
name = "1"

[[items]]
name = "2"

[place]
number = 1
name = "Torre di Pisa"
longitude = 43.7224985
latitude = 10.3970522
favorite = false
reviews = 3866
rating = 4.5

[place.creator]
name = "John Smith"
username = "jsmith"
email = "jsmith@localhost"

[proton]
up = 2
down = 1

[divisors]
1 = 1
2 = 2
4 = 3
5 = 2
"#,
            FileFormat::Toml,
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
bar = "bar is a lowercase param"
"#,
            FileFormat::Toml,
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
            FileFormat::Toml,
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
fn toml() {
    let s = Config::builder()
        .add_source(File::from_str(
            r#"
            toml_datetime = 2017-05-11T14:55:15Z
"#,
            FileFormat::Toml,
        ))
        .build()
        .unwrap();

    let date: String = s.get("toml_datetime").unwrap();
    assert_eq!(&date, "2017-05-11T14:55:15Z");
    let date: DateTime<Utc> = s.get("toml_datetime").unwrap();
    assert_eq!(date, Utc.with_ymd_and_hms(2017, 5, 11, 14, 55, 15).unwrap());
}
