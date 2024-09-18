use super::class_entry::{ClassEntry, ClassLoadingError};
use super::{directory_entry::DirectoryEntry, jar_entry::JarEntry};
use std::path::PathBuf;
#[derive(Debug)]
pub struct ClassPath {
    entries: Vec<Box<dyn ClassEntry>>,
}

impl Default for ClassPath {
    fn default() -> Self {
        let path = std::env::current_dir().unwrap();
        let entry1: Box<dyn ClassEntry> = Box::new(DirectoryEntry::new(path).unwrap());

        //This is the path in mayhem's device. Feel free to comment it out and change accordingly.
        //let mut jar_path = PathBuf::new();
        //jar_path.push("/usr/lib/jvm/java-22-openjdk/lib/jrt-fs.jar");
        //jar_path.push("/usr/lib/jvm/java-22-openjdk/lib/modules");
        //let entry2: Box<dyn ClassEntry> = Box::new(JarEntry::new(jar_path).unwrap());

        let mut home_dir = std::env::home_dir().unwrap();
        home_dir.push("./my_jar_dir/java.base/");
        //let path = PathBuf::from("~/my_jar_dir/java.base/");
        let entry2: Box<dyn ClassEntry> = Box::new(DirectoryEntry::new(home_dir).unwrap());
        let entries = vec![entry1, entry2];
        Self { entries }
    }
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
