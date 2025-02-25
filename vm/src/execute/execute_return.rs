use crate::jvm_error::JVMError;
use crate::runtime::*;

use super::execute::ExecutionResult;

impl Frame {
    pub fn return_void (&mut self) -> Result<ExecutionResult, JVMError> {
        Ok(ExecutionResult::Return(None))
    }

    pub fn return_int(&mut self) -> Result<ExecutionResult, JVMError> {
        let value = Value::Int(self.pop_expect_int()?);
        Ok(ExecutionResult::Return(Some(value)))
    }

    pub fn return_long(&mut self) -> Result<ExecutionResult, JVMError> {
        let value = Value::Long(self.pop_expect_long()?);
        Ok(ExecutionResult::Return(Some(value)))
    }

    pub fn return_float(&mut self) -> Result<ExecutionResult, JVMError> {
        let value = Value::Float(self.pop_expect_float()?);
        Ok(ExecutionResult::Return(Some(value)))
    }

    pub fn return_double(&mut self) -> Result<ExecutionResult, JVMError> {
        let value = Value::Double(self.pop_expect_double()?);
        Ok(ExecutionResult::Return(Some(value)))
    }

    pub fn return_reference(&mut self) -> Result<ExecutionResult, JVMError> {
        let value = Value::Reference(self.pop_expect_reference()?);
        Ok(ExecutionResult::Return(Some(value)))
    }
}
