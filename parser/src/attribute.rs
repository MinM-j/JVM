#![allow(dead_code)]
use super::types::*;
//use std::fmt;

#[derive(Debug)]
pub enum AttributeInfo {
    Attribute(RemainingAttribute),
    ConstantValue(ConstantValue),
    LineNumberTable(LineNumberTable),
    Code(Code),
    BootstrapMethod(BootstrapMethod),
}
#[derive(Debug)]
pub struct RemainingAttribute {
    pub attribute_name_index: U2,
    //attribute_length: U4, //length of info in bytes removed
    pub info: Vec<U1>,
}

//impl fmt::Debug for AttributeInfo {
//fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
//f.debug_struct("AttributeInfo")
//.field("attribute_name_index", &self.attribute_name_index)
//.field("info ", &String::from_utf8_lossy(&self.info))
//.finish()
//}
//}

/// referred by attributes of FieldInfo structure
/// specification structure:
/// pub attribute_name_index: U2, // it is always ConstantValue
/// attribute_length: U4, // it is always 2
/// pub constant_value_index: U2, //store as struct tuple
#[derive(Debug)]
pub struct ConstantValue(pub U2); //constant_value_index

/// Code_attribute {
/// u2 attribute_name_index; // specified from type
/// u4 attribute_length;  // only needed while parsing
/// u2 max_stack;
/// u2 max_locals;
/// u4 code_length;
/// u1 code[code_length];
/// u2 exception_table_length;
/// { u2 start_pc;
/// u2 end_pc;
/// u2 handler_pc;
/// u2 catch_type;
/// } exception_table[exception_table_length];
/// u2 attributes_count;
/// attribute_info attributes[attributes_count];
/// }
#[derive(Debug)]
pub struct Code {
    //pub attribute_name_index: U2,
    //pub attribute_length: U4,
    pub max_stack: U2,
    pub max_locals: U2,
    pub code: Vec<U1>,                             //code_length
    pub exception_table: Vec<ExceptionTableEntry>, //exception_table_count
    pub attributes: Vec<AttributeInfo>,            //attributes_count
}

#[derive(Debug)]
pub struct ExceptionTableEntry {
    pub start_pc: U2,
    pub end_pc: U2,
    pub handler_pc: U2,
    pub catch_type: U2,
}

#[derive(Debug)]
pub struct LineNumberTable(pub Vec<LineNumberTableEntry>); // line_number_table_count

#[derive(Debug)]
pub struct LineNumberTableEntry {
    pub start_pc: U2,
    pub line_number: U2,
}

//MORE https://stackoverflow.com/a/25110513
// TODO later this is needed for verification
#[derive(Debug)]
pub struct StackMapTable(Vec<StackMapFrame>);

#[derive(Debug)]
pub struct StackMapFrame {}

#[derive(Debug)]
pub struct BootstrapMethod(pub Vec<BootstrapMethodEntry>);

#[derive(Debug)]
pub struct BootstrapMethodEntry {
    pub bootstrap_method_ref: U2,
    pub bootstrap_args: Vec<U2>,
}
