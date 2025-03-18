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
