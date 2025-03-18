mod format;
pub(crate) mod source;

use std::fmt::Debug;
use std::path::{Path, PathBuf};

use self::source::FileSource;
use crate::error::{ConfigError, Result};
use crate::map::Map;
use crate::source::Source;
use crate::value::Value;
use crate::Format;

pub use self::format::FileFormat;
pub use self::source::file::FileSourceFile;
pub use self::source::string::FileSourceString;

/// An extension of [`Format`] trait.
///
/// Associates format with file extensions, therefore linking storage-agnostic notion of format to a file system.
pub trait FileStoredFormat: Format {
    /// Returns a vector of file extensions, for instance `[yml, yaml]`.
    fn file_extensions(&self) -> &'static [&'static str];
}

/// A configuration source backed up by a file.
///
/// It supports optional automatic file format discovery.
#[derive(Clone, Debug)]
#[must_use]
pub struct File<T, F> {
    source: T,

    /// Format of file (which dictates what driver to use).
    format: Option<F>,

    /// A required File will error if it cannot be found
    required: bool,
}

impl<F> File<FileSourceString, F>
where
    F: FileStoredFormat + 'static,
{
    pub fn from_str(s: &str, format: F) -> Self {
        Self {
            format: Some(format),
            required: true,
            source: s.into(),
        }
    }
}

impl<F> File<FileSourceFile, F>
where
    F: FileStoredFormat + 'static,
{
    pub fn new(name: &str, format: F) -> Self {
        Self {
            format: Some(format),
            required: true,
            source: FileSourceFile::new(name.into()),
        }
    }
}

impl File<FileSourceFile, FileFormat> {
    /// Given the basename of a file, will attempt to locate a file by setting its
    /// extension to a registered format.
    pub fn with_name(base_name: &str) -> Self {
        Self {
            format: None,
            required: true,
            source: FileSourceFile::new(base_name.into()),
        }
    }
}

impl<T, F> File<T, F>
where
    F: FileStoredFormat + 'static,
    T: FileSource<F>,
{
    pub fn format(mut self, format: F) -> Self {
        self.format = Some(format);
        self
    }

    /// Set required to false to make a file optional when building the config.
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

impl<'a> From<&'a Path> for File<FileSourceFile, FileFormat> {
    fn from(path: &'a Path) -> Self {
        Self {
            format: None,
            required: true,
            source: FileSourceFile::new(path.to_path_buf()),
        }
    }
}

impl From<PathBuf> for File<FileSourceFile, FileFormat> {
    fn from(path: PathBuf) -> Self {
        Self {
            format: None,
            required: true,
            source: FileSourceFile::new(path),
        }
    }
}

impl<T, F> Source for File<T, F>
where
    F: FileStoredFormat + Debug + Clone + Send + Sync + 'static,
    T: Sync + Send + FileSource<F> + 'static,
{
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<Map<String, Value>> {
        // Coerce the file contents to a string
        let (uri, contents, format) = match self
            .source
            .resolve(self.format.clone())
            .map_err(ConfigError::Foreign)
        {
            Ok(result) => (result.uri, result.content, result.format),

            Err(error) => {
                if !self.required {
                    return Ok(Map::new());
                }

                return Err(error);
            }
        };

        // Parse the string using the given format
        format
            .parse(uri.as_ref(), &contents)
            .map_err(|cause| ConfigError::FileParse { uri, cause })
    }
}
