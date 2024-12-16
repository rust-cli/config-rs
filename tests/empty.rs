use config::Config;
use serde_derive::{Deserialize, Serialize};

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
