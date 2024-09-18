use super::class_entry::{ClassEntry, ClassLoadingError};
use super::{directory_entry::DirectoryEntry, jar_entry::JarEntry};
#[derive(Default, Debug)]
pub struct ClassPath {
    entries: Vec<Box<dyn ClassEntry>>,
}

#[derive(Debug)]
pub enum ClassPathError {
    Invalid(String),
}

impl ClassPath {
    pub fn add(&mut self, string: &str) -> Result<(), ClassPathError> {
        let mut entries: Vec<Box<dyn ClassEntry>> = Vec::new();
        for entry in string.split(':') {
            let parsed_entry = Self::try_parse(entry)?;
            entries.push(parsed_entry);
        }
        self.entries.append(&mut entries);
        Ok(())
    }

    fn try_parse(path: &str) -> Result<Box<dyn ClassEntry>, ClassPathError> {
        Self::try_parse_directory(path).or_else(|_| Self::try_parse_jar(path))
    }

    fn try_parse_directory(path: &str) -> Result<Box<dyn ClassEntry>, ClassPathError> {
        let entry =
            DirectoryEntry::new(path).map_err(|_| ClassPathError::Invalid(path.to_string()))?;
        Ok(Box::new(entry))
    }

    fn try_parse_jar(path: &str) -> Result<Box<dyn ClassEntry>, ClassPathError> {
        let entry = JarEntry::new(path).map_err(|_| ClassPathError::Invalid(path.to_string()))?;
        Ok(Box::new(entry))
    }

    pub fn resolve(&self, name: &str) -> Result<Option<Vec<u8>>, ClassLoadingError> {
        for entry in self.entries.iter() {
            let entry_ret = entry.resolve(name)?;
            if let Some(class_bytes) = entry_ret {
                return Ok(Some(class_bytes));
            }
        }
        Ok(None)
    }
}
