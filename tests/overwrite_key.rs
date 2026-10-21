use config::{Config, File, Environment};
use serde::{Serialize,  Deserialize};

const CONFIG: &str = r#"
name = "foo"
[v4]
ca_path = "ca"
cert_path = "bar"
key_path = "baz"
"#;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub name: String,
    pub v4: TlsConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TlsConfig {
    pub ca_path: String,
    #[serde(alias = "certpath")]
    pub cert_path: String,
    // #[serde(alias = "keypath")]
    pub key_path: String,
}

#[test]
fn overwrite_key() {
    std::env::set_var("V4_CERTPATH", "Hello World");

    let s = Config::builder()
        .add_source(File::from_str(CONFIG, config::FileFormat::Toml))
        .add_source(Environment::default().separator("_"))
        .build();

    assert!(s.is_ok(), "build failed: {:?}", s);
    let s = s.unwrap();

    let v: Result<Settings, _> = s.try_deserialize();

    // This is expected to error because the key `certpath` is specified by ENV and `cert_path`
    // from the TOML.
    // This should work, but does not because of the way this crate deserializes into T.
    //
    // The fix is to name the "TlsConfig::cert_path" field "TlsConfig::certpath".
    assert!(v.is_err(), "accidentially ok: {:?}", v);
}
