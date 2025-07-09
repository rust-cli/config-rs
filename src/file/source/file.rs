use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::file::{
    format::all_extensions, source::FileSourceResult, FileSource, FileStoredFormat, Format,
};

/// Describes a file sourced from a file
#[derive(Clone, Debug)]
pub struct FileSourceFile {
    /// Path of configuration file
    name: PathBuf,
}

impl FileSourceFile {
    pub fn new(name: PathBuf) -> Self {
        Self { name }
    }

    fn find_file<F>(
        &self,
        format_hint: Option<F>,
    ) -> Result<(PathBuf, Box<dyn Format>), Box<dyn Error + Send + Sync>>
    where
        F: FileStoredFormat + Format + 'static,
    {
        let filename = if self.name.is_absolute() {
            self.name.clone()
        } else {
            env::current_dir()?.as_path().join(&self.name)
        };

        // First check for an _exact_ match
        if filename.is_file() {
            return if let Some(format) = format_hint {
                Ok((filename, Box::new(format)))
            } else {
                for (format, extensions) in all_extensions().iter() {
                    if extensions.contains(
                        &filename
                            .extension()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .as_ref(),
                    ) {
                        return Ok((filename, Box::new(*format)));
                    }
                }

                Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "configuration file \"{}\" is not of a registered file format",
                        filename.to_string_lossy()
                    ),
                )))
            };
        }
        // Adding a dummy extension will make sure we will not override secondary extensions, i.e. "file.local"
        // This will make the following set_extension function calls to append the extension.
        let mut filename = add_dummy_extension(filename);

        match format_hint {
            Some(format) => {
                for ext in format.file_extensions() {
                    filename.set_extension(ext);

                    if filename.is_file() {
                        return Ok((filename, Box::new(format)));
                    }
                }
            }

            None => {
                for format in all_extensions().keys() {
                    for ext in format.extensions() {
                        filename.set_extension(ext);

                        if filename.is_file() {
                            return Ok((filename, Box::new(*format)));
                        }
                    }
                }
            }
        }

        Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "configuration file \"{}\" not found",
                self.name.to_string_lossy()
            ),
        )))
    }
}

impl<F> FileSource<F> for FileSourceFile
where
    F: Format + FileStoredFormat + 'static,
{
    fn resolve(
        &self,
        format_hint: Option<F>,
    ) -> Result<FileSourceResult, Box<dyn Error + Send + Sync>> {
        // Find file
        let (filename, format) = self.find_file(format_hint)?;

        // Attempt to use a relative path for the URI
        let uri = env::current_dir()
            .ok()
            .and_then(|base| pathdiff::diff_paths(&filename, base))
            .unwrap_or_else(|| filename.clone());

        // Read contents from file
        let buf = fs::read(filename)?;

        // If it exists, skip the UTF-8 BOM byte sequence: EF BB BF
        let buf = if buf.len() >= 3 && &buf[0..3] == b"\xef\xbb\xbf" {
            &buf[3..]
        } else {
            &buf
        };

        let c = String::from_utf8_lossy(buf);
        let text = c.into_owned();

        Ok(FileSourceResult {
            uri: Some(uri.to_string_lossy().into_owned()),
            content: text,
            format,
        })
    }
}

fn add_dummy_extension(mut filename: PathBuf) -> PathBuf {
    match filename.extension() {
        Some(extension) => {
            let mut ext = extension.to_os_string();
            ext.push(".");
            ext.push("dummy");
            filename.set_extension(ext);
        }
        None => {
            filename.set_extension("dummy");
        }
    }
    filename
}
