use parser::{
    access_flag::AccessFlags, consant_pool::ConstantPool, consant_pool::FieldInfo,
    consant_pool::MethodInfo, types::*,
};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ClassId {
    id: u32,
}

impl ClassId {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
    pub fn id_as_u32(&self) -> u32 {
        self.id
    }
}

#[derive(Debug)]
pub struct Class<'a> {
    pub id: ClassId,
    pub name: String,
    pub constants: ConstantPool,
    //pub flags: AccessFlags,
    pub flags: U2, //TODO use AccessFlags
    pub superclass: Option<ClassRef<'a>>,
    pub interfaces: Vec<ClassRef<'a>>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub first_field_index: usize,
    pub total_fields: usize,
}

pub type ClassRef<'a> = &'a Class<'a>;
