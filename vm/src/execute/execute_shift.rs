use super::execute::ExecutionResult;
use crate::jvm_error::JVMError;
use crate::runtime::*;

impl Frame {
    pub fn ishl(&mut self) -> Result<ExecutionResult, JVMError> {
        let shift = self.pop_expect_int()? & 0x1f; 
        let value = self.pop_expect_int()?;
        self.push(Value::Int(value << shift))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn ishr(&mut self) -> Result<ExecutionResult, JVMError> {
        let shift = self.pop_expect_int()? & 0x1f; 
        let value = self.pop_expect_int()?;
        self.push(Value::Int(value >> shift))?; 
        Ok(ExecutionResult::Continue)
    }

    pub fn iushr(&mut self) -> Result<ExecutionResult, JVMError> {
        let shift = self.pop_expect_int()? & 0x1f; 
        let value = self.pop_expect_int()?;
        self.push(Value::Int(((value as u32) >> shift) as i32))?; 
        Ok(ExecutionResult::Continue)
    }

    pub fn lshl(&mut self) -> Result<ExecutionResult, JVMError> {
        let shift = self.pop_expect_int()? & 0x3f; 
        let value = self.pop_expect_long()?;
        self.push(Value::Long(value << shift))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn lshr(&mut self) -> Result<ExecutionResult, JVMError> {
        let shift = self.pop_expect_int()? & 0x3f; 
        let value = self.pop_expect_long()?;
        self.push(Value::Long(value >> shift))?; 
        Ok(ExecutionResult::Continue)
    }

    pub fn lushr(&mut self) -> Result<ExecutionResult, JVMError> {
        let shift = self.pop_expect_int()? & 0x3f; 
        let value = self.pop_expect_long()?;
        self.push(Value::Long(((value as u64) >> shift) as i64))?; 
        Ok(ExecutionResult::Continue)
    }

    pub fn ior(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop_expect_int()?;
        let value2 = self.pop_expect_int()?;
        self.push(Value::Int(value1 | value2))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn lor(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop_expect_long()?;
        let value2 = self.pop_expect_long()?;
        self.push(Value::Long(value1 | value2))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn ixor(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop_expect_int()?;
        let value2 = self.pop_expect_int()?;
        self.push(Value::Int(value1 ^ value2))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn lxor(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop_expect_long()?;
        let value2 = self.pop_expect_long()?;
        self.push(Value::Long(value1 ^ value2))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn iand(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop_expect_int()?;
        let value2 = self.pop_expect_int()?;
        self.push(Value::Int(value1 & value2))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn land(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop_expect_long()?;
        let value2 = self.pop_expect_long()?;
        self.push(Value::Long(value1 & value2))?;
        Ok(ExecutionResult::Continue)
    }
}
