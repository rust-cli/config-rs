//! Use `config` as a higher-level API over `std::env::var_os`

use std::sync::OnceLock;

use config::Config;

fn main() {
    println!("{:?}", get::<String>("foo"));
}

/// Get a configuration value from the environment
pub fn get<'a, T: serde::Deserialize<'a>>(path: &str) -> T {
    // You shouldn't probably do it like that and actually handle that error that might happen
    // here, but for the sake of simplicity, we do it like this here
    config().get::<T>(path).unwrap()
}

fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        Config::builder()
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap()
    })
}
