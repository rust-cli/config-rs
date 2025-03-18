use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::OnceLock;
use std::sync::RwLock;
use std::time::Duration;

use config::{Config, File};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};

fn settings() -> &'static RwLock<Config> {
    static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();
    CONFIG.get_or_init(|| {
        let settings = load();

        RwLock::new(settings)
    })
}

fn refresh() {
    *settings().write().unwrap() = load();
}

fn load() -> Config {
    Config::builder()
        .add_source(File::with_name("examples/watch/Settings.toml"))
        .build()
        .unwrap()
}

fn show() {
    println!(
        " * Settings :: \n\x1b[31m{:?}\x1b[0m",
        settings()
            .read()
            .unwrap()
            .clone()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );
}

fn watch() -> ! {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        notify::Config::default().with_poll_interval(Duration::from_secs(2)),
    )
    .unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(
            Path::new("examples/watch/Settings.toml"),
            RecursiveMode::NonRecursive,
        )
        .unwrap();

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(Ok(Event {
                kind: notify::event::EventKind::Modify(_),
                ..
            })) => {
                println!(" * Settings.toml written; refreshing configuration ...");
                refresh();
                show();
            }

            Err(e) => println!("watch error: {e:?}"),

            _ => {
                // Ignore event
            }
        }
    }
}

fn main() {
    // This is just an example of what could be done, today
    // We do want this to be built-in to config-rs at some point
    // Feel free to take a crack at a PR

    show();
    watch();
}
