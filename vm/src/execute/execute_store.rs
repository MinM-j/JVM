use crate::runtime::*;
use crate::jvm_error::JVMError;

use super::execute::ExecutionResult;

impl Frame {
    pub fn istore(&mut self, index: u8) -> Result<ExecutionResult, JVMError> {
        let index = index as usize;
        self.check_local_index(index)?;

        match self.pop()? {
            Value::Int(value) => {
                self.locals[index] = Value::Int(value);
                Ok(ExecutionResult::Continue)
            }
            other => Err(JVMError::TypeMismatch {
                expected: "int".to_string(),
                found: Self::get_value_type(&other).to_string(),
            }),
        }
    }

    pub fn lstore(&mut self, index: u8) -> Result<ExecutionResult, JVMError> {
        let index = index as usize;
        self.check_local_index(index)?;

        match self.pop()? {
            Value::Long(value) => {
                self.locals[index] = Value::Long(value);
                Ok(ExecutionResult::Continue)
            }
            other => Err(JVMError::TypeMismatch {
                expected: "long".to_string(),
                found: Self::get_value_type(&other).to_string(),
            }),
        }
    }

    pub fn fstore(&mut self, index: u8) -> Result<ExecutionResult, JVMError> {
        let index = index as usize;
        self.check_local_index(index)?;

        match self.pop()? {
            Value::Float(value) => {
                self.locals[index] = Value::Float(value);
                Ok(ExecutionResult::Continue)
            }
            other => Err(JVMError::TypeMismatch {
                expected: "float".to_string(),
                found: Self::get_value_type(&other).to_string(),
            }),
        }
    }

    pub fn dstore(&mut self, index: u8) -> Result<ExecutionResult, JVMError> {
        let index = index as usize;
        self.check_local_index(index)?;

        match self.pop()? {
            Value::Double(value) => {
                self.locals[index] = Value::Double(value);
                Ok(ExecutionResult::Continue)
            }
            other => Err(JVMError::TypeMismatch {
                expected: "double".to_string(),
                found: Self::get_value_type(&other).to_string(),
            }),
        }
    }

    pub fn astore(&mut self, index: u8) -> Result<ExecutionResult, JVMError> {
        let index = index as usize;
        self.check_local_index(index)?;

        match self.pop()? {
            Value::Reference(value) => {
                self.locals[index] = Value::Reference(value);
                Ok(ExecutionResult::Continue)
            }
            other => Err(JVMError::TypeMismatch {
                expected: "reference".to_string(),
                found: Self::get_value_type(&other).to_string(),
            }),
        }
    }

}
