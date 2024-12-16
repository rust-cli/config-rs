#![cfg(feature = "yaml")]

use config::{Config, File, FileFormat};

#[test]
fn test_file_not_required() {
    let res = Config::builder()
        .add_source(File::new("tests/NoSettings", FileFormat::Yaml).required(false))
        .build();

    assert!(res.is_ok());
}

#[test]
fn test_file_required_not_found() {
    let res = Config::builder()
        .add_source(File::new("tests/NoSettings", FileFormat::Yaml))
        .build();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "configuration file \"tests/NoSettings\" not found".to_owned()
    );
}

#[test]
#[cfg(feature = "toml")]
fn test_file_auto() {
    let c = Config::builder()
        .add_source(File::with_name("tests/Settings-production"))
        .build()
        .unwrap();

    assert_eq!(c.get("debug").ok(), Some(false));
    assert_eq!(c.get("production").ok(), Some(true));
}

#[test]
fn test_file_auto_not_found() {
    let res = Config::builder()
        .add_source(File::with_name("tests/NoSettings"))
        .build();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "configuration file \"tests/NoSettings\" not found".to_owned()
    );
}

#[test]
#[cfg(feature = "json")]
fn test_file_ext() {
    let c = Config::builder()
        .add_source(File::with_name("tests/Settings.json"))
        .build()
        .unwrap();

    assert_eq!(c.get("debug").ok(), Some(true));
    assert_eq!(c.get("production").ok(), Some(false));
}

#[test]
#[cfg(feature = "ini")]
fn test_file_second_ext() {
    let c = Config::builder()
        .add_source(File::with_name("tests/Settings2.default"))
        .build()
        .unwrap();

    assert_eq!(c.get("debug").ok(), Some(true));
    assert_eq!(c.get("production").ok(), Some(false));
}
