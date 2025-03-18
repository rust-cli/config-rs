use std::sync::OnceLock;

use config::Config;

fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        Config::builder()
            .add_source(config::Environment::with_prefix("APP_NAME").separator("_"))
            .build()
            .unwrap()
    })
}

/// Get a configuration value from the static configuration object
pub fn get<'a, T: serde::Deserialize<'a>>(key: &str) -> T {
    // You shouldn't probably do it like that and actually handle that error that might happen
    // here, but for the sake of simplicity, we do it like this here
    config().get::<T>(key).unwrap()
}

fn main() {
    println!("{:?}", get::<String>("foo"));
}
