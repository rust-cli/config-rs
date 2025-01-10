use snapbox::{assert_data_eq, prelude::*, str};

use config::{Config, File, FileFormat, Map};

#[test]
#[cfg(feature = "json")]
fn test_merge() {
    let c = Config::builder()
        .add_source(File::from_str(
            r#"
{
  "debug": true,
  "production": false,
  "place": {
    "rating": 4.5,
    "creator": {
      "name": "John Smith",
      "username": "jsmith",
      "email": "jsmith@localhost"
    }
  }
}
"#,
            FileFormat::Json,
        ))
        .add_source(File::from_str(
            r#"
{
  "debug": false,
  "production": true,
  "place": {
    "rating": 4.9,
    "creator": {
      "name": "Somebody New"
    }
  }
}
"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();

    assert_eq!(c.get("debug").ok(), Some(false));
    assert_eq!(c.get("production").ok(), Some(true));
    assert_eq!(c.get("place.rating").ok(), Some(4.9));

    if cfg!(feature = "preserve_order") {
        let m: Map<String, String> = c.get("place.creator").unwrap();
        assert_eq!(
            m.into_iter().collect::<Vec<(String, String)>>(),
            vec![
                ("name".to_owned(), "Somebody New".to_owned()),
                ("username".to_owned(), "jsmith".to_owned()),
                ("email".to_owned(), "jsmith@localhost".to_owned()),
            ]
        );
    } else {
        assert_eq!(
            c.get("place.creator.name").ok(),
            Some("Somebody New".to_owned())
        );
    }
}

#[test]
fn test_merge_whole_config() {
    let builder1 = Config::builder().set_override("x", 10).unwrap();
    let builder2 = Config::builder().set_override("y", 25).unwrap();

    let config1 = builder1.build_cloned().unwrap();
    let config2 = builder2.build_cloned().unwrap();

    assert_eq!(config1.get("x").ok(), Some(10));
    assert_eq!(config2.get::<()>("x").ok(), None);

    assert_eq!(config2.get("y").ok(), Some(25));
    assert_eq!(config1.get::<()>("y").ok(), None);

    let config3 = builder1.add_source(config2).build().unwrap();

    assert_eq!(config3.get("x").ok(), Some(10));
    assert_eq!(config3.get("y").ok(), Some(25));
}

#[test]
#[cfg(feature = "json")]
/// Test a few scenarios with empty maps:
fn test_merge_empty_maps() {
    use std::collections::BTreeMap;

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)] // temporary while this test is broken
    struct Settings {
        profile: BTreeMap<String, Profile>,
    }

    #[derive(Debug, Default, Deserialize)]
    #[allow(dead_code)] // temporary while this test is broken
    struct Profile {
        name: Option<String>,
    }

    // * missing_to_empty: no key -> empty map
    let cfg = Config::builder()
        .add_source(File::from_str(r#"{ "profile": {} }"#, FileFormat::Json))
        .add_source(File::from_str(
            r#"{ "profile": { "missing_to_empty": {} } }"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();
    let res = cfg.try_deserialize::<Settings>();
    assert_data_eq!(
        res.unwrap_err().to_string(),
        str![
            "invalid type: unit value, expected struct Profile for key `profile.missing_to_empty`"
        ]
    );

    // * missing_to_non_empty: no key -> map with k/v
    let cfg = Config::builder()
        .add_source(File::from_str(r#"{ "profile": {} }"#, FileFormat::Json))
        .add_source(File::from_str(
            r#"{ "profile": { "missing_to_non_empty": { "name": "bar" } } }"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();
    let res = cfg.try_deserialize::<Settings>();
    assert_data_eq!(
        res.unwrap().to_debug(),
        str![[r#"
Settings {
    profile: {
        "missing_to_non_empty": Profile {
            name: Some(
                "bar",
            ),
        },
    },
}

"#]]
    );

    // * empty_to_empty: empty map -> empty map
    let cfg = Config::builder()
        .add_source(File::from_str(
            r#"{ "profile": { "empty_to_empty": {} } }"#,
            FileFormat::Json,
        ))
        .add_source(File::from_str(
            r#"{ "profile": { "empty_to_empty": {} } }"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();
    let res = cfg.try_deserialize::<Settings>();
    assert_data_eq!(
        res.unwrap_err().to_string(),
        str!["invalid type: unit value, expected struct Profile for key `profile.empty_to_empty`"]
    );

    // * empty_to_non_empty: empty map -> map with k/v
    let cfg = Config::builder()
        .add_source(File::from_str(
            r#"{ "profile": { "empty_to_non_empty": {} } }"#,
            FileFormat::Json,
        ))
        .add_source(File::from_str(
            r#"{ "profile": { "empty_to_non_empty": { "name": "bar" } } }"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();
    let res = cfg.try_deserialize::<Settings>();
    assert_data_eq!(
        res.unwrap().to_debug(),
        str![[r#"
Settings {
    profile: {
        "empty_to_non_empty": Profile {
            name: Some(
                "bar",
            ),
        },
    },
}

"#]]
    );

    // * non_empty_to_empty: map with k/v -> empty map
    let cfg = Config::builder()
        .add_source(File::from_str(
            r#"{ "profile": { "non_empty_to_empty": { "name": "foo" } } }"#,
            FileFormat::Json,
        ))
        .add_source(File::from_str(
            r#"{ "profile": { "non_empty_to_empty": {} } }"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();
    let res = cfg.try_deserialize::<Settings>();
    assert_data_eq!(
        res.unwrap().to_debug(),
        str![[r#"
Settings {
    profile: {
        "non_empty_to_empty": Profile {
            name: Some(
                "foo",
            ),
        },
    },
}

"#]]
    );

    // * non_empty_to_non_empty: map with k/v -> map with k/v (override)
    let cfg = Config::builder()
        .add_source(File::from_str(
            r#"{ "profile": { "non_empty_to_non_empty": { "name": "foo" } } }"#,
            FileFormat::Json,
        ))
        .add_source(File::from_str(
            r#"{ "profile": { "non_empty_to_non_empty": { "name": "bar" } } }"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();
    let res = cfg.try_deserialize::<Settings>();
    assert_data_eq!(
        res.unwrap().to_debug(),
        str![[r#"
Settings {
    profile: {
        "non_empty_to_non_empty": Profile {
            name: Some(
                "bar",
            ),
        },
    },
}

"#]]
    );

    // * null_to_empty: null -> empty map
    // * null_to_non_empty: null -> map with k/v
    // * int_to_empty: int -> empty map
    // * int_to_non_empty: int -> map with k/v
    let cfg = Config::builder()
        .add_source(File::from_str(
            r#"{ "profile": { "null_to_empty": null } }"#,
            FileFormat::Json,
        ))
        .add_source(File::from_str(
            r#"{ "profile": { "null_to_empty": {} } }"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();
    let res = cfg.try_deserialize::<Settings>();
    assert_data_eq!(
        res.unwrap_err().to_string(),
        str!["invalid type: unit value, expected struct Profile for key `profile.null_to_empty`"]
    );

    // * null_to_non_empty: null -> map with k/v
    let cfg = Config::builder()
        .add_source(File::from_str(
            r#"{ "profile": { "null_to_non_empty": null } }"#,
            FileFormat::Json,
        ))
        .add_source(File::from_str(
            r#"{ "profile": { "null_to_non_empty": { "name": "bar" } } }"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();
    let res = cfg.try_deserialize::<Settings>();
    assert_data_eq!(
        res.unwrap().to_debug(),
        str![[r#"
Settings {
    profile: {
        "null_to_non_empty": Profile {
            name: Some(
                "bar",
            ),
        },
    },
}

"#]]
    );

    // * int_to_empty: int -> empty map
    let cfg = Config::builder()
        .add_source(File::from_str(
            r#"{ "profile": { "int_to_empty": 42 } }"#,
            FileFormat::Json,
        ))
        .add_source(File::from_str(
            r#"{ "profile": { "int_to_empty": {} } }"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();
    let res = cfg.try_deserialize::<Settings>();
    assert_data_eq!(
        res.unwrap_err().to_string(),
        str!["invalid type: integer `42`, expected struct Profile for key `profile.int_to_empty`"]
    );

    // * int_to_non_empty: int -> map with k/v
    let cfg = Config::builder()
        .add_source(File::from_str(
            r#"{ "profile": { "int_to_non_empty": 42 } }"#,
            FileFormat::Json,
        ))
        .add_source(File::from_str(
            r#"{ "int_to_non_empty": { "name": "bar" } }"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();
    let res = cfg.try_deserialize::<Settings>();
    assert_data_eq!(
        res.unwrap_err().to_string(),
        str!["invalid type: integer `42`, expected struct Profile for key `profile.int_to_non_empty`"]
    );
}
