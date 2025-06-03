pub(crate) mod file;
pub(crate) mod string;

use std::error::Error;
use std::fmt::Debug;

use crate::{file::FileStoredFormat, Format};

/// Describes where the [`File`][super::File] is sourced
pub trait FileSource<T>: Debug + Clone
where
    T: Format + FileStoredFormat,
{
    fn resolve(
        &self,
        format_hint: Option<T>,
    ) -> Result<FileSourceResult, Box<dyn Error + Send + Sync>>;
}

#[allow(unnameable_types)] // Unsure if/how to expose this
pub struct FileSourceResult {
    pub(crate) uri: Option<String>,
    pub(crate) content: String,
    pub(crate) format: Box<dyn Format>,
}

impl FileSourceResult {
    pub fn uri(&self) -> &Option<String> {
        &self.uri
    }

    pub fn content(&self) -> &str {
        self.content.as_str()
    }

    pub fn format(&self) -> &dyn Format {
        self.format.as_ref()
    }
}
