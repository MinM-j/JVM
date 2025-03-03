use super::execute::ExecutionResult;
use crate::class_loader::loaded_class::LoadedClass;
use crate::runtime::*;
use crate::vm::VM;
use crate::{class_loader::loaded_class::NameDes, jvm_error::JVMError};
use parser::access_flag::ClassFlags;
use parser::access_flag::MethodFlags;
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
    ) -> Result<(Arc<LoadedClass>, Arc<Code>), JVMError> {
        let code = class
            .get_code_from_method(name_des)
            .ok_or_else(|| JVMError::MethodNotFound {
                class: class.class_name.clone(),
                name: name_des.name.clone(),
                descriptor: name_des.des.clone(),
            });
        Ok((class.clone(), code?))
    }

    fn lookup_virtual_method(
        &self,
        class: &Arc<LoadedClass>,
        name_des: &NameDes,
    ) -> Result<(Arc<LoadedClass>, Arc<Code>), JVMError> {
        if let Some(code) = class.get_code_from_method(name_des) {
            return Ok((class.clone(), code));
        }
        let mut current_class = class.clone();
        while let Some(super_class) = &current_class.super_class {
            if let Some(code) = super_class.get_code_from_method(name_des) {
                return Ok((super_class.clone(), code));
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
        let fut = Box::pin(vm.class_loader.load_class(&class_name, vm));

        let target_class = fut
            .await
            .map_err(|e| JVMError::Other(e.to_string()))
            .unwrap();
        let method_info = target_class
            .methods
            .iter()
            .find(|method| {
                let method_name = target_class
                    .constant_pool
                    .get_underlying_string_from_utf8_index(method.name_index)
                    .unwrap();
                let method_desc = target_class
                    .constant_pool
                    .get_underlying_string_from_utf8_index(method.descriptor_index)
                    .unwrap();
                *method_name == name_des.name && *method_desc == name_des.des
            })
            .ok_or_else(|| JVMError::Other(format!("Method not found: {}", name_des.name)))?;

        if method_info.access_flags.contains(MethodFlags::ACC_NATIVE) {
            if let Some(native_loader) = vm.native_methods.get(&name_des) {
                let mut args = self.prepare_arguments(&name_des.des)?;
                args.reverse();
                let native_name =
                    format!("Java_{}_{}", class_name.replace('/', "_"), name_des.name);
                //println!("{native_name}");
                let result = native_loader
                    .invoke(&native_name, &args)
                    .map_err(|e| JVMError::Other(format!("Native call failed: {}", e)))?;
                if name_des.des.ends_with("V") {
                    Ok(ExecutionResult::Continue)
                } else {
                    self.push(result)?;
                    Ok(ExecutionResult::Continue)
                }
            } else {
                Err(JVMError::Other(format!(
                    "Native method not found: {}",
                    name_des.name
                )))
            }
        } else {
            let (method_class, method_code) =
                self.lookup_virtual_method(&target_class, &name_des)?;
            let args = self.prepare_arguments(&name_des.des)?;

            let mut new_frame = Frame::new(method_class, &name_des, method_code);
            let mut i = 0;
            for arg in args.into_iter() {
                new_frame.set_local(i, arg.clone());
                if Self::get_value_type(&arg) == "double" || Self::get_value_type(&arg) == "long" {
                    i = i + 2;
                } else {
                    i = i + 1;
                }
            }

            Ok(ExecutionResult::Invoke(new_frame))
        }
    }

    pub async fn invokespecial(
        &mut self,
        index: u16,
        vm: &VM,
    ) -> Result<ExecutionResult, JVMError> {
        let (class_name, name_des) = self.resolve_method_ref(index)?;
        let fut = Box::pin(vm.class_loader.load_class(&class_name, vm));
        let target_class = fut.await.map_err(|e| JVMError::Other(e.to_string()))?;
        let (method_class, method_code) = Self::lookup_method(&target_class, &name_des)?;
        let mut args = self.prepare_arguments(&name_des.des)?;
        let object_ref = self.pop()?;
        match object_ref {
            Value::Reference(Some(obj)) => {
                if let Some(obj_class) = &obj.class {
                    if !self.is_compatible_class(obj_class, &class_name) {
                        return Err(JVMError::IncompatibleClass {
                            expected: class_name,
                            found: obj_class.class_name.clone(),
                        });
                    }
                }
                args.insert(0, Value::Reference(Some(obj)));
            }
            Value::Reference(None) => return Err(JVMError::NullReference),
            _ => {
                return Err(JVMError::TypeMismatch {
                    expected: "Reference".to_string(),
                    found: "non-reference".to_string(),
                })
            }
        }

        let mut new_frame = Frame::new(method_class, &name_des, method_code);
        let mut i = 0;
        for arg in args.into_iter() {
            new_frame.set_local(i, arg.clone());
            if Self::get_value_type(&arg) == "double" || Self::get_value_type(&arg) == "long" {
                i = i + 2;
            } else {
                i = i + 1;
            }
        }

        Ok(ExecutionResult::Invoke(new_frame))
    }

    pub async fn invokevirtual(
        &mut self,
        index: u16,
        vm: &VM,
    ) -> Result<ExecutionResult, JVMError> {
        let (class_name, name_des) = self.resolve_method_ref(index)?;
        let mut args = self.prepare_arguments(&name_des.des)?;
        let object_ref = self.pop()?;
        match object_ref {
            Value::Reference(Some(obj)) => match &obj.class {
                Some(target_class) => {
                    let target_class = Arc::clone(target_class);
                    let (method_class, method_code) =
                        self.lookup_virtual_method(&target_class, &name_des)?;
                    args.insert(0, Value::Reference(Some(obj)));

                    let mut new_frame = Frame::new(method_class, &name_des, method_code);
                    let mut i = 0;
                    for arg in args.into_iter() {
                        new_frame.set_local(i, arg.clone());
                        if Self::get_value_type(&arg) == "double"
                            || Self::get_value_type(&arg) == "long"
                        {
                            i = i + 2;
                        } else {
                            i = i + 1;
                        }
                    }

                    Ok(ExecutionResult::Invoke(new_frame))
                }
                None => Err(JVMError::NullReference),
            },
            Value::Reference(None) => Err(JVMError::NullReference),
            _ => Err(JVMError::TypeMismatch {
                expected: "Reference".to_string(),
                found: "non-reference".to_string(),
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

    pub async fn invokeinterface(
        &mut self,
        index: u16,
        vm: &VM,
    ) -> Result<ExecutionResult, JVMError> {
        let (class_name, name_des) = self.resolve_method_ref(index)?;
        let mut args = self.prepare_arguments(&name_des.des)?;
        let object_ref = self.pop()?;
        match object_ref {
            Value::Reference(Some(obj)) => match &obj.class {
                Some(target_class) => {
                    let target_class = Arc::clone(target_class);
                    let fut = Box::pin(vm.class_loader.load_class(&class_name, vm));
                    let interface_class = fut.await.map_err(|e| JVMError::Other(e.to_string()))?;
                    if !self.implements_interface(&target_class, &interface_class) {
                        return Err(JVMError::IncompatibleClass {
                            expected: class_name,
                            found: target_class.class_name.clone(),
                        });
                    }
                    let (method_class, method_code) =
                        self.lookup_virtual_method(&target_class, &name_des)?;
                    args.insert(0, Value::Reference(Some(obj)));
                    let mut new_frame = Frame::new(method_class, &name_des, method_code);
                    let mut i = 0;
                    for arg in args.into_iter() {
                        new_frame.set_local(i, arg.clone());
                        if Self::get_value_type(&arg) == "double"
                            || Self::get_value_type(&arg) == "long"
                        {
                            i = i + 2;
                        } else {
                            i = i + 1;
                        }
                    }
                    Ok(ExecutionResult::Invoke(new_frame))
                }
                None => Err(JVMError::NullReference),
            },
            Value::Reference(None) => Err(JVMError::NullReference),
            _ => Err(JVMError::TypeMismatch {
                expected: "Reference".to_string(),
                found: "non-reference".to_string(),
            }),
        }
    }
}
