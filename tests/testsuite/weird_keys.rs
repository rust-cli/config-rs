//! Please note: This file is named "weird" keys because these things are normally not keys, not
//! because your software is weird if it expects these keys in the config file.
//!
//! Please don't be offended!

use serde::{Deserialize, Serialize};

use config::{File, FileFormat};

/// Helper fn to test the different deserializations
fn test_config_as<'a, T>(config: &str, format: FileFormat) -> T
where
    T: Deserialize<'a> + std::fmt::Debug,
{
    let cfg = config::Config::builder()
        .add_source(File::from_str(config, format))
        .build();

    assert!(cfg.is_ok(), "Config could not be built: {:?}", cfg);
    let cfg = cfg.unwrap().try_deserialize();

    assert!(cfg.is_ok(), "Config could not be transformed: {:?}", cfg);
    let cfg: T = cfg.unwrap();
    cfg
}

#[test]
#[cfg(feature = "json")]
fn test_colon_key_json() {
    #[derive(Debug, Serialize, Deserialize)]
    struct SettingsColon {
        #[serde(rename = "foo:foo")]
        foo: u8,

        bar: u8,
    }

    let config = r#" {"foo:foo": 8, "bar": 12 } "#;

    let cfg = test_config_as::<SettingsColon>(config, FileFormat::Json);
    assert_eq!(cfg.foo, 8);
    assert_eq!(cfg.bar, 12);
}

#[test]
#[cfg(feature = "json")]
fn test_slash_key_json() {
    #[derive(Debug, Serialize, Deserialize)]
    struct SettingsSlash {
        #[serde(rename = "foo/foo")]
        foo: u8,
        bar: u8,
    }

    let config = r#" {"foo/foo": 8, "bar": 12 } "#;

    let cfg = test_config_as::<SettingsSlash>(config, FileFormat::Json);
    assert_eq!(cfg.foo, 8);
    assert_eq!(cfg.bar, 12);
}

#[test]
#[cfg(feature = "json")]
fn test_doublebackslash_key_json() {
    #[derive(Debug, Serialize, Deserialize)]
    struct SettingsDoubleBackslash {
        #[serde(rename = "foo\\foo")]
        foo: u8,
        bar: u8,
    }

    let config = r#" {"foo\\foo": 8, "bar": 12 } "#;

    let cfg = test_config_as::<SettingsDoubleBackslash>(config, FileFormat::Json);
    assert_eq!(cfg.foo, 8);
    assert_eq!(cfg.bar, 12);
}
