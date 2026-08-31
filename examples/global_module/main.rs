mod settings;
mod cred;

use crate::settings::Settings;
use crate::cred::list_cred;

fn main() {
    // init the config module
    Settings::init(Some("examples/global_module/config/default.toml"));

    // now your config may be used anywhere in the code where you are able to
    // use "crate::settings" or "super::settings".
    let verbosity = Settings::verbosity();

    if verbosity > 0 {
        println!("Hello world, verbosity setting is greater than 0");
    }

    list_cred();
}
