use config::Config;

#[test]
#[cfg(feature = "json")]
fn wrapping_u16() {
    let c = Config::builder()
        .add_source(config::File::from_str(
            r#"
{
    "settings": {
        "port": 66000
    }
}
"#,
            config::FileFormat::Json,
        ))
        .build()
        .unwrap();

    // FIXME: Can't compare ConfigError, because Unexpected are private.
    let _port_error = c.get::<u16>("settings.port").unwrap_err();
    /*
    assert!(matches!(
        Err(ConfigError::invalid_type(None, config::Unexpected::U64(66000), "an unsigned 16 bit integer"),)
        port_error
    ));
    */
}

#[test]
#[cfg(feature = "json")]
fn nonwrapping_u32() {
    let c = Config::builder()
        .add_source(config::File::from_str(
            r#"
{
    "settings": {
        "port": 66000
    }
}
"#,
            config::FileFormat::Json,
        ))
        .build()
        .unwrap();

    let port: u32 = c.get("settings.port").unwrap();
    assert_eq!(port, 66000);
}

#[test]
#[should_panic]
#[cfg(feature = "json")]
fn invalid_signedness() {
    let c = Config::builder()
        .add_source(config::File::from_str(
            r#"
{
    "settings": {
        "port": -1
    }
}
"#,
            config::FileFormat::Json,
        ))
        .build()
        .unwrap();

    let _: u32 = c.get("settings.port").unwrap();
}

#[cfg(feature = "preserve_order")]
#[test]
fn serde_i128_min() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    struct Container<T> {
        inner: T,
    }

    #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
    struct I128 {
        val: i128,
    }

    impl From<I128> for config::ValueKind {
        fn from(i: I128) -> Self {
            let mut properties = indexmap::IndexMap::new();
            properties.insert("val".to_owned(), config::Value::from(i.val));

            Self::Table(properties)
        }
    }

    let num = I128 { val: i128::MIN };
    let container = Container { inner: num };
    let built = Config::builder()
        .set_default("inner", num)
        .unwrap()
        .build()
        .unwrap();

    let deserialized = built.clone().try_deserialize::<Container<I128>>().unwrap();
    assert_eq!(deserialized, container);

    let serialized = Config::try_from(&container).unwrap();
    assert_eq!(serialized.cache, built.cache);
}

#[cfg(feature = "preserve_order")]
#[test]
fn serde_u128_max() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    struct Container<T> {
        inner: T,
    }

    #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
    struct U128 {
        val: u128,
    }

    impl From<U128> for config::ValueKind {
        fn from(i: U128) -> Self {
            let mut properties = indexmap::IndexMap::new();
            properties.insert("val".to_owned(), config::Value::from(i.val));

            Self::Table(properties)
        }
    }

    let num = U128 { val: u128::MAX };
    let container = Container { inner: num };
    let built = Config::builder()
        .set_default("inner", num)
        .unwrap()
        .build()
        .unwrap();

    let deserialized = built.clone().try_deserialize::<Container<U128>>().unwrap();
    assert_eq!(deserialized, container);

    let serialized = Config::try_from(&container).unwrap();
    assert_eq!(serialized.cache, built.cache);
}
