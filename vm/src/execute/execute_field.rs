use super::execute::ExecutionResult;
use crate::class_loader::loaded_class::LoadedClass;
use crate::jvm_error::JVMError;
use crate::runtime::*;
use crate::vm::VM;
use parser::constant_pool::{ConstantFieldRefInfo, ConstantInfo, ConstantNameAndTypeInfo};

impl Frame {
    fn pop_expect(&mut self, expected: &'static str) -> Result<Value, JVMError> {
        match expected {
            "I" => Ok(Value::Int(self.pop_expect_int()?)),
            "J" => Ok(Value::Long(self.pop_expect_long()?)),
            "D" => Ok(Value::Double(self.pop_expect_double()?)),
            "F" => Ok(Value::Float(self.pop_expect_float()?)),
            "L" => Ok(Value::Reference(self.pop_expect_reference()?)),
            _ => Err(JVMError::TypeMismatch {
                expected: expected.to_string(),
                found: "other".to_string(),
            }),
        }
    }

    pub fn is_compatible_class(
        &self,
        actual_class: &LoadedClass,
        expected_class_name: &str,
    ) -> bool {
        let mut current = Some(actual_class);
        while let Some(cls) = current {
            if cls.class_name == expected_class_name {
                return true;
            }
            current = cls.super_class.as_deref();
        }
        false
    }

    fn get_name_and_type(&self, index: u16) -> Result<(String, String), JVMError> {
        let cp_entry = self.constant_pool.get_entry(index).ok_or_else(|| {
            JVMError::ConstantPoolIndexOutOfBounds {
                index,
                max: self.constant_pool.get_len(),
            }
        })?;

        let (name, type_) = match cp_entry {
            ConstantInfo::NameAndType(ConstantNameAndTypeInfo {
                name_index,
                descriptor_index,
            }) => {
                let name = self
                    .constant_pool
                    .get_underlying_string_from_utf8_index(*name_index)
                    .ok_or_else(|| JVMError::InvalidConstantType {
                        expected: "NameAndType",
                        found: "missing name",
                    })?
                    .to_string();

                let type_ = self
                    .constant_pool
                    .get_underlying_string_from_utf8_index(*descriptor_index)
                    .ok_or_else(|| JVMError::InvalidConstantType {
                        expected: "NameAndType",
                        found: "missing type",
                    })?
                    .to_string();

                (name, type_)
            }
            _ => {
                return Err(JVMError::InvalidConstantType {
                    expected: "NameAndType",
                    found: "other",
                });
            }
        };

        Ok((name, type_))
    }

    pub async fn putfield(&mut self, index: u16, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let cp_entry = self.constant_pool.get_entry(index).ok_or_else(|| {
            JVMError::ConstantPoolIndexOutOfBounds {
                index,
                max: self.constant_pool.get_len(),
            }
        })?;
        let (class_index, name_and_type_index) = match cp_entry {
            ConstantInfo::FieldRef(ConstantFieldRefInfo {
                class_index,
                name_and_type_index,
            }) => (class_index, name_and_type_index),
            _ => {
                return Err(JVMError::InvalidConstantType {
                    expected: "Fieldref",
                    found: "other",
                })
            }
        };
        let expected_class_name = self
            .constant_pool
            .get_underlying_string_from_constant_class_info_index(*class_index)
            .ok_or_else(|| JVMError::InvalidConstantType {
                expected: "Class",
                found: "missing name",
            })?
            .to_string();
        let (name, descriptor) = self.get_name_and_type(*name_and_type_index)?;
        let expected_type = descriptor.split(';').next().unwrap_or(&descriptor);
        let value = match expected_type {
            "Z" | "B" | "C" | "S" | "I" => self.pop_expect("I")?,
            "J" => self.pop_expect("J")?,
            "F" => self.pop_expect("F")?,
            "D" => self.pop_expect("D")?,
            t if t.starts_with("L") || t.starts_with("[") => self.pop_expect("L")?,
            _ => {
                return Err(JVMError::Other(format!(
                    "Unsupported field descriptor: {}",
                    descriptor
                )))
            }
        };
        let obj_ref = self.pop_expect("L")?;
        if let Value::Reference(Some(obj)) = &obj_ref {
            let actual_class = obj
                .class
                .as_ref()
                .ok_or_else(|| JVMError::Other("Object has no class".to_string()))?;
            if !self.is_compatible_class(actual_class, &expected_class_name) {
                return Err(JVMError::IncompatibleClass {
                    expected: expected_class_name.to_string(),
                    found: actual_class.class_name.clone(),
                });
            }
            obj.set_field(&name, value)?;
            vm.memory_snap().await;
            Ok(ExecutionResult::Continue)
        } else {
            Err(JVMError::NullReference)
        }
    }

    pub async fn getfield(&mut self, index: u16) -> Result<ExecutionResult, JVMError> {
        let cp_entry = self.constant_pool.get_entry(index).ok_or_else(|| {
            JVMError::ConstantPoolIndexOutOfBounds {
                index,
                max: self.constant_pool.get_len(),
            }
        })?;
        let (class_index, name_and_type_index) = match cp_entry {
            ConstantInfo::FieldRef(ConstantFieldRefInfo {
                class_index,
                name_and_type_index,
            }) => (class_index, name_and_type_index),
            _ => {
                return Err(JVMError::InvalidConstantType {
                    expected: "Fieldref",
                    found: "other",
                })
            }
        };
        let expected_class_name = self
            .constant_pool
            .get_underlying_string_from_constant_class_info_index(*class_index)
            .ok_or_else(|| JVMError::InvalidConstantType {
                expected: "Class",
                found: "missing name",
            })?
            .to_string();
        let (name, descriptor) = self.get_name_and_type(*name_and_type_index)?;

        let obj_ref = self.pop_expect_reference()?;
        if let Some(obj) = &obj_ref {
            let actual_class = obj
                .class
                .as_ref()
                .ok_or_else(|| JVMError::Other("Object has no class".to_string()))?;
            if !self.is_compatible_class(actual_class, &expected_class_name) {
                return Err(JVMError::IncompatibleClass {
                    expected: expected_class_name.to_string(),
                    found: actual_class.class_name.clone(),
                });
            }
            let value = obj.get_field(&name)?;
            self.push(value.clone())?;
            Ok(ExecutionResult::Continue)
        } else {
            Err(JVMError::NullReference)
        }
    }

    pub async fn putstatic(&mut self, index: u16, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let cp_entry = self.constant_pool.get_entry(index).ok_or_else(|| {
            JVMError::ConstantPoolIndexOutOfBounds {
                index,
                max: self.constant_pool.get_len(),
            }
        })?;
        let (class_index, name_and_type_index) = match cp_entry {
            ConstantInfo::FieldRef(ConstantFieldRefInfo {
                class_index,
                name_and_type_index,
            }) => (class_index, name_and_type_index),
            _ => {
                return Err(JVMError::InvalidConstantType {
                    expected: "Fieldref",
                    found: "other",
                })
            }
        };
        let class_name = self
            .constant_pool
            .get_underlying_string_from_constant_class_info_index(*class_index)
            .ok_or_else(|| JVMError::InvalidConstantType {
                expected: "Class",
                found: "missing name",
            })?
            .to_string();
        let (name, descriptor) = self.get_name_and_type(*name_and_type_index)?;
        let expected_type = descriptor.split(';').next().unwrap_or(&descriptor);

        let value = match expected_type {
            "Z" | "B" | "C" | "S" | "I" => self.pop_expect("I")?,
            "J" => self.pop_expect("J")?,
            "F" => self.pop_expect("F")?,
            "D" => self.pop_expect("D")?,
            t if t.starts_with("L") || t.starts_with("[") => self.pop_expect("L")?,
            _ => {
                return Err(JVMError::Other(format!(
                    "Unsupported field descriptor: {}",
                    descriptor
                )))
            }
        };

        let fut = Box::pin(vm.class_loader.load_class(&class_name, vm));
        let class = fut.await.unwrap();
        class.set_static_field(&name, value)?;
        vm.memory_snap().await;
        Ok(ExecutionResult::Continue)
    }

    pub async fn getstatic(&mut self, index: u16, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let cp_entry = self.constant_pool.get_entry(index).ok_or_else(|| {
            JVMError::ConstantPoolIndexOutOfBounds {
                index,
                max: self.constant_pool.get_len(),
            }
        })?;
        let (class_index, name_and_type_index) = match cp_entry {
            ConstantInfo::FieldRef(ConstantFieldRefInfo {
                class_index,
                name_and_type_index,
            }) => (class_index, name_and_type_index),
            _ => {
                return Err(JVMError::InvalidConstantType {
                    expected: "Fieldref",
                    found: "other",
                })
            }
        };
        let class_name = self
            .constant_pool
            .get_underlying_string_from_constant_class_info_index(*class_index)
            .ok_or_else(|| JVMError::InvalidConstantType {
                expected: "Class",
                found: "missing name",
            })?
            .to_string();
        let (name, _descriptor) = self.get_name_and_type(*name_and_type_index)?;

        let fut = Box::pin(vm.class_loader.load_class(&class_name, vm));
        let class = fut.await.unwrap();
        let value = class.get_static_field(&name)?;
        self.push(value)?;
        Ok(ExecutionResult::Continue)
    }
}
