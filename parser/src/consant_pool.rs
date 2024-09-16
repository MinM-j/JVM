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
pub struct ConstantClassInfo {
    pub name_index: U2,
}

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
    pub access_flags: U2,
    pub name_index: U2,
    pub descriptor_index: U2,
    pub attributes_count: U2, // MAYBE remove it?
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: U2,
    pub name_index: U2,
    pub descriptor_index: U2,
    pub attributes_count: U2, // MAYBE remove it?
    pub attributes: Vec<AttributeInfo>,
}
