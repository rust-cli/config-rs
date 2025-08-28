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

                // NOTE: Temporary ownership/burrow issue workaround for `filename` that
                // generates a closure per format, this will be resolved in a follow-up commit.
                let predicate = |f| identify_format(&filename)(f);

                for format in all_extensions().keys() {
                    if predicate(format) {
                        return Ok((filename, Box::new(*format)));
                    }
                }

                Err(self.error_invalid_format())
            };
        }

        // Without an exact filename, try to find a valid file by appending format extensions
        let mut filename = filename;
        // Preserve any extension-like text within the provided file stem by appending a fake extension
        // which will be replaced by `set_extension()` calls (e.g.  `file.local.placeholder` => `file.local.json`)
        if filename.extension().is_some() {
            filename.as_mut_os_string().push(".placeholder");
        }

        match format_hint {
            Some(format) => {
                // This generated predicate will mutate `filename` per format extension tried
                // NOTE: Calling the returned closure in the conditional avoids upsetting the borrow checker.
                if file_exists_with_format(&mut filename)(&format) {
                    return Ok((filename, Box::new(format)));
                }
            }

            None => {
                // NOTE: Temporary ownership/burrow issue workaround for `filename` that
                // generates a closure per format, this will be resolved in a follow-up commit.
                //
                // This generated predicate will mutate `filename` per format extension tried each call
                let mut predicate = |f| file_exists_with_format(&mut filename)(f);

                for format in all_extensions().keys() {
                    if predicate(format) {
                        return Ok((filename, Box::new(*format)));
                    }
                }
            }
        }

        Err(self.error_invalid_path())
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
