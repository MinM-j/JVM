use super::class_entry::{ClassEntry, ClassLoadingError};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct DirectoryEntry {
    base: PathBuf,
}

impl DirectoryEntry {
    pub fn new() -> DirectoryEntry {
        todo!()
    }
}
