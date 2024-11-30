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

#[test]
fn test_empty_seq() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Test {
        int: u32,
        seq: Vec<String>,
    }

    let test = Test {
        int: 1,
        seq: vec![],
    };
    let config = Config::try_from(&test).unwrap();

    let actual: Test = config.try_into().unwrap();
    assert_eq!(test, actual);
}

