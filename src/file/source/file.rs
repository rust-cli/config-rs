use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::file::{source::FileSourceResult, FileFormat, FileSource, FileStoredFormat, Format};

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
        let path = if self.name.is_absolute() {
            self.name.clone()
        } else {
            env::current_dir()?.as_path().join(&self.name)
        };

        // First check for an _exact_ match
        if path.is_file() {
            if let Some(format) = format_hint {
                return Ok((path, Box::new(format)));
            } else {
                let ext = path.extension().unwrap_or_default().to_string_lossy();
                for format in FileFormat::all() {
                    if format.extensions().contains(&ext.as_ref()) {
                        return Ok((path, Box::new(*format)));
                    }
                }
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "configuration file \"{}\" is not of a supported file format",
                        path.to_string_lossy()
                    ),
                )));
            };
        }

        let mut path = path;
        // Preserve any extension-like text within the provided file stem by appending a fake extension
        // which will be replaced by `set_extension()` calls (e.g.  `file.local.placeholder` => `file.local.json`)
        if path.extension().is_some() {
            path.as_mut_os_string().push(".placeholder");
        }
        match format_hint {
            Some(format) => {
                for ext in format.file_extensions() {
                    path.set_extension(ext);

                    if path.is_file() {
                        return Ok((path, Box::new(format)));
                    }
                }
            }
            None => {
                for format in FileFormat::all() {
                    for ext in format.extensions() {
                        path.set_extension(ext);

                        if path.is_file() {
                            return Ok((path, Box::new(*format)));
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
