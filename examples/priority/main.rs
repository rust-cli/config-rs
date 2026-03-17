//! Gather all conf files from conf.d/ using glob and put in 1 merge call.

use std::collections::HashMap;

use config::{Config, File};
use glob::glob;

fn main() {
    // Glob results are sorted, ensuring user priority is preserved
    let files = glob("examples/glob/conf.d/*")
        .unwrap()
        .map(|path| File::from(path.unwrap()))
        .collect::<Vec<_>>();
    let settings = Config::builder().add_source(files).build().unwrap();

    // Print out our settings (as a HashMap)
    println!(
        "\n{:?} \n\n-----------",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );
}
