use std::io::{Error, ErrorKind};

use config::{Config, File, FileStoredFormat, Format, Map, Value, ValueKind};

/// The private and public key sources will be read into their associated variable:
#[derive(serde_derive::Deserialize, Clone, Debug)]
pub struct Settings {
    pub private_key: Option<String>,
    pub public_key: Option<String>,
}

fn main() {
    // Sourcing from two separate files for the `Settings` struct,:
    let file_public_key = File::new("examples/custom_file_format/files/public.pem", PemFile);
    let file_private_key = File::new("examples/custom_file_format/files/private.pem", PemFile);

    // Provide the sources and build the config object:
    // Both are marked as optional to avoid failure if the file doesn't exist.
    let settings = Config::builder()
        .add_source(file_public_key.required(false))
        .add_source(file_private_key.required(false))
        .build()
        .unwrap();

    // Deserialize the config object into your Settings struct:
    let settings: Settings = settings.try_deserialize().unwrap();
    println!("{settings:#?}");
}

#[derive(Debug, Clone)]
pub struct PemFile;

impl Format for PemFile {
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<Map<String, Value>, Box<dyn std::error::Error + Send + Sync>> {
        // Store any valid keys into this map, they'll be merged with other sources into the final config map:
        let mut result = Map::new();

        // Identify the PEM encoded data type by the first occurrence found:
        // NOTE: This example is kept simple, multiple or other encoded types are not handled.
        let key_type = vec!["PUBLIC", "PRIVATE"]
            .into_iter()
            .find(|s| text.contains(s));
        let key = match key_type {
            Some("PRIVATE") => "private_key",
            Some("PUBLIC") => "public_key",
            // Otherwise fail with an error message (the filename is implicitly appended):
            _ => {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidData,
                    "PEM file did not contain a Private or Public key",
                )))
            }
        };

        result.insert(
            key.to_owned(),
            Value::new(uri, ValueKind::String(text.into())),
        );

        Ok(result)
    }
}

// A slice of extensions associated to this format, when an extension
// is omitted from a file source, these will be tried implicitly:
impl FileStoredFormat for PemFile {
    fn file_extensions(&self) -> &'static [&'static str] {
        &["pem"]
    }
}
