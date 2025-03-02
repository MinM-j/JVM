use super::execute::ExecutionResult;
use crate::jvm_error::JVMError;
use crate::object::Object;
use crate::{class_loader::loaded_class::LoadedClass, runtime::*, vm::VM};
use std::sync::Arc;

impl Frame {
    /*
    pub async fn find_exception_handler(&self, exception: &Arc<Object>, vm: &VM) -> Option<usize> {
        for entry in &self.code.exception_table {
            let pc = self.code.get_address_at_index(self.pc);
            if pc >= entry.start_pc.into() && pc < entry.end_pc.into() {
                if entry.catch_type == 0 {
                    return Some(entry.handler_pc as usize);
                }
                let catch_class_name = self
                    .constant_pool
                    .get_underlying_string_from_constant_class_info_index(entry.catch_type)
                    .unwrap();
                let fut = Box::pin(vm.class_loader.load_class(catch_class_name, vm));
                let catch_class = fut.await.unwrap();
                if let Some(exception_class) = &exception.class {
                    if self.is_subclass_of(&exception_class, &catch_class) {
                        return Some(entry.handler_pc as usize);
                    }
                }
            }
        }
        None
    }
    */
    pub async fn find_exception_handler(
        &self,
        exception_class_name: &str,
        vm: &VM,
    ) -> Option<usize> {
        for entry in &self.code.exception_table {
            let pc = self.code.get_address_at_index(self.pc);
            if pc >= entry.start_pc.into() && pc < entry.end_pc.into() {
                if entry.catch_type == 0 {
                    return Some(entry.handler_pc as usize); // Catch-all handler
                }
                if exception_class_name
                    == self
                        .constant_pool
                        .get_underlying_string_from_constant_class_info_index(entry.catch_type)
                        .unwrap()
                {
                    return Some(entry.handler_pc as usize); // Catch-all handler
                }
                /*
                                let catch_class_name = self
                                    .constant_pool
                                    .get_underlying_string_from_constant_class_info_index(entry.catch_type)
                                    .unwrap();
                                let fut = Box::pin(vm
                                    .class_loader
                                    .load_class(catch_class_name, vm));
                                let catch_class = fut
                                    .await
                                    .unwrap();
                                let fut = Box::pin(vm
                                    .class_loader
                                    .load_class(exception_class_name, vm));
                                let exception_class = fut
                                    .await
                                    .unwrap();
                                if self.is_subclass_of_temp(&exception_class, &catch_class) {
                                    return Some(entry.handler_pc as usize);
                                }
                */
            }
        }
        None
    }

    fn is_subclass_of(&self, class: &Arc<LoadedClass>, target: &Arc<LoadedClass>) -> bool {
        let mut current = Arc::clone(class);
        loop {
            if current.class_name == target.class_name {
                return true;
            }
            if let Some(super_class) = &current.super_class {
                current = Arc::clone(super_class);
            } else {
                break;
            }
        }
        for interface in &class.interfaces {
            if self.is_subclass_of(interface, target) {
                return true;
            }
        }
        false
    }

    /*
    pub async fn athrow(&mut self, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let exception = match self.pop_expect_reference()? {
            Some(value) => value,
            None => {
                return Err(JVMError::Other(
                    "Expected some reference value but got null".to_string(),
                ))
            }
        };
        if let Some(exception_class) = &exception.class {
            let fut = Box::pin(vm.class_loader.load_class("java/lang/Throwable", vm));
            let exception_super_class = fut.await.unwrap();
            if !self.is_subclass_of(&exception_class, &exception_super_class) {
                return Err(JVMError::TypeMismatch {
                    expected: "Throwable".to_string(),
                    found: "Some other class".to_string(),
                });
            }
        }
        Ok(ExecutionResult::Throw(Value::Reference(Some(exception))))
    }
    */
    pub async fn athrow(&mut self, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let exception = match self.pop_expect_reference()? {
            Some(value) => value,
            None => {
                return Err(JVMError::Other(
                    "Expected some reference value but got null".to_string(),
                ))
            }
        };
        if let Some(exception_class) = &exception.class {
            let fut = Box::pin(vm.class_loader.load_class("java/lang/Throwable", vm));
            let exception_super_class = fut.await.unwrap();
            if !self.is_subclass_of(exception_class, &exception_super_class) {
                return Err(JVMError::TypeMismatch {
                    expected: "Throwable".to_string(),
                    found: exception_class.class_name.clone(),
                });
            }
            Ok(ExecutionResult::Throw(exception_class.class_name.clone()))
        } else {
            Err(JVMError::Other("Exception object has no class".to_string()))
        }
    }

    fn is_subclass_of_temp(
        &self,
        subclass: &Arc<LoadedClass>,
        superclass: &Arc<LoadedClass>,
    ) -> bool {
        let mut current = Some(subclass.clone());
        while let Some(cls) = current {
            if Arc::ptr_eq(&cls, superclass) {
                return true;
            }
            current = cls.super_class.clone();
        }
        false
    }
}
