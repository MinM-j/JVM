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
    pub fn get_constant_pool_entry(&self, index: U2) -> Option<&ConstantInfo> {
        if index == 0 {
            return None;
        } else {
            self.constant_pool.get((index - 1) as usize)
        }
    }

    pub fn get_class_name(&self) -> &String {
        if let Some(class_name) =
            self.get_underlying_string_from_constant_class_info_index(self.this_class)
        {
            class_name
        } else {
            unreachable!()
        }
    }

    pub fn get_super_class_name(&self) -> Option<&String> {
        self.get_underlying_string_from_constant_class_info_index(self.super_class)
    }

    pub fn get_underlying_string_from_constant_class_info_index(
        &self,
        index: U2,
    ) -> Option<&String> {
        self.get_constant_pool_entry(index)
            .and_then(|cp_entry| match cp_entry {
                &ConstantInfo::Class(ConstantClassInfo(utf8_index)) => self
                    .get_constant_pool_entry(utf8_index)
                    .and_then(|cp_entry| match cp_entry {
                        ConstantInfo::Utf8(ConstantUtf8Info(ref class_name)) => Some(class_name),
                        _ => None,
                    }),
                _ => None,
            })
    }
}
