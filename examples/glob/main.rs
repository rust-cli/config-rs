use std::collections::HashMap;
use std::path::Path;

use config::{Config, File};
use glob::glob;

fn main() {
    // Option 1
    // --------
    // Gather all conf files from conf/ manually
    let settings = Config::builder()
        // File::with_name(..) is shorthand for File::from(Path::new(..))
        .add_source(File::with_name("examples/glob/conf/00-default.toml"))
        .add_source(File::from(Path::new("examples/glob/conf/05-some.yml")))
        .add_source(File::from(Path::new("examples/glob/conf/99-extra.json")))
        .build()
        .unwrap();

    // Print out our settings (as a HashMap)
    println!(
        "\n{:?} \n\n-----------",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );

    // Option 2
    // --------
    // Gather all conf files from conf/ manually, but put in 1 merge call.
    let settings = Config::builder()
        .add_source(vec![
            File::with_name("examples/glob/conf/00-default.toml"),
            File::from(Path::new("examples/glob/conf/05-some.yml")),
            File::from(Path::new("examples/glob/conf/99-extra.json")),
        ])
        .build()
        .unwrap();

    // Print out our settings (as a HashMap)
    println!(
        "\n{:?} \n\n-----------",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );

    // Option 3
    // --------
    // Gather all conf files from conf/ using glob and put in 1 merge call.
    let settings = Config::builder()
        .add_source(
            glob("examples/glob/conf/*")
                .unwrap()
                .map(|path| File::from(path.unwrap()))
                .collect::<Vec<_>>(),
        )
        .build()
        .unwrap();

    // Print out our settings (as a HashMap)
    println!(
        "\n{:?} \n\n-----------",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );
}
