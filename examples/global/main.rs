#![allow(deprecated)]
use std::error::Error;
use std::sync::OnceLock;
use std::sync::RwLock;

use config::Config;

fn settings() -> &'static RwLock<Config> {
    static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();
    CONFIG.get_or_init(|| RwLock::new(Config::default()))
}

fn try_main() -> Result<(), Box<dyn Error>> {
    // Set property
    settings().write()?.set("property", 42)?;

    // Get property
    println!("property: {}", settings().read()?.get::<i32>("property")?);

    Ok(())
}

fn main() {
    try_main().unwrap();
}
