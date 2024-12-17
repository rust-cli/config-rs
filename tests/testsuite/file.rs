use snapbox::{assert_data_eq, str};

use config::{Config, File, FileFormat};

#[test]
#[cfg(feature = "json")]
fn test_file_not_required() {
    let res = Config::builder()
        .add_source(File::new("tests/testsuite/file-nonexistent", FileFormat::Json).required(false))
        .build();

    assert!(res.is_ok());
}

#[test]
#[cfg(feature = "json")]
fn test_file_required_not_found() {
    let res = Config::builder()
        .add_source(File::new(
            "tests/testsuite/file-nonexistent",
            FileFormat::Json,
        ))
        .build();

    assert!(res.is_err());
    assert_data_eq!(
        res.unwrap_err().to_string(),
        str![[r#"configuration file "tests/testsuite/file-nonexistent" not found"#]]
    );
}

#[test]
#[cfg(feature = "json")]
fn test_file_auto() {
    let c = Config::builder()
        .add_source(File::with_name("tests/testsuite/file-auto"))
        .build()
        .unwrap();

    assert_eq!(c.get("debug").ok(), Some(true));
    assert_eq!(c.get("production").ok(), Some(false));
}

#[test]
#[cfg(feature = "json")]
fn test_file_auto_not_found() {
    let res = Config::builder()
        .add_source(File::with_name("tests/testsuite/file-nonexistent"))
        .build();

    assert!(res.is_err());
    assert_data_eq!(
        res.unwrap_err().to_string(),
        str![[r#"configuration file "tests/testsuite/file-nonexistent" not found"#]]
    );
}

#[test]
#[cfg(feature = "json")]
fn test_file_ext() {
    let c = Config::builder()
        .add_source(File::with_name("tests/testsuite/file-ext.json"))
        .build()
        .unwrap();

    assert_eq!(c.get("debug").ok(), Some(true));
    assert_eq!(c.get("production").ok(), Some(false));
}

#[test]
#[cfg(feature = "json")]
fn test_file_second_ext() {
    let c = Config::builder()
        .add_source(File::with_name("tests/testsuite/file-second-ext.default"))
        .build()
        .unwrap();

    assert_eq!(c.get("debug").ok(), Some(true));
    assert_eq!(c.get("production").ok(), Some(false));
}
