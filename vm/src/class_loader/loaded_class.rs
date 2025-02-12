use parser::access_flag::ClassFlags;
use parser::attribute::Code;
use parser::constant_pool::{ConstantNameAndTypeInfo, ConstantPool, FieldInfo, MethodInfo};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct LoadedClass {
    pub class_name: String,
    pub super_class: Option<Arc<LoadedClass>>,
    pub interfaces: Vec<Arc<LoadedClass>>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub constant_pool: Arc<ConstantPool>,
    pub access_flags: ClassFlags,

    pub code_cache: Mutex<HashMap<NameDes, Arc<Code>>>,
}

impl LoadedClass {
    pub fn get_method_info_from_name_and_descriptor(
        &self,
        name_des: &NameDes,
    ) -> Option<&MethodInfo> {
        self.methods.iter().find(|method| {
            method.get_name(&self.constant_pool) == name_des.name
                && method.get_des(&self.constant_pool) == name_des.des
        })
    }

    pub fn get_code_from_method(&self, name_des: &NameDes) -> Arc<Code> {
        let mut cache = self.code_cache.lock().unwrap();
        if let Some(cached_code) = cache.get(&name_des) {
            return Arc::clone(cached_code);
        }

        let temp_code = self
            .get_method_info_from_name_and_descriptor(&name_des)
            .and_then(|method_info| method_info.get_code_attribute())
            .expect("Method doesn't have code");

        let arc_code = Arc::new(temp_code.clone());

        cache.insert(name_des.clone(), Arc::clone(&arc_code));

        arc_code
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
            des: cp.get_underlying_string_from_utf8_index(argu.descriptor_index)
                .unwrap()
                .clone(),
        }
    }
}
