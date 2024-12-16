#![cfg(feature = "preserve_order")]

#[derive(serde_derive::Deserialize, Eq, PartialEq, Debug)]
struct Container<T> {
    inner: T,
}

#[derive(serde_derive::Deserialize, Eq, PartialEq, Debug)]
struct Unsigned {
    unsigned: u16,
}

impl Default for Unsigned {
    fn default() -> Self {
        Self { unsigned: 128 }
    }
}

impl From<Unsigned> for config::ValueKind {
    fn from(unsigned: Unsigned) -> Self {
        let mut properties = indexmap::IndexMap::new();
        properties.insert(
            "unsigned".to_owned(),
            config::Value::from(unsigned.unsigned),
        );

        Self::Table(properties)
    }
}

#[test]
fn test_deser_unsigned_int() {
    let container = Container {
        inner: Unsigned::default(),
    };

    let built = config::Config::builder()
        .set_default("inner", Unsigned::default())
        .unwrap()
        .build()
        .unwrap()
        .try_deserialize::<Container<Unsigned>>()
        .unwrap();

    assert_eq!(container, built);
}
