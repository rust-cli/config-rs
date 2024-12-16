#![cfg(feature = "toml")]

use std::collections::HashSet;

use config::{Config, File, FileFormat, Map, Value};
use float_cmp::ApproxEqUlps;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Place {
    name: String,
    longitude: f64,
    latitude: f64,
    favorite: bool,
    telephone: Option<String>,
    reviews: u64,
    rating: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    debug: f64,
    production: Option<String>,
    place: Place,
}

fn make() -> Config {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings", FileFormat::Toml))
        .unwrap();

    c
}

#[test]
fn test_not_found() {
    let c = make();
    let res = c.get::<bool>("not_found");

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "configuration property \"not_found\" not found".to_owned()
    );
}

#[test]
fn test_scalar() {
    let c = make();

    assert_eq!(c.get("debug").ok(), Some(true));
    assert_eq!(c.get("production").ok(), Some(false));
}

#[test]
fn test_scalar_type_loose() {
    let c = make();

    assert_eq!(c.get("debug").ok(), Some(true));
    assert_eq!(c.get("debug").ok(), Some("true".to_owned()));
    assert_eq!(c.get("debug").ok(), Some(1));
    assert_eq!(c.get("debug").ok(), Some(1.0));

    assert_eq!(c.get("debug_s").ok(), Some(true));
    assert_eq!(c.get("debug_s").ok(), Some("true".to_owned()));
    assert_eq!(c.get("debug_s").ok(), Some(1));
    assert_eq!(c.get("debug_s").ok(), Some(1.0));

    assert_eq!(c.get("production").ok(), Some(false));
    assert_eq!(c.get("production").ok(), Some("false".to_owned()));
    assert_eq!(c.get("production").ok(), Some(0));
    assert_eq!(c.get("production").ok(), Some(0.0));

    assert_eq!(c.get("production_s").ok(), Some(false));
    assert_eq!(c.get("production_s").ok(), Some("false".to_owned()));
    assert_eq!(c.get("production_s").ok(), Some(0));
    assert_eq!(c.get("production_s").ok(), Some(0.0));
}

#[test]
fn test_get_scalar_path() {
    let c = make();

    assert_eq!(c.get("place.favorite").ok(), Some(false));
    assert_eq!(
        c.get("place.creator.name").ok(),
        Some("John Smith".to_owned())
    );
}

#[test]
fn test_get_scalar_path_subscript() {
    let c = make();

    assert_eq!(c.get("arr[2]").ok(), Some(3));
    assert_eq!(c.get("items[0].name").ok(), Some("1".to_owned()));
    assert_eq!(c.get("items[1].name").ok(), Some("2".to_owned()));
    assert_eq!(c.get("items[-1].name").ok(), Some("2".to_owned()));
    assert_eq!(c.get("items[-2].name").ok(), Some("1".to_owned()));
}

#[test]
fn test_map() {
    let c = make();
    let m: Map<String, Value> = c.get("place").unwrap();

    assert_eq!(m.len(), 8);
    assert_eq!(
        m["name"].clone().into_string().unwrap(),
        "Torre di Pisa".to_owned()
    );
    assert_eq!(m["reviews"].clone().into_int().unwrap(), 3866);
}

#[test]
fn test_map_str() {
    let c = make();
    let m: Map<String, String> = c.get("place.creator").unwrap();

    if cfg!(feature = "preserve_order") {
        assert_eq!(
            m.into_iter().collect::<Vec<(String, String)>>(),
            vec![
                ("name".to_owned(), "John Smith".to_owned()),
                ("username".to_owned(), "jsmith".to_owned()),
                ("email".to_owned(), "jsmith@localhost".to_owned()),
            ]
        );
    } else {
        assert_eq!(m.len(), 3);
        assert_eq!(m["name"], "John Smith".to_owned());
    }
}

#[test]
fn test_map_struct() {
    #[derive(Debug, Deserialize)]
    struct Settings {
        place: Map<String, Value>,
    }

    let c = make();
    let s: Settings = c.try_deserialize().unwrap();

    assert_eq!(s.place.len(), 8);
    assert_eq!(
        s.place["name"].clone().into_string().unwrap(),
        "Torre di Pisa".to_owned()
    );
    assert_eq!(s.place["reviews"].clone().into_int().unwrap(), 3866);
}

#[test]
fn test_file_struct() {
    let c = make();

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
}

#[test]
fn test_scalar_struct() {
    let c = make();

    // Deserialize a scalar struct that has lots of different
    // data types
    let p: Place = c.get("place").unwrap();

    assert_eq!(p.name, "Torre di Pisa");
    assert!(p.longitude.approx_eq_ulps(&43.722_498_5, 2));
    assert!(p.latitude.approx_eq_ulps(&10.397_052_2, 2));
    assert!(!p.favorite);
    assert_eq!(p.reviews, 3866);
    assert_eq!(p.rating, Some(4.5));
    assert_eq!(p.telephone, None);
}

#[test]
fn test_array_scalar() {
    let c = make();
    let arr: Vec<i64> = c.get("arr").unwrap();

    assert_eq!(arr.len(), 10);
    assert_eq!(arr[3], 4);
}

#[test]
fn test_struct_array() {
    #[derive(Debug, Deserialize)]
    struct Settings {
        #[serde(rename = "arr")]
        elements: Vec<String>,
    }

    let c = make();
    let s: Settings = c.try_deserialize().unwrap();

    assert_eq!(s.elements.len(), 10);
    assert_eq!(s.elements[3], "4".to_owned());
}

#[test]
fn test_enum() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "lowercase")]
    enum Diode {
        Off,
        Brightness(i32),
        Blinking(i32, i32),
        Pattern { name: String, inifinite: bool },
    }
    #[derive(Debug, Deserialize)]
    struct Settings {
        diodes: Map<String, Diode>,
    }

    let c = make();
    let s: Settings = c.try_deserialize().unwrap();

    assert_eq!(s.diodes["green"], Diode::Off);
    assert_eq!(s.diodes["red"], Diode::Brightness(100));
    assert_eq!(s.diodes["blue"], Diode::Blinking(300, 700));
    assert_eq!(
        s.diodes["white"],
        Diode::Pattern {
            name: "christmas".into(),
            inifinite: true,
        }
    );
}

#[test]
fn test_enum_key() {
    #[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
    #[serde(rename_all = "lowercase")]
    enum Quark {
        Up,
        Down,
        Strange,
        Charm,
        Bottom,
        Top,
    }

    #[derive(Debug, Deserialize)]
    struct Settings {
        proton: Map<Quark, usize>,
        // Just to make sure that set keys work too.
        quarks: HashSet<Quark>,
    }

    let c = make();
    let s: Settings = c.try_deserialize().unwrap();

    assert_eq!(s.proton[&Quark::Up], 2);
    assert_eq!(s.quarks.len(), 6);
}

#[test]
fn test_int_key() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Settings {
        divisors: Map<u32, u32>,
    }

    let c = make();
    let s: Settings = c.try_deserialize().unwrap();
    assert_eq!(s.divisors[&4], 3);
    assert_eq!(s.divisors.len(), 4);
}
