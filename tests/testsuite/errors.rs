#![cfg(feature = "toml")]

use std::path::PathBuf;

use config::{Config, ConfigError, File, FileFormat, Map, Value};
use serde_derive::Deserialize;

fn make() -> Config {
    Config::builder()
        .add_source(File::new("tests/Settings", FileFormat::Toml))
        .build()
        .unwrap()
}

#[test]
fn test_error_parse() {
    let res = Config::builder()
        .add_source(File::new("tests/Settings-invalid", FileFormat::Toml))
        .build();

    assert!(res.is_err());
    assert!(res
        .unwrap_err()
        .to_string()
        .contains("TOML parse error at line 2, column 9"));
}

#[test]
fn test_error_type() {
    let c = make();

    let res = c.get::<bool>("boolean_s_parse");

    let path: PathBuf = ["tests", "Settings.toml"].iter().collect();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        format!(
            "invalid type: string \"fals\", expected a boolean for key `boolean_s_parse` in {}",
            path.display()
        )
    );
}

#[test]
#[cfg(unix)]
fn test_error_deser_whole() {
    #[derive(Deserialize, Debug)]
    struct Place {
        #[allow(dead_code)]
        name: usize, // is actually s string
    }

    #[derive(Deserialize, Debug)]
    struct Output {
        #[allow(dead_code)]
        place: Place,
    }

    let c = make();
    let err = c.try_deserialize::<Output>().unwrap_err().to_string();
    assert_eq!(
        err,
        "invalid type: string \"Torre di Pisa\", expected an integer for key `place.name` in tests/Settings.toml",
    );
}

#[test]
fn test_error_type_detached() {
    let c = make();

    let value = c.get::<Value>("boolean_s_parse").unwrap();
    let res = value.try_deserialize::<bool>();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "invalid type: string \"fals\", expected a boolean".to_owned()
    );
}

#[test]
fn test_error_type_get_bool() {
    let c = make();

    let res = c.get_bool("boolean_s_parse");

    let path: PathBuf = ["tests", "Settings.toml"].iter().collect();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        format!(
            "invalid type: string \"fals\", expected a boolean for key `boolean_s_parse` in {}",
            path.display()
        )
    );
}

#[test]
fn test_error_type_get_table() {
    let c = make();

    let res = c.get_table("debug");

    let path: PathBuf = ["tests", "Settings.toml"].iter().collect();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        format!(
            "invalid type: boolean `true`, expected a map for key `debug` in {}",
            path.display()
        )
    );
}

#[test]
fn test_error_type_get_array() {
    let c = make();

    let res = c.get_array("debug");

    let path: PathBuf = ["tests", "Settings.toml"].iter().collect();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        format!(
            "invalid type: boolean `true`, expected an array for key `debug` in {}",
            path.display()
        )
    );
}

#[test]
fn test_error_enum_de() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    enum Diode {
        Off,
        Brightness(i32),
        Blinking(i32, i32),
        Pattern { name: String, inifinite: bool },
    }

    let on_v: Value = "on".into();
    let on_d = on_v.try_deserialize::<Diode>();
    assert_eq!(
        on_d.unwrap_err().to_string(),
        "enum Diode does not have variant constructor on".to_owned()
    );

    let array_v: Value = vec![100, 100].into();
    let array_d = array_v.try_deserialize::<Diode>();
    assert_eq!(
        array_d.unwrap_err().to_string(),
        "value of enum Diode should be represented by either string or table with exactly one key"
    );

    let confused_v: Value = [
        ("Brightness".to_owned(), 100.into()),
        ("Blinking".to_owned(), vec![300, 700].into()),
    ]
    .iter()
    .cloned()
    .collect::<Map<String, Value>>()
    .into();
    let confused_d = confused_v.try_deserialize::<Diode>();
    assert_eq!(
        confused_d.unwrap_err().to_string(),
        "value of enum Diode should be represented by either string or table with exactly one key"
    );
}

#[test]
fn error_with_path() {
    #[derive(Debug, Deserialize)]
    struct Inner {
        #[allow(dead_code)]
        test: i32,
    }

    #[derive(Debug, Deserialize)]
    struct Outer {
        #[allow(dead_code)]
        inner: Inner,
    }
    const CFG: &str = r#"
inner:
    test: ABC
"#;

    let e = Config::builder()
        .add_source(File::from_str(CFG, FileFormat::Yaml))
        .build()
        .unwrap()
        .try_deserialize::<Outer>()
        .unwrap_err();

    if let ConfigError::Type {
        key: Some(path), ..
    } = e
    {
        assert_eq!(path, "inner.test");
    } else {
        panic!("Wrong error {:?}", e);
    }
}

#[test]
fn test_error_root_not_table() {
    match Config::builder()
        .add_source(File::from_str(r#"false"#, FileFormat::Json5))
        .build()
    {
        Ok(_) => panic!("Should not merge if root is not a table"),
        Err(e) => match e {
            ConfigError::FileParse { cause, .. } => assert_eq!(
                "invalid type: boolean `false`, expected a map",
                format!("{cause}")
            ),
            _ => panic!("Wrong error: {:?}", e),
        },
    }
}
