use super::class_loading_error::ClassLoadingError;
use super::classpath_entry::*;
use super::loaded_class::LoadedClass;
use crate::vm::VM;
use parser::access_flag::ClassFlags;
use parser::class_file_reader::ClassFileReader;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ClassLoader {
    loaded_classes: Arc<Mutex<HashMap<String, Arc<LoadedClass>>>>,
    class_path_entries: Vec<Box<dyn ClassPathEntry>>,
}

impl ClassLoader {
    pub fn new() -> Self {
        ClassLoader {
            loaded_classes: Arc::new(Mutex::new(HashMap::new())),
            class_path_entries: Vec::new(),
        }
    }

    pub fn add_directory_entry(&mut self, path: String) -> Result<(), ClassLoadingError> {
        self.class_path_entries
            .push(Box::new(DirectoryEntry { path }));
        Ok(())
    }

    pub fn add_jar_entry(&mut self, path: String) -> Result<(), ClassLoadingError> {
        let jar_entry = JarEntry::new(path)?;
        self.class_path_entries.push(Box::new(jar_entry));
        Ok(())
    }

    pub async fn load_class(
        &self,
        class_name: &str,
        vm: &VM,
    ) -> Result<Arc<LoadedClass>, ClassLoadingError> {
        if let Some(loaded_class) = self.find_loaded_class(class_name) {
            return Ok(loaded_class);
        }

        let class_data = self.load_class_data(class_name).await?;

        self.verify_class_format(&class_data)?;

        let parsed_class = ClassFileReader::new(class_data).parse().unwrap();

        let superclass = if class_name != "java/lang/Object" {
            match parsed_class.get_super_class_name() {
                Some(super_class_name) => {
                    let fut = Box::pin(self.load_class(super_class_name, vm));
                    match fut.await {
                        Ok(value) => Some(value),
                        Err(e) => {
                            return Err(ClassLoadingError::NoClassDefFoundError(format!(
                                "Failed to load superclass {}: {}",
                                super_class_name, e
                            )))
                        }
                    }
                }
                None => None,
            }
        } else {
            None
        };

        let mut interfaces = Vec::new();
        for interface_name in &parsed_class.get_interfaces_name() {
            let fut = Box::pin(self.load_class(interface_name, vm));
            match fut.await {
                Ok(interface) => {
                    if !interface.access_flags.contains(ClassFlags::ACC_INTERFACE) {
                        return Err(ClassLoadingError::IncompatibleClassChangeError(format!(
                            "Class {} is not an interface",
                            interface_name
                        )));
                    }
                    interfaces.push(interface);
                }
                Err(e) => {
                    return Err(ClassLoadingError::NoClassDefFoundError(format!(
                        "Failed to load interface {}: {}",
                        interface_name, e
                    )))
                }
            }
        }

        let loaded_class = Arc::new(LoadedClass::new(
            class_name.to_string(),
            superclass,
            interfaces,
            parsed_class.fields,
            parsed_class.methods,
            Arc::new(parsed_class.constant_pool),
            parsed_class.access_flags,
        ));

        self.loaded_classes
            .lock()
            .unwrap()
            .insert(class_name.to_string(), Arc::clone(&loaded_class));
        //LoadedClass::initialize(loaded_class.clone(), vm)
         //   .await
          //  .unwrap();
        //println!("{class_name} loaded");

        Ok(loaded_class)
    }

    fn find_loaded_class(&self, class_name: &str) -> Option<Arc<LoadedClass>> {
        self.loaded_classes
            .lock()
            .unwrap()
            .get(class_name)
            .map(Arc::clone)
    }

    async fn load_class_data(&self, class_name: &str) -> Result<Vec<u8>, ClassLoadingError> {
        for entry in &self.class_path_entries {
            match entry.read_class(class_name).await {
                Ok(data) => return Ok(data),
                Err(_) => continue,
            }
        }
        Err(ClassLoadingError::ClassNotFoundException(format!(
            "Could not find class {}",
            class_name
        )))
    }

    fn verify_class_format(&self, class_data: &[u8]) -> Result<(), ClassLoadingError> {
        if class_data.len() < 4 || &class_data[0..4] != &[0xCA, 0xFE, 0xBA, 0xBE] {
            return Err(ClassLoadingError::ClassFormatError(
                "Invalid class file magic number".to_string(),
            ));
        }

        if class_data.len() < 8 {
            return Err(ClassLoadingError::ClassFormatError(
                "Class file too short".to_string(),
            ));
        }

        Ok(())
    }
}
