use super::types::*;
use std::fmt::Debug;
use std::fmt::Formatter;

// The tag field of each variant is represented by the Enum variant
#[derive(Debug)]
pub enum ConstantInfo {
    Class(ConstantClassInfo),
    FieldRef(ConstantFieldRefInfo),
    Methodref(ConstantMethodRefInfo),
    InterfaceMethodref(ConstantInterfaceMethodRefInfo),
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

#[derive(Debug)]
pub struct ConstantClassInfo {
    pub name_index: U2,
}

#[derive(Debug)]
pub struct ConstantFieldRefInfo {
    pub class_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug)]
pub struct ConstantMethodRefInfo {
    pub class_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug)]
pub struct ConstantInterfaceMethodRefInfo {
    pub class_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug)]
pub struct ConstantStringInfo {
    pub string_index: U2,
}

#[derive(Debug)]
pub struct ConstantIntegerInfo {
    pub bytes: U4,
}

#[derive(Debug)]
pub struct ConstantFloatInfo {
    pub bytes: U4,
}

//MAYBE use U8? instead of two U4's
#[derive(Debug)]
pub struct ConstantLongInfo {
    pub high_bytes: U4,
    pub low_bytes: U4,
}

//MAYBE use U8? instead of two U4's
#[derive(Debug)]
pub struct ConstantDoubleInfo {
    pub high_bytes: U4,
    pub low_bytes: U4,
}

#[derive(Debug)]
pub struct ConstantNameAndTypeInfo {
    pub name_index: U2,
    pub descriptor_index: U2,
}

// length field is removed because it is stored in Vec type
pub struct ConstantUtf8Info {
    pub bytes: Vec<U1>,
}

impl Debug for ConstantUtf8Info {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("ConstantUtf8Info")
            .field("bytes", &String::from_utf8_lossy(&self.bytes))
            .finish()
    }
}

#[derive(Debug)]
pub struct ConstantMethodHandleInfo {
    pub reference_kind: U1,
    pub reference_index: U2,
}

#[derive(Debug)]
pub struct ConstantMethodTypeInfo {
    pub descriptor_index: U2,
}

#[derive(Debug)]
pub struct ConstantDynamicInfo {
    pub bootstrap_method_attr_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug)]
pub struct ConstantInvokeDynamicInfo {
    pub bootstrap_method_attr_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug)]
pub struct ConstantModuleInfo {
    pub name_index: U2,
}

#[derive(Debug)]
pub struct ConstantPackageInfo {
    pub name_index: U2,
}

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
pub struct AttributeInfo {
    pub attribute_name_index: U2,
    //attribute_length: U4, //length of info in bytes removed
    pub info: Vec<U1>,
}

impl Debug for AttributeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("AttributeInfo")
            .field("attribute_name_index", &self.attribute_name_index)
            .field("info ", &String::from_utf8_lossy(&self.info))
            .finish()
    }
}

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
