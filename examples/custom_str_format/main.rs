use config::{Config, File, FileStoredFormat, Format, Map, Value, ValueKind};

fn main() {
    let config = Config::builder()
        .add_source(File::from_str("bad", MyFormat))
        .add_source(File::from_str("good", MyFormat))
        .build();

    match config {
        Ok(cfg) => println!("A config: {cfg:#?}"),
        Err(e) => println!("An error: {e}"),
    }
}

#[derive(Debug, Clone)]
pub struct MyFormat;

impl Format for MyFormat {
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<Map<String, Value>, Box<dyn std::error::Error + Send + Sync>> {
        // Let's assume our format is somewhat malformed, but this is fine
        // In real life anything can be used here - nom, serde or other.
        //
        // For some more real-life examples refer to format implementation within the library code
        let mut result = Map::new();

        if text == "good" {
            result.insert(
                "key".to_owned(),
                Value::new(uri, ValueKind::String(text.into())),
            );
        } else {
            println!("Something went wrong in {uri:?}");
        }

        Ok(result)
    }
}

// As strange as it seems for config sourced from a string, legacy demands its sacrifice
// It is only required for File source, custom sources can use Format without caring for extensions
static MY_FORMAT_EXT: Vec<&'static str> = vec![];
impl FileStoredFormat for MyFormat {
    fn file_extensions(&self) -> &'static [&'static str] {
        &MY_FORMAT_EXT
    }
}
