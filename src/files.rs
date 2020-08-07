use std::io::{self, Result};
use std::path::Path;
use walkdir::DirEntry;

pub fn is_not_hidden(entry: &DirEntry) -> bool {
    !entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub fn display_path(path: &Path, prefix: &str) -> Result<String> {
    Ok(path
        .to_str()
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "File name is not valid unicode")
        })?
        .to_string()
        .replace(prefix, "")
        .trim_end_matches(".gpg")
        .to_string())
}

pub fn display_stem<'a>(path: &'a Path) -> Result<&'a str> {
    path.file_stem()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Found a file with no name"))?
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "File name is not valid unicode"))
}
