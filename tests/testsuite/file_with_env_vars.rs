#![cfg(all(feature = "yaml", feature = "substitute_env"))]


use float_cmp::ApproxEqUlps;
use serde::Deserialize;

use config::{Config, File, FileFormat, Map, Value};

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

#[test]
fn test_yaml_file_without_envvars() {
    temp_env::with_vars(
        [("LONGITUDE", None as Option<&str>), ("REVIEWS", None), ("NAME", None)],
        || {
            let c = Config::builder()
                .add_source(File::from_str(
                    r#"
debug: true
production: false
arr: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
place:
  name: Torre di Pisa
  longitude: ${LONGITUDE:-43.7224985}
  latitude: 10.3970522
  favorite: false
  reviews: ${REVIEWS:-3866}
  rating: 4.5
  creator:
    name: ${NAME:-John Smith}
    username: jsmith
    email: jsmith@localhost
# For override tests
FOO: FOO should be overridden
bar: I am bar
nullable: null
"#,
                    FileFormat::Yaml,
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
        },
    )
}

#[test]
fn test_yaml_file_with_env_vars() {
    temp_env::with_vars(
        [("LONGITUDE", Some("43.0224985")), ("REVIEWS", Some("3066")), ("NAME",Some("John Watts")),("SIX",Some("6"))],
        || {
            let c = Config::builder()
                .add_source(File::from_str(
                    r#"
debug: true
production: false
arr: [1, 2, 3, 4, 5, "${SIX:-1}", 7, 8, 9, 10]
place:
  name: Torre di Pisa
  longitude: ${LONGITUDE:-43.7224985}
  latitude: 10.3970522
  favorite: false
  reviews: ${REVIEWS:-3866}
  rating: 4.5
  creator:
    name: ${NAME:-John Smith}
    username: jsmith
    email: jsmith@localhost
# For override tests
FOO: FOO should be overridden
bar: I am bar
nullable: null
"#,
                    FileFormat::Yaml,
                ))
                .build()
                .unwrap();

            // Deserialize the entire file as single struct
            let s: Settings = c.try_deserialize().unwrap();

            assert!(s.debug.approx_eq_ulps(&1.0, 2));
            assert_eq!(s.production, Some("false".to_owned()));
            assert_eq!(s.place.name, "Torre di Pisa");
            assert!(s.place.longitude.approx_eq_ulps(&43.022_498_5, 2));
            assert!(s.place.latitude.approx_eq_ulps(&10.397_052_2, 2));
            assert!(!s.place.favorite);
            assert_eq!(s.place.reviews, 3066);
            assert_eq!(s.place.rating, Some(4.5));
            assert_eq!(s.place.telephone, None);
            assert_eq!(s.elements.len(), 10);
            assert_eq!(s.elements[3], "4".to_owned());
            assert_eq!(s.elements[5], "6".to_owned());
            if cfg!(feature = "preserve_order") {
                assert_eq!(
                    s.place
                        .creator
                        .into_iter()
                        .collect::<Vec<(String, Value)>>(),
                    vec![
                        ("name".to_owned(), "John Watts".into()),
                        ("username".into(), "jsmith".into()),
                        ("email".into(), "jsmith@localhost".into()),
                    ]
                );
            } else {
                assert_eq!(
                    s.place.creator["name"].clone().into_string().unwrap(),
                    "John Watts".to_owned()
                );
            }
            assert_eq!(s.nullable, None);
        },
    )
}


#[test]
fn test_toml_file_without_envvars() {
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

arr = [1, 2, 3, 4, 5, "${SIX:-6}", 7, 8, 9, 10]
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
