use crate::jvm_error::JVMError;
use crate::runtime::*;
use crate::vm::VM;

impl Frame {
    pub fn return_void (&mut self, vm: &VM) -> Result<(), JVMError> {
        vm.stack.borrow_mut().pop_frame()?;
        Ok(())
    }

    pub fn return_int(&mut self, vm: &VM) -> Result<(), JVMError> {
        let value = Value::Int(self.pop_expect_int()?);
        vm.stack.borrow_mut().pop_frame()?;
        if let Some(frame) = vm.stack.borrow_mut().frames.last_mut() {
            frame.push(value)?;
        }
        Ok(())
    }

    pub fn return_long(&mut self, vm: &VM) -> Result<(), JVMError> {
        let value = Value::Long(self.pop_expect_long()?);
        vm.stack.borrow_mut().pop_frame()?;
        if let Some(frame) = vm.stack.borrow_mut().frames.last_mut() {
            frame.push(value)?;
        }
        Ok(())
    }

    pub fn return_float(&mut self, vm: &VM) -> Result<(), JVMError> {
        let value = Value::Float(self.pop_expect_float()?);
        vm.stack.borrow_mut().pop_frame()?;
        if let Some(frame) = vm.stack.borrow_mut().frames.last_mut() {
            frame.push(value)?;
        }
        Ok(())
    }

    pub fn return_double(&mut self, vm: &VM) -> Result<(), JVMError> {
        let value = Value::Double(self.pop_expect_double()?);
        vm.stack.borrow_mut().pop_frame()?;
        if let Some(frame) = vm.stack.borrow_mut().frames.last_mut() {
            frame.push(value)?;
        }
        Ok(())
    }

    pub fn return_reference(&mut self, vm: &VM) -> Result<(), JVMError> {
        let value = Value::Reference(self.pop_expect_reference()?);
        vm.stack.borrow_mut().pop_frame()?;
        if let Some(frame) = vm.stack.borrow_mut().frames.last_mut() {
            frame.push(value)?;
        }
        Ok(())
    }
}
