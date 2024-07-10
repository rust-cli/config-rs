use config::{Config, File, FileFormat};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum A {
    VariantA { port: u16 },
}

#[derive(Debug, Deserialize)]
struct Settings {
    a: A,
}

#[test]
fn test_ron_enum() {
    let c = Config::builder()
        .add_source(File::from_str(
            r#"
            (
                a: VariantA ( port: 5000 )
            )
            "#,
            FileFormat::Ron,
        ))
        .build()
        .unwrap();

    // Deserialize the entire file as single struct
    let s = c.try_deserialize::<Settings>();
    assert!(s.is_ok(), "Not Ok(_): {}", s.unwrap_err());
    let s = s.unwrap();
    let A::VariantA { port } = s.a;
    assert_eq!(port, 5000);
}
