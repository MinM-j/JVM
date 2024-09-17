use super::class_entry::ClassEntry;
use super::{directory_entry, jar_entry};
#[derive(Debug)]
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
        todo!()
    }
    fn try_parse_jar(path: &str) -> Result<Box<dyn ClassEntry>, ClassPathError> {
        todo!()
    }
}
