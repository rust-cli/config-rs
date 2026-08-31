use config::{Config, File};
use serde_derive::Deserialize;
use std::sync::OnceLock;
use std::sync::RwLock;


#[derive(Default, Clone, Deserialize)]
struct Cred {
    user: String,
    key: String,
}

#[derive(Default, Clone, Deserialize)]
pub struct Settings {
    verbose: Option<u8>,
    cred: Option<Cred>,
}


// This function defines the static settings storage.
fn settings() -> &'static RwLock<Settings> {
    static SETTINGS: OnceLock<RwLock<Settings>> = OnceLock::new();
    SETTINGS.get_or_init(|| RwLock::new(Settings::default()))
}

fn build_config(file: &str) -> Settings {
    let s = Config::builder()
        // Configuration file
        .add_source(File::with_name(file).required(false))
        .build()
        .expect("Config build failed");

    // Deserialize (and thus freeze) the entire configuration
    s.try_deserialize().unwrap()
}

impl Settings {
    // This associated function replaces previous settings values with a newly
    // loaded ones.
    //
    // It is mainly intended for loading the values from config file to replace
    // the plain default used for static allocation. Thus running it once at
    // the beginning of the program execution when the config files are known.
    //
    // But a later call to this function may be used to update the settings,
    // for example, when the config file changes during the execution and you want
    // to sync with it (signal/notify/whatever based reload).
    pub fn init(cfgfile: Option<&str>) {
        let file = cfgfile.unwrap_or("config.toml");

        let mut new_settings = settings().write().unwrap();
        *new_settings = build_config(file);
    }

    // Following associated functions are just getters, when you want to keep
    // the Settings structure members private.
    pub fn user() -> Result<String, String> {
        match &settings().read().unwrap().cred {
            Some(c) => Ok(c.user.clone()),
            None => Err("Credential config is missing".to_string()),
        }
    }

    pub fn key() -> Result<String, String> {
        match &settings().read().unwrap().cred {
            Some(c) => Ok(c.key.clone()),
            None => Err("Credential config is missing".to_string()),
        }
    }

    pub fn verbosity() -> u8 {
        settings().read().unwrap().verbose.unwrap_or(0)
    }

    // It is not a problem to make all the Settings structure members public and
    // then only create here one function, that will return a read reference
    // to the Settings. This may be useful if you want to omit the getters and
    // the settings contain just plain values that don't need any error
    // handling.
    //
    // Example:
    // pub fn new() -> Self {
    //      settings().read().unwrap()
    // }
}

