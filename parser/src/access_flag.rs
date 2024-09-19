use super::types::U2;
use bitflags::bitflags;

#[derive(Debug)]
pub enum AccessFlags {
    PUBLIC = 0x0001,
    FINAL = 0x0010,
    SUPER = 0x0020,
    INTERFACE = 0x0200,
    ABSTRACT = 0x0400,
    SYNTHETIC = 0x1000,
    ANNOTATION = 0x2000,
    ENUM = 0x4000,
    MODULE = 0x8000,
}

bitflags! {
    #[derive(Debug)]
    pub struct MethodFlags: U2{
        const ACC_PUBLIC = 0x0001;
    const ACC_PRIVATE = 0x0002;
    const ACC_PROTECTED = 0x0004;
    const ACC_STATIC = 0x0008;
    const ACC_FINAL = 0x0010;
    const ACC_SYNCHRONIZED = 0x0020;
    const ACC_BRIDGE = 0x0040;
    const ACC_VARARGS = 0x0080;
    const ACC_NATIVE = 0x0100;
    const ACC_ABSTRACT = 0x0400;
    const ACC_STRICT = 0x0800;
    const ACC_SYNTHETIC = 0x1000;
}
}
