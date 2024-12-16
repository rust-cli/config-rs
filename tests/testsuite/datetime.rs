use chrono::{DateTime, TimeZone, Utc};
use config::{Config, File, FileFormat};

#[test]
#[cfg(feature = "toml")]
fn toml() {
    let s = Config::builder()
        .add_source(File::from_str(
            r#"
            toml_datetime = 2017-05-11T14:55:15Z
            "#,
            FileFormat::Toml,
        ))
        .build()
        .unwrap();

    let date: String = s.get("toml_datetime").unwrap();
    assert_eq!(&date, "2017-05-11T14:55:15Z");
    let date: DateTime<Utc> = s.get("toml_datetime").unwrap();
    assert_eq!(date, Utc.with_ymd_and_hms(2017, 5, 11, 14, 55, 15).unwrap());
}

#[test]
#[cfg(feature = "json")]
fn json() {
    let s = Config::builder()
        .add_source(File::from_str(
            r#"
            {
                "json_datetime": "2017-05-10T02:14:53Z"
            }
            "#,
            FileFormat::Json,
        ))
        .build()
        .unwrap();

    let date: String = s.get("json_datetime").unwrap();
    assert_eq!(&date, "2017-05-10T02:14:53Z");
    let date: DateTime<Utc> = s.get("json_datetime").unwrap();
    assert_eq!(date, Utc.with_ymd_and_hms(2017, 5, 10, 2, 14, 53).unwrap());
}

#[test]
#[cfg(feature = "yaml")]
fn yaml() {
    let s = Config::builder()
        .add_source(File::from_str(
            r#"
            yaml_datetime: 2017-06-12T10:58:30Z
            "#,
            FileFormat::Yaml,
        ))
        .build()
        .unwrap();

    let date: String = s.get("yaml_datetime").unwrap();
    assert_eq!(&date, "2017-06-12T10:58:30Z");
    let date: DateTime<Utc> = s.get("yaml_datetime").unwrap();
    assert_eq!(date, Utc.with_ymd_and_hms(2017, 6, 12, 10, 58, 30).unwrap());
}

#[test]
#[cfg(feature = "ini")]
fn ini() {
    let s = Config::builder()
        .add_source(File::from_str(
            r#"
                ini_datetime = 2017-05-10T02:14:53Z
            "#,
            FileFormat::Ini,
        ))
        .build()
        .unwrap();

    let date: String = s.get("ini_datetime").unwrap();
    assert_eq!(&date, "2017-05-10T02:14:53Z");
    let date: DateTime<Utc> = s.get("ini_datetime").unwrap();
    assert_eq!(date, Utc.with_ymd_and_hms(2017, 5, 10, 2, 14, 53).unwrap());
}

#[test]
#[cfg(feature = "ron")]
fn ron() {
    let s = Config::builder()
        .add_source(File::from_str(
            r#"
            (
                ron_datetime: "2021-04-19T11:33:02Z"
            )
            "#,
            FileFormat::Ron,
        ))
        .build()
        .unwrap();

    let date: String = s.get("ron_datetime").unwrap();
    assert_eq!(&date, "2021-04-19T11:33:02Z");
    let date: DateTime<Utc> = s.get("ron_datetime").unwrap();
    assert_eq!(date, Utc.with_ymd_and_hms(2021, 4, 19, 11, 33, 2).unwrap());
}
