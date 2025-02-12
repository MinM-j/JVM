use bitflags::Flags;

use super::types::*;
use super::{
    access_flag::*,
    attribute::*,
    class_file::ClassFile,
    class_version::ClassVersion,
    constant_pool::*,
    instruction::{Instruction, Operation},
};

use std::io::Cursor;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;

#[derive(Default)]
pub struct ClassFileReader {
    buf: Cursor<Vec<u8>>,
    class_file: ClassFile,
}

impl ClassFileReader {
    pub fn new(buf: Vec<u8>) -> Self {
        Self {
            buf: Cursor::new(buf),
            class_file: ClassFile::default(),
        }
    }

    pub fn parse(mut self) -> Result<ClassFile> {
        self.parse_magic()?;
        self.parse_version()?;
        self.parse_constant_pool()?;
        self.parse_access_flags()?;
        self.parse_this_class()?;
        self.parse_super_class()?;
        self.parse_interfaces()?;
        self.parse_fields()?;
        self.parse_methods()?;
        let attributes = self.parse_attributes()?;
        self.class_file.attributes = attributes;
        Ok(self.class_file)
    }

    // this is necessary while reading Code section
    fn read_u1_with_count(&mut self, count: &mut U4) -> Result<U1> {
        *count += 1;
        self.read_u1()
    }

    fn read_u2_with_count(&mut self, count: &mut U4) -> Result<U2> {
        *count += 2;
        self.read_u2()
    }

    fn read_u1(&mut self) -> Result<U1> {
        let mut buf: [U1; 1] = [0; 1];
        self.buf.read_exact(&mut buf[..])?;
        Ok(U1::from_be_bytes(buf))
    }

    fn read_u2(&mut self) -> Result<U2> {
        let mut buf: [U1; 2] = [0; 2];

        self.buf.read_exact(&mut buf[..])?;
        Ok(U2::from_be_bytes(buf))
    }

    fn read_u4(&mut self) -> Result<U4> {
        let mut buf: [U1; 4] = [0; 4];
        self.buf.read_exact(&mut buf[..])?;
        Ok(U4::from_be_bytes(buf))
    }

    fn read_n_bytes(&mut self, n: usize) -> Result<Vec<U1>> {
        let mut buf = vec![0u8; n];
        self.buf.read_exact(&mut buf[..])?;
        Ok(buf)
    }

    fn read_u2_vec(&mut self) -> Result<Vec<U2>> {
        let item_count = self.read_u2()?;
        let mut items: Vec<U2> = Vec::with_capacity(item_count as usize);
        for _ in 0..item_count {
            let item = self.read_u2()?;
            items.push(item);
        }
        Ok(items)
    }

    // parse
    fn parse_magic(&mut self) -> Result<()> {
        self.class_file.magic = self.read_u4()?;
        Ok(())
    }

    fn parse_version(&mut self) -> Result<()> {
        let minor = self.read_u2()?;
        let major = self.read_u2()?;
        self.class_file.version = ClassVersion::from((minor, major));
        Ok(())
    }

    fn parse_constant_pool(&mut self) -> Result<()> {
        let constants_count = self.read_u2()?;
        // TODO define and use interfaces from ConstantPool
        let mut constant_pool: Vec<ConstantInfo> = Vec::with_capacity(constants_count as usize);

        //[1..constants_count - 1]
        //long and double takes two slot
        let mut i = 1;
        while i < constants_count {
            let constant = self.parse_constant()?;
            constant_pool.push(constant.clone());
            match constant {
                ConstantInfo::Long(_) | ConstantInfo::Double(_) => {
                    i += 2;
                    constant_pool.push(constant)
                }
                _ => i += 1,
            }
        }
        self.class_file.constant_pool = constant_pool.into();
        Ok(())
    }

    fn parse_constant(&mut self) -> Result<ConstantInfo> {
        let tag = self.read_u1()?;
        let constant = match ConstantTag::from(tag) {
            ConstantTag::Class => self.parse_constant_class()?,
            ConstantTag::Fieldref => self.parse_constant_field_ref()?,
            ConstantTag::Methodref => self.parse_constant_method_ref()?,
            ConstantTag::InterfaceMethodref => self.parse_constant_interface_method_ref()?,
            ConstantTag::String => self.parse_constant_string()?,
            ConstantTag::Integer => self.parse_constant_integer()?,
            ConstantTag::Float => self.parse_constant_float()?,
            ConstantTag::Long => self.parse_constant_long()?,
            ConstantTag::Double => self.parse_constant_double()?,
            ConstantTag::NameAndType => self.parse_constant_name_and_type()?,
            ConstantTag::Utf8 => self.parse_constant_utf8()?,
            ConstantTag::MethodHandle => self.parse_constant_method_handle()?,
            ConstantTag::MethodType => self.parse_constant_method_type()?,
            ConstantTag::Dynamic => self.parse_constant_dynamic()?,
            ConstantTag::InvokeDynamic => self.parse_constant_invoke_dynamic()?,
            ConstantTag::Module => self.parse_constant_module()?,
            ConstantTag::Package => self.parse_constant_package()?,
        };

        Ok(constant)
    }

    fn parse_access_flags(&mut self) -> Result<()> {
        let flags = self.read_u2()?;
        self.class_file.access_flags = ClassFlags::from_bits(flags).unwrap();
        Ok(())
    }

    fn parse_this_class(&mut self) -> Result<()> {
        let flags = self.read_u2()?;
        self.class_file.this_class = flags;
        Ok(())
    }

    fn parse_super_class(&mut self) -> Result<()> {
        let flags = self.read_u2()?;
        self.class_file.super_class = flags;
        Ok(())
    }

    fn parse_interfaces(&mut self) -> Result<()> {
        let interfaces = self.read_u2_vec()?;
        self.class_file.interfaces = interfaces;
        Ok(())
    }

    fn parse_fields(&mut self) -> Result<()> {
        let fields_count = self.read_u2()?;
        let mut fields: Vec<FieldInfo> = Vec::with_capacity(fields_count as usize);
        for _ in 0..fields_count {
            let field = self.parse_field_info()?;
            fields.push(field);
        }
        self.class_file.fields = fields;
        Ok(())
    }

    fn parse_methods(&mut self) -> Result<()> {
        let methods_count = self.read_u2()?;

        let mut methods: Vec<MethodInfo> = Vec::with_capacity(methods_count as usize);
        for _ in 0..methods_count {
            let method = self.parse_method_info()?;
            methods.push(method);
        }
        self.class_file.methods = methods;
        Ok(())
    }

    fn parse_attributes(&mut self) -> Result<Vec<AttributeInfo>> {
        let attributes_count = self.read_u2()?;
        let mut attributes: Vec<AttributeInfo> = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            let attribute = self.parse_attribute_info()?;
            attributes.push(attribute);
        }
        Ok(attributes)
    }

    //
    fn parse_constant_class(&mut self) -> Result<ConstantInfo> {
        let name_index = self.read_u2()?;
        Ok(ConstantInfo::Class(ConstantClassInfo(name_index)))
    }

    fn parse_constant_method_ref(&mut self) -> Result<ConstantInfo> {
        let class_index = self.read_u2()?;
        let name_and_type_index = self.read_u2()?;
        Ok(ConstantInfo::Methodref(ConstantMethodRefInfo {
            class_index,
            name_and_type_index,
        }))
    }

    fn parse_constant_field_ref(&mut self) -> Result<ConstantInfo> {
        let class_index = self.read_u2()?;
        let name_and_type_index = self.read_u2()?;
        {}
        Ok(ConstantInfo::FieldRef(ConstantFieldRefInfo {
            class_index,
            name_and_type_index,
        }))
    }

    fn parse_constant_interface_method_ref(&mut self) -> Result<ConstantInfo> {
        let class_index = self.read_u2()?;
        let name_and_type_index = self.read_u2()?;
        Ok(ConstantInfo::InterfaceMethodRef(
            ConstantInterfaceMethodRefInfo {
                class_index,
                name_and_type_index,
            },
        ))
    }

    fn parse_constant_string(&mut self) -> Result<ConstantInfo> {
        let string_index = self.read_u2()?;
        Ok(ConstantInfo::String(ConstantStringInfo { string_index }))
    }

    fn parse_constant_integer(&mut self) -> Result<ConstantInfo> {
        let value = self.read_u4()? as i32;
        Ok(ConstantInfo::Integer(ConstantIntegerInfo(value)))
    }

    fn parse_constant_float(&mut self) -> Result<ConstantInfo> {
        let bytes = self.read_u4()?;
        let value = f32::from_bits(bytes);
        Ok(ConstantInfo::Float(ConstantFloatInfo(value)))
    }

    fn parse_constant_long(&mut self) -> Result<ConstantInfo> {
        let raw_bytes = self.read_n_bytes(8)?;
        let value = i64::from_be_bytes(raw_bytes.try_into().unwrap());

        Ok(ConstantInfo::Long(ConstantLongInfo(value)))
    }

    fn parse_constant_double(&mut self) -> Result<ConstantInfo> {
        let raw_bytes = self.read_n_bytes(8)?;
        let value = f64::from_be_bytes(raw_bytes.try_into().unwrap());
        Ok(ConstantInfo::Double(ConstantDoubleInfo(value)))
    }

    fn parse_constant_name_and_type(&mut self) -> Result<ConstantInfo> {
        let name_index = self.read_u2()?;
        let descriptor_index = self.read_u2()?;
        Ok(ConstantInfo::NameAndType(ConstantNameAndTypeInfo {
            name_index,
            descriptor_index,
        }))
    }

    fn parse_constant_utf8(&mut self) -> Result<ConstantInfo> {
        let length = self.read_u2()?;
        let java_utf8_bytes = self.read_n_bytes(length as usize)?;
        let utf8_bytes = if let Ok(utf8_str) = cesu8::from_java_cesu8(&java_utf8_bytes) {
            utf8_str.to_string()
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Modified UTF8 decoding errror ",
            ));
        };
        Ok(ConstantInfo::Utf8(ConstantUtf8Info(utf8_bytes)))
    }

    fn parse_constant_method_handle(&mut self) -> Result<ConstantInfo> {
        let reference_kind = self.read_u1()?;
        let reference_index = self.read_u2()?;
        Ok(ConstantInfo::MethodHandle(ConstantMethodHandleInfo {
            reference_kind,
            reference_index,
        }))
    }

    fn parse_constant_method_type(&mut self) -> Result<ConstantInfo> {
        let descriptor_index = self.read_u2()?;
        Ok(ConstantInfo::MethodType(ConstantMethodTypeInfo {
            descriptor_index,
        }))
    }

    fn parse_constant_dynamic(&mut self) -> Result<ConstantInfo> {
        let bootstrap_method_attr_index = self.read_u2()?;
        let name_and_type_index = self.read_u2()?;
        Ok(ConstantInfo::Dynamic(ConstantDynamicInfo {
            bootstrap_method_attr_index,
            name_and_type_index,
        }))
    }

    fn parse_constant_invoke_dynamic(&mut self) -> Result<ConstantInfo> {
        let bootstrap_method_attr_index = self.read_u2()?;
        let name_and_type_index = self.read_u2()?;
        Ok(ConstantInfo::InvokeDynamic(ConstantInvokeDynamicInfo {
            bootstrap_method_attr_index,
            name_and_type_index,
        }))
    }

    fn parse_constant_module(&mut self) -> Result<ConstantInfo> {
        let name_index = self.read_u2()?;
        Ok(ConstantInfo::Module(ConstantModuleInfo { name_index }))
    }

    fn parse_constant_package(&mut self) -> Result<ConstantInfo> {
        let name_index = self.read_u2()?;
        Ok(ConstantInfo::Package(ConstantPackageInfo { name_index }))
    }

    fn parse_method_info(&mut self) -> Result<MethodInfo> {
        let flag_value = self.read_u2()?;
        let access_flags = MethodFlags::from_bits(flag_value).unwrap();
        let name_index = self.read_u2()?;
        let descriptor_index = self.read_u2()?;
        let attributes_count = self.read_u2()?;

        let mut attributes: Vec<AttributeInfo> = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            let attribute = self.parse_attribute_info()?;
            attributes.push(attribute);
        }

        Ok(MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }

    fn parse_field_info(&mut self) -> Result<FieldInfo> {
        let flag_value = self.read_u2()?;
        let access_flags = FieldFlags::from_bits(flag_value).unwrap();
        let name_index = self.read_u2()?;
        let descriptor_index = self.read_u2()?;
        let attributes_count = self.read_u2()?;

        let mut attributes: Vec<AttributeInfo> = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            let attribute = self.parse_attribute_info()?;
            attributes.push(attribute);
        }

        Ok(FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }

    //attribute
    fn parse_attribute_info(&mut self) -> Result<AttributeInfo> {
        let attribute_name_index = self.read_u2()?;
        let attribute_length = self.read_u4()?;

        let attribute_name = if let Some(ConstantInfo::Utf8(ConstantUtf8Info(s))) = self
            .class_file
            .get_constant_pool_entry(attribute_name_index)
        {
            s
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "expected ConstantInfo::Utf8 got sth else",
            ));
        };

        match attribute_name.as_str() {
            "ConstantValue" => self.parse_constant_value_attribute(attribute_length),
            "Code" => self.parse_code_attribute(),
            "LineNumberTable" => self.parse_line_number_table_attribute(),
            "BootstrapMethods" => self.parse_bootstrap_method_attribute(),
            "NestHost" => self.parse_nest_host_attribute(),
            "NestMembers" => self.parse_nest_members_attribute(),
            "PermittedSubclasses" => self.parse_permitted_subclasses_atttribute(),

            //"StackMapTable" => todo!(),
            _ => self.parse_remaining_attribute(attribute_name_index, attribute_length),
        }
    }

    fn parse_constant_value_attribute(&mut self, attribute_length: U4) -> Result<AttributeInfo> {
        if attribute_length == 2 {
            let constant_value_index = self.read_u2()?;
            //TODO directly store the constant value instead of index
            Ok(AttributeInfo::ConstantValue(ConstantValue(
                constant_value_index,
            )))
        } else {
            Err(std::io::Error::new(
                ErrorKind::Other,
                "Invalid length of attribute ConstantValue",
            ))
        }
    }

    fn parse_remaining_attribute(
        &mut self,
        attribute_name_index: U2,
        attribute_length: U4,
    ) -> Result<AttributeInfo> {
        let info = self.read_n_bytes(attribute_length as usize)?;
        Ok(AttributeInfo::Attribute(RemainingAttribute {
            attribute_name_index,
            info,
        }))
    }

    fn parse_code_attribute(&mut self) -> Result<AttributeInfo> {
        let max_stack = self.read_u2()?;
        let max_locals = self.read_u2()?;

        let code_length = self.read_u4()?;
        assert!(code_length > 0);

        let code = self.parse_byte_code(code_length)?;
        let exception_table_count = self.read_u2()?;
        let exception_table = self.parse_exception_table(exception_table_count)?;
        let attributes = self.parse_attributes()?;

        let address_to_index = code
            .iter()
            .enumerate()
            .map(|(index, Instruction(address, _))| (*address, index))
            .collect();

        Ok(AttributeInfo::Code(Code {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
            address_to_index,
        }))
    }

    fn parse_byte_code(&mut self, code_length: U4) -> Result<Vec<Instruction>> {
        //self.read_n_bytes(code_length as usize);
        let mut address: U4 = 0;
        let mut instructions: Vec<Instruction> = Vec::with_capacity(code_length as usize);
        loop {
            let (instr_bytes, instruction) = self.parse_instruction(&address)?;
            instructions.push(Instruction(address, instruction));
            address += instr_bytes;
            if address == code_length {
                break;
            } else if address > code_length {
                return Err(std::io::Error::new(
                    ErrorKind::Other,
                    format!("Invalid position reach while parsing byte code"),
                ));
            }
        }
        Ok(instructions)
    }

    fn parse_exception_table(
        &mut self,
        exception_table_count: U2,
    ) -> Result<Vec<ExceptionTableEntry>> {
        let mut exception_table: Vec<ExceptionTableEntry> =
            Vec::with_capacity(exception_table_count as usize);

        for _ in 0..exception_table_count {
            let start_pc = self.read_u2()?;
            let end_pc = self.read_u2()?;
            let handler_pc = self.read_u2()?;
            let catch_type = self.read_u2()?;
            let entry = ExceptionTableEntry {
                start_pc,
                end_pc,
                handler_pc,
                catch_type,
            };
            exception_table.push(entry);
        }

        Ok(exception_table)
    }

    fn parse_line_number_table_attribute(&mut self) -> Result<AttributeInfo> {
        let line_number_table_count = self.read_u2()?;
        let mut line_number_table: Vec<LineNumberTableEntry> =
            Vec::with_capacity(line_number_table_count as usize);
        for _ in 0..line_number_table_count {
            let start_pc = self.read_u2()?;
            let line_number = self.read_u2()?;
            line_number_table.push(LineNumberTableEntry {
                start_pc,
                line_number,
            })
        }
        Ok(AttributeInfo::LineNumberTable(LineNumberTable(
            line_number_table,
        )))
    }

    fn parse_bootstrap_method_attribute(&mut self) -> Result<AttributeInfo> {
        let bootstrap_methods_count = self.read_u2()?;
        let mut bootstrap_methods: Vec<BootstrapMethodEntry> =
            Vec::with_capacity(bootstrap_methods_count as usize);

        for _ in 0..bootstrap_methods_count {
            let bootstrap_method_ref = self.read_u2()?;

            let bootstrap_args = self.read_u2_vec()?;

            let bootstrap_method = BootstrapMethodEntry {
                bootstrap_method_ref,
                bootstrap_args,
            };
            bootstrap_methods.push(bootstrap_method);
        }
        Ok(AttributeInfo::BootstrapMethod(BootstrapMethod(
            bootstrap_methods,
        )))
    }
    fn parse_nest_host_attribute(&mut self) -> Result<AttributeInfo> {
        let host_class_index = self.read_u2()?;
        Ok(AttributeInfo::NestHost(NestHost(host_class_index)))
    }
    fn parse_nest_members_attribute(&mut self) -> Result<AttributeInfo> {
        let classes = self.read_u2_vec()?;
        Ok(AttributeInfo::NestMembers(NestMembers(classes)))
    }

    fn parse_permitted_subclasses_atttribute(&mut self) -> Result<AttributeInfo> {
        let classes = self.read_u2_vec()?;
        Ok(AttributeInfo::PermitterSubclasses(PermitterSubclasses(
            classes,
        )))
    }

    fn parse_instruction(&mut self, address: &U4) -> Result<(U4, Operation)> {
        const LOOKUP_SWITCH_OPERAND_COUNT: usize = 8;
        const TABLE_SWITCH_OPERAND_COUNT: usize = 16;

        let mut byte_read: U4 = 1;

        let instruction = match self.read_u1()? {
            0x32 => Operation::Aaload,
            0x53 => Operation::Aastore,
            0x01 => Operation::Aconstnull,
            0x19 => Operation::Aload(self.read_u1_with_count(&mut byte_read)?),
            0x2a => Operation::Aload0,
            0x2b => Operation::Aload1,
            0x2c => Operation::Aload2,
            0x2d => Operation::Aload3,
            0xbd => Operation::Anewarray(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xb0 => Operation::Areturn,
            0xbe => Operation::Arraylength,
            0x3a => Operation::Astore(self.read_u1_with_count(&mut byte_read)?),
            0x4b => Operation::Astore0,
            0x4c => Operation::Astore1,
            0x4d => Operation::Astore2,
            0x4e => Operation::Astore3,
            0xbf => Operation::Athrow,
            0x33 => Operation::Baload,
            0x54 => Operation::Bastore,
            0x10 => Operation::Bipush(self.read_u1_with_count(&mut byte_read)?),
            0xca => Operation::Breakpoint,
            0x34 => Operation::Caload,
            0x55 => Operation::Castore,
            0xc0 => Operation::Checkcast(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x90 => Operation::D2f,
            0x8e => Operation::D2i,
            0x8f => Operation::D2l,
            0x63 => Operation::Dadd,
            0x31 => Operation::Daload,
            0x52 => Operation::Dastore,
            0x98 => Operation::Dcmpg,
            0x97 => Operation::Dcmpl,
            0x0e => Operation::Dconst0,
            0x0f => Operation::Dconst1,
            0x6f => Operation::Ddiv,
            0x18 => Operation::Dload(self.read_u1_with_count(&mut byte_read)?),
            0x26 => Operation::Dload0,
            0x27 => Operation::Dload1,
            0x28 => Operation::Dload2,
            0x29 => Operation::Dload3,
            0x6b => Operation::Dmul,
            0x77 => Operation::Dneg,
            0x73 => Operation::Drem,
            0xaf => Operation::Dreturn,
            0x39 => Operation::Dstore(self.read_u1_with_count(&mut byte_read)?),
            0x47 => Operation::Dstore0,
            0x48 => Operation::Dstore1,
            0x49 => Operation::Dstore2,
            0x4a => Operation::Dstore3,
            0x67 => Operation::Dsub,
            0x59 => Operation::Dup,
            0x5a => Operation::Dupx1(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x5b => Operation::Dupx2(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x5c => Operation::Dup2,
            0x5d => Operation::Dup2x1(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x5e => Operation::Dup2x2,
            0x8d => Operation::F2d,
            0x8b => Operation::F2i,
            0x8c => Operation::F2l,
            0x62 => Operation::Fadd,
            0x30 => Operation::Faload,
            0x51 => Operation::Fastore,
            0x96 => Operation::Fcmpg,
            0x95 => Operation::Fcmpl,
            0x0b => Operation::Fconst0,
            0x0c => Operation::Fconst1,
            0x0d => Operation::Fconst2,
            0x6e => Operation::Fdiv,
            0x17 => Operation::Fload(self.read_u1_with_count(&mut byte_read)?),
            0x22 => Operation::Fload0,
            0x23 => Operation::Fload1,
            0x24 => Operation::Fload2,
            0x25 => Operation::Fload3,
            0x6a => Operation::Fmul,
            0x76 => Operation::Fneg,
            0x72 => Operation::Frem,
            0xae => Operation::Freturn,
            0x38 => Operation::Fstore(self.read_u1_with_count(&mut byte_read)?),
            0x43 => Operation::Fstore0,
            0x44 => Operation::Fstore1,
            0x45 => Operation::Fstore2,
            0x46 => Operation::Fstore3,
            0x66 => Operation::Fsub,
            0xb4 => Operation::Getfield(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xb2 => Operation::Getstatic(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xa7 => Operation::Goto(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xc8 => Operation::Gotow(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x91 => Operation::I2b,
            0x92 => Operation::I2c,
            0x87 => Operation::I2d,
            0x86 => Operation::I2f,
            0x85 => Operation::I2l,
            0x93 => Operation::I2s,
            0x60 => Operation::Iadd,
            0x2e => Operation::Iaload,
            0x7e => Operation::Iand,
            0x4f => Operation::Iastore,
            0x02 => Operation::Iconstm1,
            0x03 => Operation::Iconst0,
            0x04 => Operation::Iconst1,
            0x05 => Operation::Iconst2,
            0x06 => Operation::Iconst3,
            0x07 => Operation::Iconst4,
            0x08 => Operation::Iconst5,
            0x6c => Operation::Idiv,
            0xa5 => Operation::Ifacmpeq(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xa6 => Operation::Ifacmpne(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x9f => Operation::Ificmpeq(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xa2 => Operation::Ificmpge(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xa3 => Operation::Ificmpgt(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xa4 => Operation::Ificmple(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xa1 => Operation::Ificmplt(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xa0 => Operation::Ificmpne(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x99 => Operation::Ifeq(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x9c => Operation::Ifge(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x9d => Operation::Ifgt(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x9e => Operation::Ifle(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x9b => Operation::Iflt(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x9a => Operation::Ifne(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xc7 => Operation::Ifnonnull(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xc6 => Operation::Ifnull(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x84 => Operation::Iinc(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x15 => Operation::Iload(self.read_u1_with_count(&mut byte_read)?),
            0x1a => Operation::Iload0,
            0x1b => Operation::Iload1,
            0x1c => Operation::Iload2,
            0x1d => Operation::Iload3,
            0xfe => Operation::Impdep1,
            0xff => Operation::Impdep2,
            0x68 => Operation::Imul,
            0x74 => Operation::Ineg,
            0xc1 => Operation::Instanceof(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xba => Operation::Invokedynamic(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xb9 => Operation::Invokeinterface(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xb7 => Operation::Invokespecial(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xb8 => Operation::Invokestatic(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xb6 => Operation::Invokevirtual(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x80 => Operation::Ior,
            0x70 => Operation::Irem,
            0xac => Operation::Ireturn,
            0x78 => Operation::Ishl,
            0x7a => Operation::Ishr,
            0x36 => Operation::Istore(self.read_u1_with_count(&mut byte_read)?),
            0x3b => Operation::Istore0,
            0x3c => Operation::Istore1,
            0x3d => Operation::Istore2,
            0x3e => Operation::Istore3,
            0x64 => Operation::Isub,
            0x7c => Operation::Iushr,
            0x82 => Operation::Ixor,
            0xa8 => Operation::Jsr(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xc9 => Operation::Jsrw(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x8a => Operation::L2d,
            0x89 => Operation::L2f,
            0x88 => Operation::L2i,
            0x61 => Operation::Ladd,
            0x2f => Operation::Laload,
            0x7f => Operation::Land,
            0x50 => Operation::Lastore,
            0x94 => Operation::Lcmp,
            0x09 => Operation::Lconst0,
            0x0a => Operation::Lconst1,
            0x12 => Operation::Ldc(self.read_u1_with_count(&mut byte_read)?),
            0x13 => Operation::Ldcw(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x14 => Operation::Ldc2w(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x6d => Operation::Ldiv,
            0x16 => Operation::Lload(self.read_u1_with_count(&mut byte_read)?),
            0x1e => Operation::Lload0,
            0x1f => Operation::Lload1,
            0x20 => Operation::Lload2,
            0x21 => Operation::Lload3,
            0x69 => Operation::Lmul,
            0x75 => Operation::Lneg,
            /*
                        0xab => {
                            byte_read += LOOKUP_SWITCH_OPERAND_COUNT as U4;
                            Operation::Lookupswitch(self.read_n_bytes(LOOKUP_SWITCH_OPERAND_COUNT)?)
                        }
            */
            0xab => {
                let padding = (4 - ((address + 1) % 4)) % 4;

                self.skip_padding(padding)?;
                byte_read += padding;

                let default_offset = i32::from_be_bytes(self.read_n_bytes(4)?.try_into().unwrap());
                byte_read += 4;

                let npairs = i32::from_be_bytes(self.read_n_bytes(4)?.try_into().unwrap());
                if npairs < 0 {
                    return Err(std::io::Error::new(
                        ErrorKind::InvalidData,
                        format!("lookupswitch npairs is negative: {}", npairs),
                    ));
                }

                byte_read += 4;

                let mut pairs = Vec::with_capacity(npairs as usize);
                let mut last_match: Option<i32> = None;

                for _ in 0..npairs {
                    let match_value = i32::from_be_bytes(self.read_n_bytes(4)?.try_into().unwrap());
                    byte_read += 4;

                    let offset = i32::from_be_bytes(self.read_n_bytes(4)?.try_into().unwrap());
                    byte_read += 4;

                    if let Some(last) = last_match {
                        if match_value <= last {
                            return Err(std::io::Error::new(
                                ErrorKind::InvalidData,
                                format!(
                        "lookupswitch match values not in strictly increasing order: {} follows {}",
                        match_value, last
                    ),
                            ));
                        }
                    }

                    pairs.push((match_value, offset));
                    last_match = Some(match_value);
                }

                Operation::Lookupswitch(default_offset, npairs, pairs)
            }

            0x81 => Operation::Lor,
            0x71 => Operation::Lrem,
            0xad => Operation::Lreturn,
            0x79 => Operation::Lshl,
            0x7b => Operation::Lshr,
            0x37 => Operation::Lstore(self.read_u1_with_count(&mut byte_read)?),
            0x3f => Operation::Lstore0,
            0x40 => Operation::Lstore1,
            0x41 => Operation::Lstore2,
            0x42 => Operation::Lstore3,
            0x65 => Operation::Lsub,
            0x7d => Operation::Lushr,
            0x83 => Operation::Lxor,
            0xc2 => Operation::Monitorenter,
            0xc3 => Operation::Monitorexit,
            0xc5 => Operation::Multianewarray(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xbb => Operation::New(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xbc => Operation::Newarray(self.read_u1_with_count(&mut byte_read)?),
            0x00 => Operation::Nop,
            0x57 => Operation::Pop,
            0x58 => Operation::Pop2,
            0xb5 => Operation::Putfield(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xb3 => Operation::Putstatic(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0xa9 => Operation::Ret(self.read_u1_with_count(&mut byte_read)?),
            0xb1 => Operation::Return,
            0x35 => Operation::Saload,
            0x56 => Operation::Sastore,
            0x11 => Operation::Sipush(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            0x5f => Operation::Swap(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            /*
            0xaa => {
                byte_read += LOOKUP_SWITCH_OPERAND_COUNT as U4;
                Operation::Tableswitch(self.read_n_bytes(TABLE_SWITCH_OPERAND_COUNT)?)
            }
            */
            0xaa => {
                let padding = (4 - ((address + 1) % 4)) % 4;
                self.skip_padding(padding)?;
                byte_read += padding;

                let default_offset = i32::from_be_bytes(self.read_n_bytes(4)?.try_into().unwrap());
                byte_read += 4;

                let low = i32::from_be_bytes(self.read_n_bytes(4)?.try_into().unwrap());
                byte_read += 4;

                let high = i32::from_be_bytes(self.read_n_bytes(4)?.try_into().unwrap());
                byte_read += 4;

                if low > high {
                    return Err(std::io::Error::new(
                        ErrorKind::InvalidData,
                        format!("tableswitch low ({}) is greater than high ({})", low, high),
                    ));
                }

                let num_offsets = (high - low + 1) as usize;

                let mut jump_offsets = Vec::with_capacity(num_offsets);
                for _ in 0..num_offsets {
                    let offset = i32::from_be_bytes(self.read_n_bytes(4)?.try_into().unwrap());
                    byte_read += 4;
                    jump_offsets.push(offset);
                }

                Operation::Tableswitch(default_offset, low, high, jump_offsets)
            }

            0xc4 => Operation::Wide(
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
                self.read_u1_with_count(&mut byte_read)?,
            ),
            op_code => {
                return Err(std::io::Error::new(
                    ErrorKind::Other,
                    format!("Invalid op code {op_code}"),
                ))
            }
        };
        Ok((byte_read, instruction))
    }

    fn skip_padding(&mut self, padding: U4) -> Result<()> {
        for _ in 0..padding {
            self.read_u1()?;
        }
        Ok(())
    }
}
/*
* Seven attributes are critical to correct interpretation of the class file by the Java Virtual Machine:
* • ConstantValue ^^
* • Code ^^
* • StackMapTable
* • BootstrapMethods ^^
* • NestHost ^^
* • NestMembers ^^
* • PermittedSubclasses ^^
*/
