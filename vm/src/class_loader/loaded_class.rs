use parser::access_flag::ClassFlags;
use parser::consant_pool::{ConstantPool, FieldInfo, MethodInfo};
use std::sync::Arc;

#[derive(Debug)]
pub struct LoadedClass {
    pub class_name: String,
    pub super_class: Option<Arc<LoadedClass>>,
    pub interfaces: Vec<Arc<LoadedClass>>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub constant_pool: ConstantPool,
    pub access_flags: ClassFlags,
}
