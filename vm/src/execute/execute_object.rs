use crate::runtime::*;
use crate::jvm_error::JVMError;
use super::execute::ExecutionResult;
use parser::constant_pool::{ConstantInfo, ConstantClassInfo};
use crate::vm::VM;

impl Frame {
    pub async fn execute_new(&mut self, index: u16, stack: &Stack, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let cp_entry = self.constant_pool.get_entry(index).ok_or_else(|| {
            JVMError::ConstantPoolIndexOutOfBounds {
                index,
                max: self.constant_pool.get_len(),
            }
        })?;
        let class_name = match cp_entry {
            ConstantInfo::Class(ConstantClassInfo (name_index)) => {
                self.constant_pool
                    .get_underlying_string_from_utf8_index(*name_index)
                    .ok_or_else(|| JVMError::Other(format!("Invalid name_index {}", name_index)))?
            }
            _ => {
                return Err(JVMError::InvalidConstantType {
                    expected: "Class",
                    found: "other",
                })
            }
        };
        //println!("{class_name}");

        let fut = Box::pin(vm.allocate_object(stack, class_name));
        let obj_ref = fut.await?;
        self.push(obj_ref)?;
        Ok(ExecutionResult::Continue)
    }

    pub fn dup(&mut self) -> Result<ExecutionResult, JVMError> {
        let value = self.pop()?;
        self.push(value.clone())?;
        self.push(value)?;
        Ok(ExecutionResult::Continue)
    }

    pub fn dup_x1(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop()?;
        let value2 = self.pop()?;
        self.push(value1.clone())?;
        self.push(value2)?;
        self.push(value1)?;
        Ok(ExecutionResult::Continue)
    }

    pub fn dup_x2(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop()?;
        let value2 = self.pop()?;
        let value3 = self.pop()?;
        self.push(value1.clone())?;
        self.push(value3)?;
        self.push(value2)?;
        self.push(value1)?;
        Ok(ExecutionResult::Continue)
    }

    pub fn dup2(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop()?;
        let value2 = self.pop()?;
        self.push(value2.clone())?;
        self.push(value1.clone())?;
        self.push(value2)?;
        self.push(value1)?;
        Ok(ExecutionResult::Continue)
    }

    pub fn dup2_x1(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop()?;
        let value2 = self.pop()?;
        let value3 = self.pop()?;
        self.push(value2.clone())?;
        self.push(value1.clone())?;
        self.push(value3)?;
        self.push(value2)?;
        self.push(value1)?;
        Ok(ExecutionResult::Continue)
    }

    pub fn dup2_x2(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop()?;
        let value2 = self.pop()?;
        let value3 = self.pop()?;
        let value4 = self.pop()?;
        self.push(value2.clone())?;
        self.push(value1.clone())?;
        self.push(value4)?;
        self.push(value3)?;
        self.push(value2)?;
        self.push(value1)?;
        Ok(ExecutionResult::Continue)
    }
}
