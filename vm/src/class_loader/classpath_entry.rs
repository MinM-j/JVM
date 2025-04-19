use super::class_loading_error::ClassLoadingError;
use async_trait::async_trait;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use zip::ZipArchive;

#[async_trait]
pub trait ClassPathEntry: Send + Sync {
    async fn read_class(&self, class_name: &str) -> Result<Vec<u8>, ClassLoadingError>;
}

// Directory-based class loading
pub struct DirectoryEntry {
    pub path: String,
}

#[async_trait]
impl ClassPathEntry for DirectoryEntry {
    async fn read_class(&self, class_name: &str) -> Result<Vec<u8>, ClassLoadingError> {
        let class_file_path = class_name.to_string() + ".class";
        let full_path = Path::new(&self.path).join(&class_file_path);
       // println!("{:?}",full_path);
        fs::read(&full_path).map_err(|e| ClassLoadingError::IoError(e))
    }
}

// JAR file class loading
pub struct JarEntry {
    path: String,
    archive: Arc<Mutex<ZipArchive<fs::File>>>,
}

impl JarEntry {
    pub fn new(path: String) -> Result<Self, ClassLoadingError> {
        let file = fs::File::open(&path).map_err(|e| ClassLoadingError::IoError(e))?;
        let archive =
            ZipArchive::new(file).map_err(|e| ClassLoadingError::InvalidJarFile(e.to_string()))?;

        Ok(JarEntry {
            path,
            archive: Arc::new(Mutex::new(archive)),
        })
    }
}

#[async_trait]
impl ClassPathEntry for JarEntry {
    async fn read_class(&self, class_name: &str) -> Result<Vec<u8>, ClassLoadingError> {
        let class_file_path = if self.path.as_str() == base {
            "classes/".to_string() + class_name + ".class"
        } else {
            class_name.to_string() + ".class"
        };
        let mut archive = self.archive.lock().unwrap();

        let mut entry = archive
            .by_name(&class_file_path)
            .map_err(|e| ClassLoadingError::ClassNotFoundException(e.to_string()))?;

        let mut buffer = Vec::new();
        entry
            .read_to_end(&mut buffer)
            .map_err(|e| ClassLoadingError::IoError(e))?;

        Ok(buffer)
    }
}
const base: &str = "/usr/lib/jvm/java-24-openjdk/jmods/java.base.jmod";
