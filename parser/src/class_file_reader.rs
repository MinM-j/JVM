use super::types::*;
use super::{attribute::*, class_file::ClassFile, class_version::ClassVersion, consant_pool::*};

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
        self.class_file.constant_pool = constant_pool;
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
        self.class_file.access_flags = flags;
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
        Ok(ConstantInfo::Class(ConstantClassInfo { name_index }))
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
        let access_flags = self.read_u2()?;
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
        let access_flags = self.read_u2()?;
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

        let attribute_name = if let ConstantInfo::Utf8(ConstantUtf8Info(s)) = self
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

        dbg!(attribute_name);

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

        Ok(AttributeInfo::Code(Code {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        }))
    }

    fn parse_byte_code(&mut self, code_length: U4) -> Result<Vec<U1>> {
        self.read_n_bytes(code_length as usize)
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
