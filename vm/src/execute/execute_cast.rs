use super::execute::ExecutionResult;
use crate::class_loader::loaded_class::LoadedClass;
use crate::jvm_error::JVMError;
use crate::runtime::*;
use crate::vm::VM;
use std::sync::Arc;

impl Frame {
    pub async fn checkcast(&mut self, index: u16, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let ref_value = self.pop()?;
        match ref_value {
            Value::Reference(None) => {
                self.push(Value::Reference(None))?;
                Ok(ExecutionResult::Continue)
            }
            Value::Reference(Some(obj)) => {
                let target_class_name = self
                    .constant_pool
                    .get_underlying_string_from_constant_class_info_index(index)
                    .ok_or_else(|| JVMError::Other(format!("Invalid class index: {}", index)))?;
                let fut = Box::pin(vm.class_loader.load_class(&target_class_name, vm));
                let target_class = fut.await.map_err(|e| JVMError::Other(e.to_string()))?;

                let obj_class = obj
                    .class
                    .as_ref()
                    .ok_or_else(|| JVMError::Other("Object has no class".to_string()))?;

                if self.is_assignable(obj_class, &target_class) {
                    self.push(Value::Reference(Some(obj)))?;
                    Ok(ExecutionResult::Continue)
                } else {
                    Err(JVMError::ClassCastException(format!(
                        "Cannot cast {} to {}",
                        obj_class.class_name, target_class_name
                    )))
                }
            }
            _ => Err(JVMError::Other(
                "Checkcast requires a reference type".to_string(),
            )),
        }
    }

    pub async fn instanceof(&mut self, index: u16, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let ref_value = self.pop()?;
        match ref_value {
            Value::Reference(None) => {
                self.push(Value::Int(0))?;
                Ok(ExecutionResult::Continue)
            }
            Value::Reference(Some(obj)) => {
                let target_class_name = self
                    .constant_pool
                    .get_underlying_string_from_constant_class_info_index(index)
                    .ok_or_else(|| JVMError::Other(format!("Invalid class index: {}", index)))?;
                let fut = Box::pin(vm.class_loader.load_class(&target_class_name, vm));
                let target_class = fut.await.map_err(|e| JVMError::Other(e.to_string()))?;

                let obj_class = obj
                    .class
                    .as_ref()
                    .ok_or_else(|| JVMError::Other("Object has no class".to_string()))?;

                let is_instance = self.is_assignable(obj_class, &target_class);
                self.push(Value::Int(if is_instance { 1 } else { 0 }))?;
                Ok(ExecutionResult::Continue)
            }
            _ => Err(JVMError::Other(
                "Instanceof requires a reference type".to_string(),
            )),
        }
    }

    fn is_assignable(&self, from_class: &Arc<LoadedClass>, to_class: &Arc<LoadedClass>) -> bool {
        if from_class.class_name == to_class.class_name {
            return true;
        }
        let mut current = from_class.super_class.clone();
        while let Some(super_class) = current {
            if super_class.class_name == to_class.class_name {
                return true;
            }
            current = super_class.super_class.clone();
        }
        false
    }
}
