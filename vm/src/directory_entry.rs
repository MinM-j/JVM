use super::class_entry::{ClassEntry, ClassLoadingError};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct DirectoryEntry {
    base: PathBuf,
}

impl DirectoryEntry {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self, InvalidDirectory> {
        let mut base = PathBuf::new();
        base.push(path);
        if !base.exists() || !base.is_dir() {
            Err(InvalidDirectory {
                path: base.to_string_lossy().to_string(),
            })
        } else {
            Ok(Self { base })
        }
    }
}

#[derive(Debug)]
pub struct InvalidDirectory {
    path: String,
}

impl ClassEntry for DirectoryEntry {
    fn resolve(&self, class_name: &str) -> Result<Option<Vec<u8>>, ClassLoadingError> {
        let mut path = self.base.clone();
        path.push(class_name);
        path.set_extension("class");
        if path.exists() {
            std::fs::read(path).map(Some).map_err(ClassLoadingError::new)
        }
        else {
            Ok(None)
        }
    }
}
