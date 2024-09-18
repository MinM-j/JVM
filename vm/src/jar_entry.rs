use super::class_entry::{ClassEntry, ClassLoadingError};
use std::{
    cell::RefCell,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};
use zip::{result::ZipError, ZipArchive};

#[derive(Debug)]
pub struct JarEntry {
    name: String,
    zip: RefCell<ZipArchive<BufReader<File>>>,
}

impl JarEntry {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self, JarError> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(JarError::NotFound(path.to_string_lossy().to_string()));
        }
        let file = File::open(path)
            .map_err(|_| JarError::ReadingError(path.to_string_lossy().to_string()))?;
        let reader = BufReader::new(file);
        let zip = ZipArchive::new(reader)
            .map_err(|_| JarError::InvalidJar(path.to_string_lossy().to_string()))?;
        Ok(Self {
            name: path.to_string_lossy().to_string(),
            zip: RefCell::new(zip),
        })
    }
}

#[derive(Debug)]
pub enum JarError {
    NotFound(String),
    ReadingError(String),
    InvalidJar(String),
}

impl ClassEntry for JarEntry {
    fn resolve(&self, class_name: &str) -> Result<Option<Vec<u8>>, ClassLoadingError> {
        let name = class_name.to_string() + ".class";
        match self.zip.borrow_mut().by_name(&name) {
            Ok(mut zip_file) => {
                let mut buffer: Vec<u8> = Vec::with_capacity(zip_file.size() as usize);
                zip_file
                    .read_to_end(&mut buffer)
                    .map_err(ClassLoadingError::new)?;
                Ok(Some(buffer))
            }
            Err(err) => match err {
                ZipError::FileNotFound => Ok(None),
                _ => Err(ClassLoadingError::new(err)),
            },
        }
    }
}
