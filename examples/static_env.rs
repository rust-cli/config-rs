//! Use `config` as a higher-level API over `std::env::var_os`

use std::sync::OnceLock;

use config::Config;

fn main() {
    println!("APP_STRING={:?}", get::<String>("string"));
    println!("APP_INT={:?}", get::<i32>("int"));
    println!("APP_STRLIST={:?}", get::<Vec<i32>>("strlist"));
}

/// Get a configuration value from the environment
pub fn get<'a, T: serde::Deserialize<'a>>(path: &str) -> Option<T> {
    config().get::<T>(path).ok()
}

fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        Config::builder()
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .unwrap()
    })
}
