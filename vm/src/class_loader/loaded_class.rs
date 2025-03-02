use crate::jvm_error::JVMError;
use crate::runtime::*;
use crate::vm::VM;
use parser::access_flag::*;
use parser::attribute::Code;
use parser::constant_pool::{ConstantNameAndTypeInfo, ConstantPool, FieldInfo, MethodInfo};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InitState {
    Uninitialized,
    InProgress,
    Initialized,
}

#[derive(Debug)]
pub struct LoadedClass {
    pub class_name: String,
    pub super_class: Option<Arc<LoadedClass>>,
    pub interfaces: Vec<Arc<LoadedClass>>,
    pub instance_fields: Vec<FieldInfo>,
    pub instance_fields_indices: HashMap<String, usize>,
    pub instance_fields_descriptors: HashMap<usize, String>,
    pub static_fields: Vec<FieldInfo>,
    pub static_values: RefCell<Vec<Value>>,
    pub static_field_indices: HashMap<String, usize>,
    pub methods: Vec<MethodInfo>,
    pub constant_pool: Arc<ConstantPool>,
    pub access_flags: ClassFlags,
    pub code_cache: Mutex<HashMap<NameDes, Arc<Code>>>,
    pub init_state: Mutex<InitState>,
}

impl LoadedClass {
    pub fn new(
        class_name: String,
        super_class: Option<Arc<LoadedClass>>,
        interfaces: Vec<Arc<LoadedClass>>,
        fields: Vec<FieldInfo>,
        methods: Vec<MethodInfo>,
        constant_pool: Arc<ConstantPool>,
        access_flags: ClassFlags,
    ) -> Self {
        let (instance_fields, static_fields): (Vec<FieldInfo>, Vec<FieldInfo>) = fields
            .into_iter()
            .partition(|f| !f.access_flags.contains(FieldFlags::ACC_STATIC));
        let static_field_indices: HashMap<String, usize> = static_fields
            .iter()
            .enumerate()
            .filter_map(|(i, f)| {
                constant_pool
                    .get_underlying_string_from_utf8_index(f.name_index)
                    .map(|name| (name.clone(), i))
            })
            .collect();
        let mut all_instance_fields = Vec::new();
        if let Some(ref super_class) = super_class {
            all_instance_fields.extend(super_class.instance_fields.iter().cloned());
        }
        all_instance_fields.extend(instance_fields);
        let mut field_indices = HashMap::new();
        let mut indices_des = HashMap::new();
        let mut slot_index = 0;
        if let Some(ref super_class) = super_class {
            for field in &super_class.instance_fields {
                let name = super_class
                    .constant_pool
                    .get_underlying_string_from_utf8_index(field.name_index)
                    .expect("Invalid name_index in superclass constant pool");
                let des = super_class
                    .constant_pool
                    .get_underlying_string_from_utf8_index(field.descriptor_index)
                    .expect("Invalid descriptor_index in superclass constant pool");
                field_indices.insert(name.to_string(), slot_index);
                indices_des.insert(slot_index, des.to_string());
                slot_index += 1;
            }
        }
        for field in &all_instance_fields[slot_index..] {
            let name = constant_pool
                .get_underlying_string_from_utf8_index(field.name_index)
                .expect("Invalid name_index in constant pool");
            let des = constant_pool
                .get_underlying_string_from_utf8_index(field.descriptor_index)
                .expect("Invalid descriptor_index in constant pool");
            field_indices.insert(name.to_string(), slot_index);
            indices_des.insert(slot_index, des.to_string());
            slot_index += 1;
        }
        let static_values = static_fields
            .iter()
            .map(|field| {
                let descriptor = constant_pool
                    .get_underlying_string_from_utf8_index(field.descriptor_index)
                    .expect("Invalid descriptor_index in constant pool");
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

        LoadedClass {
            class_name,
            super_class,
            interfaces,
            instance_fields: all_instance_fields,
            instance_fields_indices: field_indices,
            instance_fields_descriptors: indices_des,
            static_fields,
            static_values: RefCell::new(static_values),
            static_field_indices,
            methods,
            constant_pool,
            access_flags,
            code_cache: Mutex::new(HashMap::new()),
            init_state: Mutex::new(InitState::Uninitialized),
        }
    }

    pub fn get_method_info_from_name_and_descriptor(
        &self,
        name_des: &NameDes,
    ) -> Option<&MethodInfo> {
        self.methods.iter().find(|method| {
            method.get_name(&self.constant_pool) == name_des.name
                && method.get_des(&self.constant_pool) == name_des.des
        })
    }

    pub fn get_code_from_method(&self, name_des: &NameDes) -> Option<Arc<Code>> {
        let mut cache = self.code_cache.lock().unwrap();
        if let Some(cached_code) = cache.get(&name_des) {
            return Some(Arc::clone(cached_code));
        }

        let temp_code = self
            .get_method_info_from_name_and_descriptor(&name_des)
            .and_then(|method_info| method_info.get_code_attribute());

        if let Some(code) = temp_code {
            let arc_code = Arc::new(code.clone());
            cache.insert(name_des.clone(), Arc::clone(&arc_code));
            Some(arc_code)
        } else {
            None
        }
    }

    pub async fn initialize(self_class: Arc<LoadedClass>, vm: &VM) -> Result<(), JVMError> {
        let mut state = self_class.init_state.lock().unwrap();
        match *state {
            InitState::Initialized => return Ok(()),
            InitState::InProgress => {
                return Err(JVMError::Other(
                    "Circular initialization detected".to_string(),
                ))
            }
            InitState::Uninitialized => {
                *state = InitState::InProgress;

                if let Some(super_class) = &self_class.super_class {
                    let fut = Box::pin(LoadedClass::initialize(super_class.clone(), vm));
                    fut.await?;
                }

                let clinit_name_des = NameDes {
                    name: "<clinit>".to_string(),
                    des: "()V".to_string(),
                };
                if let Some(code) = self_class.get_code_from_method(&clinit_name_des) {
                    let frame = Frame::new(self_class.clone(), &clinit_name_des, code);
                    let mut stack = Stack::new();
                    stack.push_frame(frame)?;
                    stack.execute_current_frame(vm).await?;
                }
                *state = InitState::Initialized;
                Ok(())
            }
        }
    }

    pub fn get_static_field(&self, name: &str) -> Result<Value, JVMError> {
        let mut current = Some(self);
        while let Some(cls) = current {
            if let Some(&index) = cls.static_field_indices.get(name) {
                let static_values = cls.static_values.borrow();
                return Ok(static_values[index].clone());
            }
            current = cls.super_class.as_deref();
        }
        Err(JVMError::Other(format!(
            "Static field {} not found in class hierarchy",
            name
        )))
    }

    pub fn set_static_field(&self, name: &str, value: Value) -> Result<(), JVMError> {
        let mut current = Some(self);
        while let Some(cls) = current {
            if let Some(&index) = cls.static_field_indices.get(name) {
                let mut static_values = cls.static_values.borrow_mut();
                static_values[index] = value;
                return Ok(());
            }
            current = cls.super_class.as_deref();
        }
        Err(JVMError::Other(format!(
            "Static field {} not found in class hierarchy",
            name
        )))
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct NameDes {
    pub name: String,
    pub des: String,
}

impl NameDes {
    pub fn new(argu: &ConstantNameAndTypeInfo, cp: &ConstantPool) -> Self {
        Self {
            name: cp
                .get_underlying_string_from_utf8_index(argu.name_index)
                .unwrap()
                .clone(),
            des: cp
                .get_underlying_string_from_utf8_index(argu.descriptor_index)
                .unwrap()
                .clone(),
        }
    }
}
