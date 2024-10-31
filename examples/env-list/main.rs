use config::Config;

#[derive(Debug, Default, serde_derive::Deserialize, PartialEq, Eq)]
struct ListOfStructs {
    a: String,
    b: String,
}

#[derive(Debug, Default, serde_derive::Deserialize, PartialEq, Eq)]
struct AppConfig {
    list: Vec<String>,
    structs: Vec<Option<ListOfStructs>>,
}

fn main() {
    std::env::set_var("APP_LIST", "Hello World");
    std::env::set_var("APP_STRUCTS_0_A", "Hello");
    std::env::set_var("APP_STRUCTS_0_B", "World");
    std::env::set_var("APP_STRUCTS_2_A", "foo");
    std::env::set_var("APP_STRUCTS_2_B", "bar");

    let config = Config::builder()
        .add_source(
            config::Environment::with_prefix("APP")
                .try_parsing(true)
                .separator("_")
                .list_separator(" ")
                .with_list_parse_key("list"),
        )
        .build()
        .unwrap();

    let app: AppConfig = config.try_deserialize().unwrap();

    assert_eq!(app.list, vec![String::from("Hello"), String::from("World")]);
    assert_eq!(
        app.structs,
        vec![
            Some(ListOfStructs {
                a: String::from("Hello"),
                b: String::from("World")
            }),
            None,
            Some(ListOfStructs {
                a: String::from("foo"),
                b: String::from("bar")
            }),
        ]
    );

    std::env::remove_var("APP_LIST");
}
