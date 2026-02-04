#![cfg(feature = "preserve_order")]

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

#[test]
fn test_serde_u128_max() {
    let num = U128 { val: u128::MAX };
    let container = Container { inner: num };
    let built = config::Config::builder()
        .set_default("inner", num)
        .unwrap()
        .build()
        .unwrap();

    let deserialized = built.clone().try_deserialize::<Container<U128>>().unwrap();
    assert_eq!(deserialized, container);

    let serialized = config::Config::try_from(&container).unwrap();
    assert_eq!(serialized.cache, built.cache);
}
