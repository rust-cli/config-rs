use config::{Config, File, FileFormat};

#[test]
fn test_set_override_scalar() {
    let config = Config::builder()
        .set_override("value", true)
        .and_then(|b| b.build())
        .unwrap();

    assert_eq!(config.get("value").ok(), Some(true));
}

#[test]
#[cfg(feature = "json")]
fn test_set_scalar_default() {
    let config = Config::builder()
        .add_source(File::from_str(
            r#"
{
  "debug": true
}
"#,
            FileFormat::Json,
        ))
        .set_default("debug", false)
        .unwrap()
        .set_default("staging", false)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.get("debug").ok(), Some(true));
    assert_eq!(config.get("staging").ok(), Some(false));
}

#[test]
#[cfg(feature = "json")]
fn test_set_scalar_path() {
    let config = Config::builder()
        .set_override("first.second.third", true)
        .unwrap()
        .add_source(File::from_str(
            r#"
{
  "place": {
    "favorite": false
  }
}
"#,
            FileFormat::Json,
        ))
        .set_default("place.favorite", true)
        .unwrap()
        .set_default("place.blocked", true)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.get("first.second.third").ok(), Some(true));
    assert_eq!(config.get("place.favorite").ok(), Some(false));
    assert_eq!(config.get("place.blocked").ok(), Some(true));
}

#[test]
#[cfg(feature = "json")]
#[should_panic]
fn test_set_arr_path() {
    let config = Config::builder()
        .set_override("present[0].name", "Ivan")
        .unwrap()
        .set_override("absent[0].things[1].name", "foo")
        .unwrap()
        .set_override("absent[0].things[1].value", 42)
        .unwrap()
        .set_override("absent[1]", 0)
        .unwrap()
        .set_override("present[2]", "George")
        .unwrap()
        .set_override("reverse[-1]", "Bob")
        .unwrap()
        .set_override("reverse[-2]", "Alice")
        .unwrap()
        .set_override("empty[-1]", "Bob")
        .unwrap()
        .set_override("empty[-2]", "Alice")
        .unwrap()
        .add_source(File::from_str(
            r#"
{
  "present": [
    {
      "name": "1"
    },
    {
      "name": "2"
    }
  ],
  "reverse": [
    {
      "name": "l1"
    },
    {
      "name": "l2"
    }
  ],
  "empty": []
}
"#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();

    assert_eq!(config.get("present[0].name").ok(), Some("Ivan".to_owned()));
    assert_eq!(
        config.get("absent[0].things[1].name").ok(),
        Some("foo".to_owned())
    );
    assert_eq!(config.get("absent[0].things[1].value").ok(), Some(42));
    assert_eq!(config.get("absent[1]").ok(), Some(0));
    assert_eq!(config.get("present[2]").ok(), Some("George".to_owned()));
    assert_eq!(config.get("reverse[1]").ok(), Some("Bob".to_owned()));
    assert_eq!(config.get("reverse[0]").ok(), Some("Alice".to_owned()));
    assert_eq!(config.get("empty[1]").ok(), Some("Bob".to_owned()));
    assert_eq!(config.get("empty[0]").ok(), Some("Alice".to_owned()));
}

#[test]
#[cfg(feature = "json")]
fn test_set_capital() {
    let config = Config::builder()
        .set_default("this", false)
        .unwrap()
        .set_override("ThAt", true)
        .unwrap()
        .add_source(File::from_str("{\"logLevel\": 5}", FileFormat::Json))
        .build()
        .unwrap();

    assert_eq!(config.get::<bool>("this").unwrap(), false);
    assert_eq!(config.get::<bool>("ThAt").unwrap(), true);
    assert_eq!(config.get::<usize>("logLevel").unwrap(), 5);
}
