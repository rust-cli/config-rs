use config::Config;
#[derive(Debug, Default, serde::Deserialize, PartialEq, Eq)]
struct AppConfig {
    list: Vec<String>,
}

fn main() {
    std::env::set_var("APP_LIST", "Hello World");

    let config = Config::builder()
        .add_source(
            config::Environment::with_prefix("APP")
                .try_parsing(true)
                .separator("_")
                .list_separator(" "),
        )
        .build()
        .unwrap();

    let app: AppConfig = config.try_deserialize().unwrap();

    assert_eq!(app.list, vec![String::from("Hello"), String::from("World")]);

    std::env::remove_var("APP_LIST");
}
