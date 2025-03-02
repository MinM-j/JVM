use crate::jvm_error::JVMError;
use crate::runtime::*;

use super::execute::ExecutionResult;

impl Frame {
    pub fn check_local_index(&self, index: usize) -> Result<(), JVMError> {
        if index >= self.locals.len() {
            return Err(JVMError::IndexOutOfBounds {
                index,
                max: self.locals.len(),
            });
        }
        Ok(())
    }

    pub fn get_value_type(value: &Value) -> &'static str {
        match value {
            Value::Int(_) => "int",
            Value::Long(_) => "long",
            Value::Float(_) => "float",
            Value::Double(_) => "double",
            Value::Reference(_) => "reference",
            Value::Default => "default",
        }
    }

    pub fn iload(&mut self, index: u8) -> Result<ExecutionResult, JVMError> {
        let index = index as usize;
        self.check_local_index(index)?;

        match self.locals[index] {
            Value::Int(value) => {
                self.push(Value::Int(value))?;
            }
            ref other => {
                return Err(JVMError::TypeMismatch {
                    expected: "int".to_string(),
                    found: Self::get_value_type(other).to_string(),
                })
            }
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn lload(&mut self, index: u8) -> Result<ExecutionResult, JVMError> {
        let index = index as usize;
        self.check_local_index(index)?;

        match self.locals[index] {
            Value::Long(value) => {
                self.push(Value::Long(value))?;
            }
            ref other => {
                return Err(JVMError::TypeMismatch {
                    expected: "long".to_string(),
                    found: Self::get_value_type(other).to_string(),
                })
            }
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn fload(&mut self, index: u8) -> Result<ExecutionResult, JVMError> {
        let index = index as usize;
        self.check_local_index(index)?;

        match self.locals[index] {
            Value::Float(value) => {
                self.push(Value::Float(value))?;
            }
            ref other => {
                return Err(JVMError::TypeMismatch {
                    expected: "float".to_string(),
                    found: Self::get_value_type(other).to_string(),
                })
            }
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn dload(&mut self, index: u8) -> Result<ExecutionResult, JVMError> {
        let index = index as usize;
        self.check_local_index(index)?;

        match self.locals[index] {
            Value::Double(value) => {
                self.push(Value::Double(value))?;
            }
            ref other => {
                return Err(JVMError::TypeMismatch {
                    expected: "double".to_string(),
                    found: Self::get_value_type(other).to_string(),
                })
            }
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn aload(&mut self, index: u8) -> Result<ExecutionResult, JVMError> {
        let index = index as usize;
        self.check_local_index(index)?;

        match &self.locals[index] {
            Value::Reference(_) => {
                let value = self.locals[index].clone();
                self.push(value)?;
            }
            ref other => {
                return Err(JVMError::TypeMismatch {
                    expected: "reference".to_string(),
                    found: Self::get_value_type(other).to_string(),
                })
            }
        }
        Ok(ExecutionResult::Continue)
    }
}
