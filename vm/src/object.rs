use super::class_loader::loaded_class::LoadedClass;
use super::jvm_error::JVMError;
use super::runtime::Value;
use super::vm::VM;
use std::cell::RefCell;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

static OBJECT_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Debug)]
pub struct ObjectHeader {
    pub mark: bool,
    pub generation: u8,
    pub ref_count: u32,
    pub object_id: u32,
}

impl ObjectHeader {
    pub fn new() -> Self {
        let id = OBJECT_ID.fetch_add(1, Ordering::Relaxed);
        //println!("{id}");
        ObjectHeader {
            mark: false,
            generation: 0,
            ref_count: 1,
            object_id: id,
        }
    }
}

#[derive(Debug)]
pub enum ObjectKind {
    ClassInstance {
        fields: RefCell<Vec<Value>>,
    },
    ArrayInstance {
        length: usize,
        elements: RefCell<Vec<Value>>,
        element_type: String,
    },
}

#[derive(Debug)]
pub struct Object {
    pub class: Option<Arc<LoadedClass>>,
    pub header: RefCell<ObjectHeader>,
    pub kind: ObjectKind,
    pub monitor: Arc<Mutex<()>>,
}

impl Object {
    pub async fn new_class(class: Arc<LoadedClass>, vm: &VM) -> Self {
        //LoadedClass::initialize(class.clone(), vm).await.unwrap();
        let fields = class
            .instance_fields
            .iter()
            .enumerate()
            .map(|(index, _field)| {
                let descriptor = class
                    .instance_fields_descriptors
                    .get(&index)
                    .expect("Field descriptor not found in instance_fields_descriptors");
                match descriptor.as_str() {
                    "Z" => Value::Int(0),
                    "B" => Value::Int(0),
                    "C" => Value::Int(0),
                    "S" => Value::Int(0),
                    "I" => Value::Int(0),
                    "J" => Value::Long(0),
                    "F" => Value::Float(0.0),
                    "D" => Value::Double(0.0),
                    d if d.starts_with("L") || d.starts_with("[") => Value::Reference(None),
                    _ => panic!("Unknown descriptor type: {}", descriptor),
                }
            })
            .collect::<Vec<_>>();
        Object {
            class: Some(class),
            header: RefCell::new(ObjectHeader::new()),
            kind: ObjectKind::ClassInstance {
                fields: RefCell::new(fields),
            },
            monitor: Arc::new(Mutex::new(())),
        }
    }

    pub fn get_field(&self, name: &str) -> Result<Value, JVMError> {
        if let ObjectKind::ClassInstance { fields } = &self.kind {
            let class = self
                .class
                .as_ref()
                .ok_or_else(|| JVMError::Other("No class for instance".to_string()))?;
            let index = class
                .instance_fields_indices
                .get(name)
                .ok_or_else(|| JVMError::Other(format!("Instance field {} not found", name)))?;
            Ok(fields.borrow()[*index].clone())
        } else {
            Err(JVMError::Other("Field access on array object".to_string()))
        }
    }

    pub fn set_field(&self, name: &str, value: Value) -> Result<(), JVMError> {
        if let ObjectKind::ClassInstance { fields } = &self.kind {
            let class = self
                .class
                .as_ref()
                .ok_or_else(|| JVMError::Other("No class for instance".to_string()))?;
            let index = class
                .instance_fields_indices
                .get(name)
                .ok_or_else(|| JVMError::Other(format!("Instance field {} not found", name)))?;
            fields.borrow_mut()[*index] = value;
            Ok(())
        } else {
            Err(JVMError::Other("Field access on array object".to_string()))
        }
    }

    pub fn new_array(class: Option<Arc<LoadedClass>>, length: usize, element_type: &str) -> Self {
        let elements = vec![Value::Default; length];
        Object {
            class,
            header: RefCell::new(ObjectHeader::new()),
            kind: ObjectKind::ArrayInstance {
                length,
                elements: RefCell::new(elements),
                element_type: element_type.to_string(),
            },
            monitor: Arc::new(Mutex::new(())),
        }
    }

    pub fn get_element(&self, index: usize) -> Result<Value, JVMError> {
        if let ObjectKind::ArrayInstance {
            elements, length, ..
        } = &self.kind
        {
            if index < *length {
                Ok(elements.borrow()[index].clone())
            } else {
                Err(JVMError::IndexOutOfBounds {
                    index,
                    max: *length,
                })
            }
        } else {
            Err(JVMError::Other(
                "Element access on class instance".to_string(),
            ))
        }
    }

    pub fn set_element(&self, index: usize, value: Value) -> Result<(), JVMError> {
        if let ObjectKind::ArrayInstance {
            elements, length, ..
        } = &self.kind
        {
            if index < *length {
                elements.borrow_mut()[index] = value;
                Ok(())
            } else {
                Err(JVMError::IndexOutOfBounds {
                    index,
                    max: *length,
                })
            }
        } else {
            Err(JVMError::Other(
                "Element access on class instance".to_string(),
            ))
        }
    }
}
