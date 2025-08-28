use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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

        // Ideally there is an exact filename match with a format hint
        if filename.is_file() {
            return if let Some(format) = format_hint {
                Ok((filename, Box::new(format)))
            } else {
                // Without a hint, try to identify the format via the file extension
                let valid_format = {
                    let predicate = identify_format(&filename);

                    all_extensions().keys()
                        .find(|f| predicate(*f))
                        .ok_or_else(|| self.error_invalid_format())?
                };

                Ok((filename, Box::new(*valid_format)))
            };
        }

        // Without an exact filename, try to find a valid file by appending format extensions
        let mut filename = filename;

        match format_hint {
            Some(_) => {
                let valid_format = format_hint
                    .filter(file_exists_with_format(&mut filename))
                    .ok_or_else(|| self.error_invalid_path())?;

                Ok((filename, Box::new(valid_format)))
            }

            None => {
                let valid_format = {
                    let mut predicate = file_exists_with_format(&mut filename);

                    all_extensions().keys()
                        .find(|f| predicate(*f))
                        .ok_or_else(|| self.error_invalid_path())?
                };

                Ok((filename, Box::new(*valid_format)))
            }
        }
    }

    fn error_invalid_format(&self) -> Box<dyn Error + Send + Sync> {
        let error_message = format!(
            "configuration file \"{}\" is not of a registered file format",
            self.name.to_string_lossy(),
        );

        Box::new(io::Error::new(io::ErrorKind::NotFound, error_message))
    }

    fn error_invalid_path(&self) -> Box<dyn Error + Send + Sync> {
        let error_message = format!(
            "configuration file \"{}\" not found",
            self.name.to_string_lossy(),
        );

        Box::new(io::Error::new(io::ErrorKind::NotFound, error_message))
    }
}

fn identify_format<F: FileStoredFormat>(filename: &Path) -> impl Fn(&F) -> bool + '_ {
    let ext = filename.extension().unwrap_or_default().to_string_lossy();
    move |format| format.file_extensions().contains(&ext.as_ref())
}

// Provides a predicate to verify a file exists with an extension that is compatible with the queried format.
// To simplify usage at the call sites, `filename` will be mutated by reference with each use of the closure.
fn file_exists_with_format<F: FileStoredFormat>(
    filename: &mut PathBuf,
) -> impl FnMut(&F) -> bool + '_ {
    // Preserve any extension-like text within the provided file stem by appending a fake extension
    // which will be replaced by `set_extension()` calls (e.g. `example.file.placeholder` => `example.file.json`)
    if filename.extension().is_some() {
        filename.as_mut_os_string().push(".placeholder");
    }

    move |format| {
        format.file_extensions().iter().any(|ext| {
            filename.set_extension(ext);
            filename.is_file()
        })
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
