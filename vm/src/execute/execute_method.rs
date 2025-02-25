use super::execute::ExecutionResult;
use crate::class_loader::loaded_class::LoadedClass;
use crate::runtime::*;
use crate::vm::VM;
use crate::{class_loader::loaded_class::NameDes, jvm_error::JVMError};
use parser::access_flag::ClassFlags;
use parser::attribute::Code;
use parser::constant_pool::{ConstantInfo, ConstantInterfaceMethodRefInfo, ConstantMethodRefInfo};
use std::sync::Arc;

impl Frame {
    pub fn get_entry(&self, index: u16) -> Result<ConstantInfo, JVMError> {
        let constant_info =
            self.constant_pool
                .get_entry(index)
                .ok_or(JVMError::ConstantPoolIndexOutOfBounds {
                    index,
                    max: self.constant_pool.get_len(),
                })?;
        Ok(constant_info.clone())
    }

    fn resolve_method_ref(&self, index: u16) -> Result<(String, NameDes), JVMError> {
        let cp_entry = self.get_entry(index)?;
        match cp_entry {
            ConstantInfo::Methodref(ConstantMethodRefInfo {
                class_index,
                name_and_type_index,
            })
            | ConstantInfo::InterfaceMethodRef(ConstantInterfaceMethodRefInfo {
                class_index,
                name_and_type_index,
            }) => {
                let class_name = self
                    .constant_pool
                    .get_underlying_string_from_constant_class_info_index(class_index)
                    .ok_or(JVMError::ConstantPoolIndexOutOfBounds {
                        index,
                        max: self.constant_pool.get_len(),
                    })?;
                let constant_name_and_type = self.get_entry(name_and_type_index)?;
                let name_and_type = match constant_name_and_type {
                    ConstantInfo::NameAndType(value) => Ok(value.clone()),
                    _ => Err(JVMError::InvalidConstantType {
                        expected: "Methodref or InterfaceMethodref",
                        found: "other",
                    }),
                };
                let name_des = NameDes::new(&name_and_type?, &self.constant_pool);
                Ok((class_name.to_string(), name_des))
            }
            _ => Err(JVMError::InvalidConstantType {
                expected: "Methodref or InterfaceMethodref",
                found: "other",
            }),
        }
    }

    fn prepare_arguments(&mut self, descriptor: &str) -> Result<Vec<Value>, JVMError> {
        let param_count = Self::count_parameters(descriptor);
        if self.operands.len() < param_count {
            return Err(JVMError::InsufficientOperands {
                required: param_count,
                found: self.operands.len(),
            });
        }
        let mut args = Vec::with_capacity(param_count);
        for _ in 0..param_count {
            args.push(self.pop()?);
        }
        args.reverse();
        Ok(args)
    }

    fn count_parameters(descriptor: &str) -> usize {
        let mut count = 0;
        let mut chars = descriptor.chars().skip(1);
        while let Some(ch) = chars.next() {
            if ch == ')' {
                break;
            }
            match ch {
                'J' | 'D' => count += 1,
                'L' => {
                    count += 1;
                    while chars.next() != Some(';') {}
                }
                '[' => {
                    count += 1;
                    while chars.next() == Some('[') {}
                    if let Some('L') = chars.next() {
                        while chars.next() != Some(';') {}
                    }
                }
                _ => count += 1,
            }
        }
        count
    }

    pub fn lookup_method(
        class: &Arc<LoadedClass>,
        name_des: &NameDes,
    ) -> Result<Arc<Code>, JVMError> {
        class
            .get_code_from_method(name_des)
            .ok_or_else(|| JVMError::MethodNotFound {
                class: class.class_name.clone(),
                name: name_des.name.clone(),
                descriptor: name_des.des.clone(),
            })
    }

    fn lookup_virtual_method(
        &self,
        class: &Arc<LoadedClass>,
        name_des: &NameDes,
    ) -> Result<Arc<Code>, JVMError> {
        if let Some(code) = class.get_code_from_method(name_des) {
            return Ok(code);
        }
        let mut current_class = class.clone();
        while let Some(super_class) = &current_class.super_class {
            if let Some(code) = super_class.get_code_from_method(name_des) {
                return Ok(code);
            }
            current_class = Arc::clone(super_class);
        }
        if class.access_flags.contains(ClassFlags::ACC_INTERFACE) {
            for interface in &class.interfaces {
                let code = self.lookup_virtual_method(interface, name_des)?;
                return Ok(code);
            }
        }

        Err(JVMError::MethodNotFound {
            class: class.class_name.clone(),
            name: name_des.name.clone(),
            descriptor: name_des.des.clone(),
        })
    }

    pub async fn invokestatic(&mut self, index: u16, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let (class_name, name_des) = self.resolve_method_ref(index)?;
        println!("{class_name}");
        let target_class = vm
            .class_loader
            .load_class(&class_name)
            .await
            .map_err(|e| JVMError::Other(e.to_string())).unwrap();
        let method_code = self.lookup_virtual_method(&target_class, &name_des)?;
        let args = self.prepare_arguments(&name_des.des)?;

        let mut new_frame = Frame::new(target_class, &name_des, method_code);
        for (i, arg) in args.into_iter().enumerate() {
            new_frame.set_local(i, arg);
        }

        Ok(ExecutionResult::Invoke(new_frame))
    }

    pub async fn invokespecial(&mut self, index: u16, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let (class_name, name_des) = self.resolve_method_ref(index)?;
        let target_class = vm.class_loader.load_class(&class_name).await
            .map_err(|e| JVMError::Other(e.to_string()))?;
        
        let method_code = Self::lookup_method(&target_class, &name_des)?;
        let mut args = self.prepare_arguments(&name_des.des)?;
        let object_ref = self.pop()?;
        match object_ref {
            Value::Reference(Some(obj)) => {
                if !obj.class.class_name.eq(&class_name) {
                    return Err(JVMError::IncompatibleClass {
                        expected: class_name,
                        found: obj.class.class_name.clone(),
                    });
                }
                args.insert(0, Value::Reference(Some(obj)));
            }
            Value::Reference(None) => return Err(JVMError::NullReference),
            _ => return Err(JVMError::TypeMismatch {
                expected: "Reference",
                found: "non-reference",
            }),
        }

        let mut new_frame = Frame::new(target_class, &name_des, method_code);
        for (i, arg) in args.into_iter().enumerate() {
            new_frame.set_local(i, arg);
        }

        Ok(ExecutionResult::Invoke(new_frame))
    }

    pub async fn invokevirtual(&mut self, index: u16, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let (class_name, name_des) = self.resolve_method_ref(index)?;
        let mut args = self.prepare_arguments(&name_des.des)?;
        let object_ref = self.pop()?;
        match object_ref {
            Value::Reference(Some(obj)) => {
                let target_class = Arc::clone(&obj.class);
                let method_code = self.lookup_virtual_method(&target_class, &name_des)?;
                args.insert(0, Value::Reference(Some(obj)));

                let mut new_frame = Frame::new(target_class, &name_des, method_code);
                for (i, arg) in args.into_iter().enumerate() {
                    new_frame.set_local(i, arg);
                }

                Ok(ExecutionResult::Invoke(new_frame))
            }
            Value::Reference(None) => Err(JVMError::NullReference),
            _ => Err(JVMError::TypeMismatch {
                expected: "Reference",
                found: "non-reference",
            }),
        }
    }

    fn implements_interface(&self, class: &Arc<LoadedClass>, interface: &Arc<LoadedClass>) -> bool {
        if class.class_name == interface.class_name {
            return true;
        }
        for impl_interface in &class.interfaces {
            if self.implements_interface(impl_interface, interface) {
                return true;
            }
        }
        if let Some(super_class) = &class.super_class {
            return self.implements_interface(super_class, interface);
        }
        false
    }

    pub async fn invokeinterface(&mut self, index: u16, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let (class_name, name_des) = self.resolve_method_ref(index)?;
        let mut args = self.prepare_arguments(&name_des.des)?;
        let object_ref = self.pop()?;
        match object_ref {
            Value::Reference(Some(obj)) => {
                let target_class = Arc::clone(&obj.class);
                let interface_class = vm.class_loader.load_class(&class_name).await
                    .map_err(|e| JVMError::Other(e.to_string()))?;
                if !self.implements_interface(&target_class, &interface_class) {
                    return Err(JVMError::IncompatibleClass {
                        expected: class_name,
                        found: target_class.class_name.clone(),
                    });
                }

                let method_code = self.lookup_virtual_method(&target_class, &name_des)?;
                args.insert(0, Value::Reference(Some(obj)));

                let mut new_frame = Frame::new(target_class, &name_des, method_code);
                for (i, arg) in args.into_iter().enumerate() {
                    new_frame.set_local(i, arg);
                }

                Ok(ExecutionResult::Invoke(new_frame))
            }
            Value::Reference(None) => Err(JVMError::NullReference),
            _ => Err(JVMError::TypeMismatch {
                expected: "Reference",
                found: "non-reference",
            }),
        }
    }
}
