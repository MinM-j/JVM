#![allow(dead_code)]

use super::attribute::*;
use super::class_version::ClassVersion;
use super::consant_pool::*;
use super::types::*;

#[derive(Default, Debug)]
pub struct ClassFile {
    pub magic: U4,
    pub version: ClassVersion,
    pub constant_pool: Vec<ConstantInfo>,
    pub access_flags: U2,
    pub this_class: U2,
    pub super_class: U2,
    pub interfaces: Vec<U2>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn get_constant_pool_entry(&self, index: U2) -> &ConstantInfo {
        &self.constant_pool[(index - 1) as usize]
    }
}
