#![allow(dead_code)]

use super::access_flag::ClassFlags;
use super::attribute::*;
use super::class_version::ClassVersion;
use super::constant_pool::*;
use super::types::*;

#[derive(Default, Debug)]
pub struct ClassFile {
    pub magic: U4,
    pub version: ClassVersion,
    pub constant_pool: ConstantPool,
    pub access_flags: ClassFlags,
    pub this_class: U2,
    pub super_class: U2,
    pub interfaces: Vec<U2>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn get_constant_pool_entry(&self, index: U2) -> Option<&ConstantInfo> {
        self.constant_pool.get_entry(index)
    }

    pub fn get_class_name(&self) -> &String {
        if let Some(class_name) = self
            .constant_pool
            .get_underlying_string_from_constant_class_info_index(self.this_class)
        {
            class_name
        } else {
            unreachable!()
        }
    }

    pub fn get_super_class_name(&self) -> Option<&String> {
        self.constant_pool
            .get_underlying_string_from_constant_class_info_index(self.super_class)
    }

    pub fn get_interfaces_name(&self) -> Vec<&String> {
        self.interfaces
            .iter()
            .map(|i| {
                self.constant_pool
                    .get_underlying_string_from_constant_class_info_index(*i)
            })
            .flatten()
            .collect::<Vec<&String>>()
    }
}
