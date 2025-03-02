use super::execute::ExecutionResult;
use crate::jvm_error::JVMError;
use crate::runtime::*;
use crate::vm::VM;

impl Frame {
    pub fn ensure_operands(&self, required: usize) -> Result<(), JVMError> {
        if self.operands.len() < required {
            return Err(JVMError::InsufficientOperands {
                required,
                found: self.operands.len(),
            });
        }
        Ok(())
    }

    pub fn pop_expect_int(&mut self) -> Result<i32, JVMError> {
        match self.pop()? {
            Value::Int(v) => Ok(v),
            other => Err(JVMError::InvalidOperandType {
                expected: "int",
                found: Self::get_value_type(&other),
            }),
        }
    }

    pub fn iadd(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_int()?;
        let v1 = self.pop_expect_int()?;

        let result = v1.checked_add(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Int(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn isub(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_int()?;
        let v1 = self.pop_expect_int()?;

        let result = v1.checked_sub(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Int(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn imul(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_int()?;
        let v1 = self.pop_expect_int()?;

        let result = v1.checked_mul(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Int(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub async fn idiv(&mut self, vm: &VM) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_int()?;
        let v1 = self.pop_expect_int()?;

        if v2 == 0 {
            //let fut = Box::pin(vm.allocate_object("java/lang/ArithmeticException"));
            //let exception = fut.await?;
            return Ok(ExecutionResult::Throw("java/lang/ArithmeticException".to_string()));
            //return Err(JVMError::DivisionByZero);
        }
        let result = v1.checked_div(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Int(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn irem(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_int()?;
        let v1 = self.pop_expect_int()?;

        if v2 == 0 {
            return Err(JVMError::DivisionByZero);
        }
        let result = v1.checked_rem(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Int(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn ineg(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(1)?;
        let v = self.pop_expect_int()?;

        let result = v.checked_neg().ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Int(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn iinc(&mut self, index: u8, value: i8) -> Result<ExecutionResult, JVMError> {
        let current = match self.get_local(index as usize).cloned() {
            Some(Value::Int(val)) => val,
            Some(other) => {
                return Err(JVMError::TypeMismatch {
                    expected: "Int".to_string(),
                    found: Self::get_value_type(&other).to_string(),
                })
            }
            None => {
                return Err(JVMError::InvalidLocalVariable {
                    index: index as usize,
                })
            }
        };

        let result = current
            .checked_add(value as i32)
            .ok_or(JVMError::ArithmeticOverflow)?;
        self.set_local(index as usize, Value::Int(result));
        Ok(ExecutionResult::Continue)
    }

    pub fn pop_expect_long(&mut self) -> Result<i64, JVMError> {
        match self.pop()? {
            Value::Long(v) => Ok(v),
            other => Err(JVMError::InvalidOperandType {
                expected: "Long",
                found: Self::get_value_type(&other),
            }),
        }
    }

    pub fn ladd(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_long()?;
        let v1 = self.pop_expect_long()?;
        let result = v1.checked_add(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Long(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn lsub(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_long()?;
        let v1 = self.pop_expect_long()?;
        let result = v1.checked_sub(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Long(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn lmul(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_long()?;
        let v1 = self.pop_expect_long()?;
        let result = v1.checked_mul(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Long(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub async fn ldiv(&mut self, vm: &VM) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_long()?;
        let v1 = self.pop_expect_long()?;
        if v2 == 0 {
            //let fut = Box::pin(vm.allocate_object("java/lang/ArithmeticException"));
            //let exception = fut.await?;
            return Ok(ExecutionResult::Throw("java/lang/ArithmeticException".to_string()));
            //return Err(JVMError::DivisionByZero);
        }
        let result = v1.checked_div(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Long(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn lrem(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_long()?;
        let v1 = self.pop_expect_long()?;
        if v2 == 0 {
            return Err(JVMError::DivisionByZero);
        }
        let result = v1.checked_rem(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Long(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn lneg(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(1)?;
        let v = self.pop_expect_long()?;
        let result = v.checked_neg().ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Long(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn pop_expect_float(&mut self) -> Result<f32, JVMError> {
        match self.pop()? {
            Value::Float(v) => Ok(v),
            other => Err(JVMError::TypeMismatch {
                expected: "Float".to_string(),
                found: Self::get_value_type(&other).to_string(),
            }),
        }
    }

    pub fn fadd(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_float()?;
        let v1 = self.pop_expect_float()?;
        self.push(Value::Float(v1 + v2))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn fsub(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_float()?;
        let v1 = self.pop_expect_float()?;
        self.push(Value::Float(v1 - v2))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn fmul(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_float()?;
        let v1 = self.pop_expect_float()?;
        self.push(Value::Float(v1 * v2))?;
        Ok(ExecutionResult::Continue)
    }

    pub async fn fdiv(&mut self, vm: &VM) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_float()?;
        let v1 = self.pop_expect_float()?;
        if v2 == 0.0 {
            //let fut = Box::pin(vm.allocate_object("java/lang/ArithmeticException"));
            //let exception = fut.await?;
            return Ok(ExecutionResult::Throw("java/lang/ArithmeticException".to_string()));
            //return Err(JVMError::DivisionByZero);
        }
        self.push(Value::Float(v1 / v2))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn frem(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_float()?;
        let v1 = self.pop_expect_float()?;
        if v2 == 0.0 {
            return Err(JVMError::DivisionByZero);
        }
        self.push(Value::Float(v1 % v2))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn fneg(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(1)?;
        let v = self.pop_expect_float()?;
        self.push(Value::Float(-v))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn fcmpg(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop_expect_float()?;
        let value2 = self.pop_expect_float()?;
        let result = if value1.is_nan() || value2.is_nan() {
            1
        } else if value1 > value2 {
            1
        } else if value1 < value2 {
            -1
        } else {
            0
        };
        self.push(Value::Int(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn fcmpl(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop_expect_float()?;
        let value2 = self.pop_expect_float()?;
        let result = if value1.is_nan() || value2.is_nan() {
            -1
        } else if value1 > value2 {
            1
        } else if value1 < value2 {
            -1
        } else {
            0
        };
        self.push(Value::Int(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn pop_expect_double(&mut self) -> Result<f64, JVMError> {
        match self.pop()? {
            Value::Double(v) => Ok(v),
            other => Err(JVMError::TypeMismatch {
                expected: "Double".to_string(),
                found: Self::get_value_type(&other).to_string(),
            }),
        }
    }

    pub fn dadd(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_double()?;
        let v1 = self.pop_expect_double()?;
        self.push(Value::Double(v1 + v2))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn dsub(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_double()?;
        let v1 = self.pop_expect_double()?;
        self.push(Value::Double(v1 - v2))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn dmul(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_double()?;
        let v1 = self.pop_expect_double()?;
        self.push(Value::Double(v1 * v2))?;
        Ok(ExecutionResult::Continue)
    }

    pub async fn ddiv(&mut self, vm: &VM) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_double()?;
        let v1 = self.pop_expect_double()?;
        if v2 == 0.0 {
            //let fut = Box::pin(vm.allocate_object("java/lang/ArithmeticException"));
            //let exception = fut.await?;
            return Ok(ExecutionResult::Throw("java/lang/ArithmeticException".to_string()));
            //return Err(JVMError::DivisionByZero);
        }
        self.push(Value::Double(v1 / v2))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn drem(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_double()?;
        let v1 = self.pop_expect_double()?;
        if v2 == 0.0 {
            return Err(JVMError::DivisionByZero);
        }
        self.push(Value::Double(v1 % v2))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn dneg(&mut self) -> Result<ExecutionResult, JVMError> {
        self.ensure_operands(1)?;
        let v = self.pop_expect_double()?;
        self.push(Value::Double(-v))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn dcmpg(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop_expect_double()?;
        let value2 = self.pop_expect_double()?;
        let result = if value1.is_nan() || value2.is_nan() {
            1
        } else if value1 > value2 {
            1
        } else if value1 < value2 {
            -1
        } else {
            0
        };
        self.push(Value::Int(result))?;
        Ok(ExecutionResult::Continue)
    }

    pub fn dcmpl(&mut self) -> Result<ExecutionResult, JVMError> {
        let value1 = self.pop_expect_double()?;
        let value2 = self.pop_expect_double()?;
        let result = if value1.is_nan() || value2.is_nan() {
            -1
        } else if value1 > value2 {
            1
        } else if value1 < value2 {
            -1
        } else {
            0
        };
        self.push(Value::Int(result))?;
        Ok(ExecutionResult::Continue)
    }
}
