#![allow(dead_code)]

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;

pub type U1 = u8;
pub type U2 = u16;
pub type U4 = u32;

#[derive(Default)]
pub struct ClassFile {
    magic: U4,
    minor_version: U2,
    major_version: U2,
    constant_pool_count: U2, // valid interval: [1, constant_pool_count-1]
    constant_pool: Vec<ConstantInfo>,
    access_flags: U2,
    this_class: U2,
    super_class: U2,
    interfaces_count: U2,
    interfaces: Vec<U2>,
    fields_count: U2,
    //field_info fields[fields_count];
    methods_count: U2,
    //method_info methods[methods_count];
    attributes_count: U2,
    //attribute_info attributes[attributes_count]
}

impl ClassFile {
    pub fn parse(mut stream: File) -> Self {
        let magic = Self::parse_magic(&mut stream);
        dbg!(magic);
        let minor_version = Self::parse_minor_version(&mut stream);
        dbg!(minor_version);
        let major_version = Self::parse_major_version(&mut stream);
        dbg!(major_version);
        let constant_pool_count = Self::parse_constant_pool_count(&mut stream);
        dbg!(constant_pool_count);
        let constant_pool = Self::parse_constant_pool(&mut stream, constant_pool_count);
        dbg!(constant_pool);
        let access_flags = Self::parse_access_flags(&mut stream);
        dbg!(access_flags);
        let this_class = Self::parse_this_class(&mut stream);
        dbg!(this_class);
        let super_class = Self::parse_super_class(&mut stream);
        dbg!(super_class);
        let interfaces_count = Self::parse_interfaces_count(&mut stream);
        dbg!(interfaces_count);
        let interfaces = Self::parse_interfaces(&mut stream, interfaces_count);
        dbg!(interfaces);
        let fields_count = Self::parse_field_count(&mut stream);
        dbg!(fields_count);
        //let fields = Self::parse_fields(&mut stream, fields_count);
        //dbg!(fields);

        Self::default()
    }

    fn parse_magic(stream: &mut File) -> U4 {
        Self::read_u4(stream)
    }

    fn parse_minor_version(stream: &mut File) -> U2 {
        Self::read_u2(stream)
    }
    fn parse_major_version(stream: &mut File) -> U2 {
        Self::read_u2(stream)
    }

    fn parse_constant_pool_count(stream: &mut File) -> U2 {
        Self::read_u2(stream)
    }

    fn parse_constant_pool(stream: &mut File, constant_pool_count: U2) -> Vec<ConstantInfo> {
        let mut constant_pool: Vec<ConstantInfo> = Vec::with_capacity(constant_pool_count as usize);

        // TODO need to  skip loop variable for long and double
        // works currently if there is no such constant info
        // check using `javap -v *.class`
        for _ in 1..constant_pool_count {
            constant_pool.push(Self::parse_constant_info(stream));
        }

        constant_pool
    }

    fn parse_constant_info(stream: &mut File) -> ConstantInfo {
        let tag = Self::read_u1(stream);

        match ConstantTag::from(tag) {
            ConstantTag::Methodref => {
                ConstantInfo::Methodref(ConstantMethodrefInfo::from_file(stream))
            }

            ConstantTag::Class => ConstantInfo::Class(ConstantClassInfo::from_file(stream)),
            ConstantTag::NameAndType => {
                ConstantInfo::NameAndType(ConstantNameAndTypeInfo::from_file(stream))
            }
            ConstantTag::Utf8 => ConstantInfo::Utf8(ConstantUtf8Info::from_file(stream)),
            ConstantTag::Fieldref => {
                ConstantInfo::FieldRef(ConstantFieldRefInfo::from_file(stream))
            }
            ConstantTag::String => ConstantInfo::String(ConstantStringInfo::from_file(stream)),
            _ => todo!(),
        }
    }

    fn parse_access_flags(stream: &mut File) -> U2 {
        // TODO maybe verify validity of access_flags
        Self::read_u2(stream)
    }

    fn parse_this_class(stream: &mut File) -> U2 {
        Self::read_u2(stream)
    }

    fn parse_super_class(stream: &mut File) -> U2 {
        Self::read_u2(stream)
    }

    fn parse_interfaces_count(stream: &mut File) -> U2 {
        Self::read_u2(stream)
    }

    //Each value in the interfaces array must be a valid index into
    //the constant_pool table. The constant_pool entry at each value
    //of interfaces[i], where 0 ≤ i < interfaces_count, must be a
    //CONSTANT_Class_info structure representing an interface that is a direct
    //superinterface of this class or interface type, in the left-to-right order given in
    //the source for the type
    fn parse_interfaces(stream: &mut File, interfaces_count: U2) -> Vec<U2> {
        let mut interfaces: Vec<U2> = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            //TODO verify interface index(must point to CONSTANT_Class_info) in constant pool table)
            interfaces.push(Self::parse_interface(stream));
        }
        interfaces
    }

    fn parse_interface(stream: &mut File) -> U2 {
        Self::read_u2(stream)
    }

    fn parse_field_count(stream: &mut File) -> U2 {
        Self::read_u2(stream)
    }

    //fn parse_fields(stream: &mut File, fields_count: U2) {}

    // todo refactor read_u<n> functions
    fn read_u1(stream: &mut File) -> U1 {
        let mut buf: [U1; 1] = [0; 1];
        match stream.read_exact(&mut buf[..]) {
            Ok(()) => buf[0],
            Err(err) => panic!("error: {}", err),
        }
    }

    fn read_u2(stream: &mut File) -> U2 {
        let mut buf: [U1; 2] = [0; 2];
        match stream.read_exact(&mut buf[..]) {
            Ok(()) => U2::from_be_bytes(buf),
            Err(err) => panic!("error: {}", err),
        }
    }

    fn read_u4(stream: &mut File) -> U4 {
        let mut buf: [U1; 4] = [0; 4];
        match stream.read_exact(&mut buf[..]) {
            Ok(()) => U4::from_be_bytes(buf),
            Err(err) => panic!("error: {}", err),
        }
    }

    fn read_n_bytes(stream: &mut File, n: usize) -> Vec<U1> {
        let mut buf = vec![0u8; n];
        match stream.read_exact(&mut buf[..]) {
            Ok(()) => buf,
            Err(err) => panic!("error: {}", err),
        }
    }
}

/// Type implementing this trait will build themselves from input file stream
/// TODO: make generic if required
pub trait FromFile: Sized {
    fn from_file(stream: &mut File) -> Self;
}

// The tag field of each variant is represented by the Enum variant
#[derive(Debug)]
pub enum ConstantInfo {
    Class(ConstantClassInfo),
    FieldRef(ConstantFieldRefInfo),
    Methodref(ConstantMethodrefInfo),
    InterfaceMethodref(ConstantInterfaceMethodrefInfo),
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
    name_index: U2,
}
impl FromFile for ConstantClassInfo {
    fn from_file(stream: &mut File) -> Self {
        let name_index = ClassFile::read_u2(stream);
        Self { name_index }
    }
}

#[derive(Debug)]
pub struct ConstantFieldRefInfo {
    class_index: U2,
    name_and_type_index: U2,
}

impl FromFile for ConstantFieldRefInfo {
    fn from_file(stream: &mut File) -> Self {
        let class_index = ClassFile::read_u2(stream);
        let name_and_type_index = ClassFile::read_u2(stream);
        Self {
            class_index,
            name_and_type_index,
        }
    }
}

#[derive(Debug)]
pub struct ConstantMethodrefInfo {
    class_index: U2,
    name_and_type_index: U2,
}

impl FromFile for ConstantMethodrefInfo {
    fn from_file(stream: &mut File) -> Self {
        let class_index = ClassFile::read_u2(stream);
        let name_and_type_index = ClassFile::read_u2(stream);
        Self {
            class_index,
            name_and_type_index,
        }
    }
}

#[derive(Debug)]
pub struct ConstantInterfaceMethodrefInfo {
    class_index: U2,
    name_and_type_index: U2,
}

#[derive(Debug)]
pub struct ConstantStringInfo {
    string_index: U2,
}

impl FromFile for ConstantStringInfo {
    fn from_file(stream: &mut File) -> Self {
        let string_index = ClassFile::read_u2(stream);
        Self { string_index }
    }
}

#[derive(Debug)]
pub struct ConstantIntegerInfo {
    bytes: U4,
}

#[derive(Debug)]
pub struct ConstantFloatInfo {
    bytes: U4,
}

//MAYBE use U8? instead of two U4's
#[derive(Debug)]
pub struct ConstantLongInfo {
    high_bytes: U4,
    low_bytes: U4,
}

//MAYBE use U8? instead of two U4's
#[derive(Debug)]
pub struct ConstantDoubleInfo {
    high_bytes: U4,
    low_bytes: U4,
}

#[derive(Debug)]
pub struct ConstantNameAndTypeInfo {
    name_index: U2,
    descriptor_index: U2,
}
impl FromFile for ConstantNameAndTypeInfo {
    fn from_file(stream: &mut File) -> Self {
        let name_index = ClassFile::read_u2(stream);
        let descriptor_index = ClassFile::read_u2(stream);
        Self {
            name_index,
            descriptor_index,
        }
    }
}

// length field is removed because it is stored in Vec type
pub struct ConstantUtf8Info {
    bytes: Vec<U1>,
}

impl Debug for ConstantUtf8Info {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("ConstantUtf8Info")
            .field("bytes", &String::from_utf8_lossy(&self.bytes))
            .finish()
    }
}
impl FromFile for ConstantUtf8Info {
    fn from_file(stream: &mut File) -> Self {
        let length = ClassFile::read_u2(stream);
        let bytes = ClassFile::read_n_bytes(stream, length as usize);
        Self { bytes }
    }
}

#[derive(Debug)]
pub struct ConstantMethodHandleInfo {
    reference_kind: U1,
    reference_index: U2,
}

#[derive(Debug)]
pub struct ConstantMethodTypeInfo {
    descriptor_index: U2,
}

#[derive(Debug)]
pub struct ConstantDynamicInfo {
    bootstrap_method_attr_index: U2,
    name_and_type_index: U2,
}

#[derive(Debug)]
pub struct ConstantInvokeDynamicInfo {
    bootstrap_method_attr_index: U2,
    name_and_type_index: U2,
}

#[derive(Debug)]
pub struct ConstantModuleInfo {
    name_index: U2,
}

#[derive(Debug)]
pub struct ConstantPackageInfo {
    name_index: U2,
}

pub struct FieldInfo {
    access_flags: U2,
    name_index: U2,
    descriptor_index: U2,
    attributes_count: U2,
    attributes: Vec<AttributeInfo>,
}

pub struct AttributeInfo {
    attribute_name_index: U2,
    attribute_length: U4,
    info: Vec<U1>,
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
