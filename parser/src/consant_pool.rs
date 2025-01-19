use crate::access_flag::FieldFlags;

use super::access_flag::MethodFlags;
use super::attribute::*;
use super::types::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum ConstantTag {
    Class = 7,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    String = 8,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    NameAndType = 12,
    Utf8 = 1,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

impl From<u8> for ConstantTag {
    fn from(value: u8) -> Self {
        match value {
            7 => ConstantTag::Class,
            9 => ConstantTag::Fieldref,
            10 => ConstantTag::Methodref,
            11 => ConstantTag::InterfaceMethodref,
            8 => ConstantTag::String,
            3 => ConstantTag::Integer,
            4 => ConstantTag::Float,
            5 => ConstantTag::Long,
            6 => ConstantTag::Double,
            12 => ConstantTag::NameAndType,
            1 => ConstantTag::Utf8,
            15 => ConstantTag::MethodHandle,
            16 => ConstantTag::MethodType,
            17 => ConstantTag::Dynamic,
            18 => ConstantTag::InvokeDynamic,
            19 => ConstantTag::Module,
            20 => ConstantTag::Package,
            _ => panic!("Invalid Constant Pool Tag value: {}", value),
        }
    }
}
// The tag field of each variant is represented by the Enum variant
#[derive(Debug, Clone)]
pub enum ConstantInfo {
    Class(ConstantClassInfo),
    FieldRef(ConstantFieldRefInfo),
    Methodref(ConstantMethodRefInfo),
    InterfaceMethodRef(ConstantInterfaceMethodRefInfo),
    String(ConstantStringInfo),
    Integer(ConstantIntegerInfo),
    Float(ConstantFloatInfo),
    Long(ConstantLongInfo),
    Double(ConstantDoubleInfo),
    NameAndType(ConstantNameAndTypeInfo),
    Utf8(ConstantUtf8Info),
    MethodHandle(ConstantMethodHandleInfo),
    MethodType(ConstantMethodTypeInfo),
    Dynamic(ConstantDynamicInfo),
    InvokeDynamic(ConstantInvokeDynamicInfo),
    Module(ConstantModuleInfo),
    Package(ConstantPackageInfo),
}

#[derive(Debug, Clone)]
pub struct ConstantClassInfo(pub U2); //name_index

#[derive(Debug, Clone)]
pub struct ConstantFieldRefInfo {
    pub class_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug, Clone)]
pub struct ConstantMethodRefInfo {
    pub class_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug, Clone)]
pub struct ConstantInterfaceMethodRefInfo {
    pub class_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug, Clone)]
pub struct ConstantStringInfo {
    pub string_index: U2,
}

#[derive(Debug, Clone)]
pub struct ConstantIntegerInfo(pub i32);

#[derive(Debug, Clone)]
pub struct ConstantFloatInfo(pub f32);

//MAYBE use U8? instead of two U4's
#[derive(Debug, Clone)]
pub struct ConstantLongInfo(pub i64);

//MAYBE use U8? instead of two U4's
#[derive(Debug, Clone)]
pub struct ConstantDoubleInfo(pub f64);

#[derive(Debug, Clone)]
pub struct ConstantNameAndTypeInfo {
    pub name_index: U2,
    pub descriptor_index: U2,
}

// length field is removed because it is stored in Vec type
#[derive(Debug, Clone)]
pub struct ConstantUtf8Info(pub String);

#[derive(Debug, Clone)]
pub struct ConstantMethodHandleInfo {
    pub reference_kind: U1,
    pub reference_index: U2,
}

#[derive(Debug, Clone)]
pub struct ConstantMethodTypeInfo {
    pub descriptor_index: U2,
}

#[derive(Debug, Clone)]
pub struct ConstantDynamicInfo {
    pub bootstrap_method_attr_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug, Clone)]
pub struct ConstantInvokeDynamicInfo {
    pub bootstrap_method_attr_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug, Clone)]
pub struct ConstantModuleInfo {
    pub name_index: U2,
}

#[derive(Debug, Clone)]
pub struct ConstantPackageInfo {
    pub name_index: U2,
}

//Debug
#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: MethodFlags,
    pub name_index: U2,
    pub descriptor_index: U2,
    pub attributes_count: U2, // MAYBE remove it?
    pub attributes: Vec<AttributeInfo>,
}

impl<'a> MethodInfo {
    pub fn get_name(&self, cp: &'a ConstantPool) -> &'a str {
        cp.get_underlying_string_from_utf8_index(self.name_index)
            .unwrap()
            .as_str()
    }

    pub fn is_main(&self, cp: &ConstantPool) -> bool {
        if self.get_name(&cp) == "main"
            && self.access_flags.contains(MethodFlags::ACC_PUBLIC | MethodFlags::ACC_STATIC)
            && cp
                .get_underlying_string_from_utf8_index(self.descriptor_index)
                .unwrap()
                .chars()
                .last()
                .unwrap()
                == 'V'
        {
            true
        } else {
            false
        }
    }

    pub fn get_code_attribute(&self) -> Option<&Code> {
        self.attributes.iter().find_map(|attr| match attr {
            AttributeInfo::Code(code) => Some(code),
            _ => None,
        })
    }
}

#[derive(Debug)]
pub struct FieldInfo {
    //TODO change to MethodFlags
    pub access_flags: FieldFlags,
    pub name_index: U2,
    pub descriptor_index: U2,
    pub attributes_count: U2, // MAYBE remove it?
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Default)]
pub struct ConstantPool(Vec<ConstantInfo>);

impl ConstantPool {
    pub fn get_entry(&self, index: U2) -> Option<&ConstantInfo> {
        if index == 0 {
            return None;
        } else {
            self.0.get((index - 1) as usize)
        }
    }

    pub fn get_underlying_string_from_utf8_index(&self, index: U2) -> Option<&String> {
        self.get_entry(index).and_then(|cp_entry| match cp_entry {
            ConstantInfo::Utf8(ConstantUtf8Info(ref class_name)) => Some(class_name),
            _ => None,
        })
    }

    pub fn get_underlying_string_from_constant_class_info_index(
        &self,
        index: U2,
    ) -> Option<&String> {
        self.get_entry(index).and_then(|cp_entry| match cp_entry {
            &ConstantInfo::Class(ConstantClassInfo(utf8_index)) => {
                self.get_underlying_string_from_utf8_index(utf8_index)
            }
            _ => None,
        })
    }
}

impl From<Vec<ConstantInfo>> for ConstantPool {
    fn from(value: Vec<ConstantInfo>) -> Self {
        Self(value)
    }
}
