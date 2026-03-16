use std::error::Error;

use crate::{
    Format,
    file::source::FileSourceResult,
    file::{FileSource, FileStoredFormat},
};

/// Describes a file sourced from a string
#[derive(Clone, Debug)]
pub struct FileSourceString(String);

impl<'a> From<&'a str> for FileSourceString {
    fn from(s: &'a str) -> Self {
        Self(s.into())
    }
}

impl<F> FileSource<F> for FileSourceString
where
    F: Format + FileStoredFormat + 'static,
{
    fn resolve(
        &self,
        format_hint: Option<F>,
    ) -> Result<FileSourceResult, Box<dyn Error + Send + Sync>> {
        Ok(FileSourceResult {
            uri: None,
            content: self.0.clone(),
            format: Box::new(format_hint.expect("from_str requires a set file format")),
        })
    }
}
