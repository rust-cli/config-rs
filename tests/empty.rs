use serde_derive::{Deserialize, Serialize};

use config::Config;

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    #[serde(skip)]
    foo: isize,
    #[serde(skip)]
    bar: u8,
}

#[test]
fn empty_deserializes() {
    let s: Settings = Config::default()
        .try_deserialize()
        .expect("Deserialization failed");
    assert_eq!(s.foo, 0);
    assert_eq!(s.bar, 0);
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct A {
    b: Option<B>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct B {}

#[test]
fn empty_inner_obj() {
    let a = A {
        b: Some(B {})
    };

    let de_from_default_object: A = Config::builder()
        .add_source(Config::try_from(&a).unwrap())
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap();
    assert_eq!(a, de_from_default_object); // Failed
}
