use super::execute::ExecutionResult;
use crate::jvm_error::JVMError;
use crate::object::Object;
use crate::runtime::*;
use std::sync::Arc;

impl Frame {
    pub fn branch_16(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        let address = self.code.get_address_at_index(self.pc) as u16;
        let new_address = address.wrapping_add(offset as u16) as u32;
        let new_pc = (self.code.get_index_at_address(new_address) - 1) as usize;
        if new_pc >= self.code.code.len() {
            return Err(JVMError::InvalidOffset(offset as i32));
        }
        self.pc = new_pc;
        Ok(ExecutionResult::Continue)
    }

    pub fn branch_32(&mut self, offset: i32) -> Result<ExecutionResult, JVMError> {
        let address = self.code.get_address_at_index(self.pc) as u32;
        let new_address = address.wrapping_add(offset as u32) as u32;
        let new_pc = (self.code.get_index_at_address(new_address) - 1) as usize;
        if new_pc >= self.code.code.len() {
            return Err(JVMError::InvalidOffset(offset as i32));
        }
        self.pc = new_pc;
        Ok(ExecutionResult::Continue)
    }

    pub fn pop_expect_reference(&mut self) -> Result<Option<Arc<Object>>, JVMError> {
        match self.pop()? {
            Value::Reference(r) => Ok(r),
            other => Err(JVMError::InvalidOperandType {
                expected: "reference",
                found: Self::get_value_type(&other),
            }),
        }
    }

    pub fn ifeq(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(1)?;
        let value = self.pop_expect_int()?;
        if value == 0 {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn ifne(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(1)?;
        let value = self.pop_expect_int()?;
        if value != 0 {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn iflt(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(1)?;
        let value = self.pop_expect_int()?;
        if value < 0 {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn ifge(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(1)?;
        let value = self.pop_expect_int()?;
        if value >= 0 {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn ifgt(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(1)?;
        let value = self.pop_expect_int()?;
        if value > 0 {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn ifle(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(1)?;
        let value = self.pop_expect_int()?;
        if value <= 0 {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn ifnull(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(1)?;
        let reference = self.pop_expect_reference()?;
        if reference.is_none() {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn ifnonnull(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(1)?;
        let reference = self.pop_expect_reference()?;
        if reference.is_some() {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn if_icmpeq(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let value2 = self.pop_expect_int()?;
        let value1 = self.pop_expect_int()?;
        if value1 == value2 {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn if_icmpne(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let value2 = self.pop_expect_int()?;
        let value1 = self.pop_expect_int()?;
        if value1 != value2 {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn if_icmplt(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let value2 = self.pop_expect_int()?;
        let value1 = self.pop_expect_int()?;
        if value1 < value2 {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn if_icmpge(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let value2 = self.pop_expect_int()?;
        let value1 = self.pop_expect_int()?;
        if value1 >= value2 {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn if_icmpgt(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let value2 = self.pop_expect_int()?;
        let value1 = self.pop_expect_int()?;
        if value1 > value2 {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn if_icmple(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let value2 = self.pop_expect_int()?;
        let value1 = self.pop_expect_int()?;
        if value1 <= value2 {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn if_acmpeq(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let ref2 = self.pop_expect_reference()?;
        let ref1 = self.pop_expect_reference()?;
        if std::ptr::eq(&ref1, &ref2) {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn if_acmpne(&mut self, offset: i16) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let ref2 = self.pop_expect_reference()?;
        let ref1 = self.pop_expect_reference()?;
        if !std::ptr::eq(&ref1, &ref2) {
            self.branch_16(offset)?;
        }
        Ok(ExecutionResult::Continue)
    }

    pub fn lookup_switch(
        &mut self,
        default_offset: i32,
        pairs: &Vec<(i32, i32)>,
    ) -> Result<ExecutionResult, JVMError> {
        let key = self.pop_expect_int()?;

        let offset = pairs
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, offset)| *offset)
            .unwrap_or(default_offset);

        self.branch_32(offset)?;
        Ok(ExecutionResult::Continue)
    }

    pub fn table_switch(
        &mut self,
        default_offset: i32,
        low: i32,
        high: i32,
        jump_offsets: &Vec<i32>,
    ) -> Result<ExecutionResult, JVMError> {
        let index = self.pop_expect_int()?;
        let offset = if index < low || index > high {
            default_offset
        } else {
            let table_index = (index - low) as usize;
            jump_offsets[table_index]
        };

        self.branch_32(offset)?;
        Ok(ExecutionResult::Continue)
    }
}
