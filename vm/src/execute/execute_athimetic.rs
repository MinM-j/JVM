use crate::jvm_error::JVMError;
use crate::runtime::*;

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

    pub fn iadd(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_int()?;
        let v1 = self.pop_expect_int()?;

        let result = v1.checked_add(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Int(result))
    }

    pub fn isub(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_int()?;
        let v1 = self.pop_expect_int()?;

        let result = v1.checked_sub(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Int(result))
    }

    pub fn imul(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_int()?;
        let v1 = self.pop_expect_int()?;

        let result = v1.checked_mul(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Int(result))
    }

    pub fn idiv(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_int()?;
        let v1 = self.pop_expect_int()?;

        if v2 == 0 {
            return Err(JVMError::DivisionByZero);
        }
        let result = v1.checked_div(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Int(result))
    }

    pub fn irem(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_int()?;
        let v1 = self.pop_expect_int()?;

        if v2 == 0 {
            return Err(JVMError::DivisionByZero);
        }
        let result = v1.checked_rem(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Int(result))
    }

    pub fn ineg(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(1)?;
        let v = self.pop_expect_int()?;

        let result = v.checked_neg().ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Int(result))
    }

    pub fn iinc(&mut self, index: u8, value: i8) -> Result<(), JVMError> {
        let current = match self.get_local(index as usize).cloned() {
            Some(Value::Int(val)) => val,
            Some(other) => {
                return Err(JVMError::TypeMismatch {
                    expected: "Int",
                    found: Self::get_value_type(&other),
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
        Ok(())
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

    pub fn ladd(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_long()?;
        let v1 = self.pop_expect_long()?;
        let result = v1.checked_add(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Long(result))?;
        Ok(())
    }

    pub fn lsub(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_long()?;
        let v1 = self.pop_expect_long()?;
        let result = v1.checked_sub(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Long(result))?;
        Ok(())
    }

    pub fn lmul(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_long()?;
        let v1 = self.pop_expect_long()?;
        let result = v1.checked_mul(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Long(result))?;
        Ok(())
    }

    pub fn ldiv(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_long()?;
        let v1 = self.pop_expect_long()?;
        if v2 == 0 {
            return Err(JVMError::DivisionByZero);
        }
        let result = v1.checked_div(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Long(result))?;
        Ok(())
    }

    pub fn lrem(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_long()?;
        let v1 = self.pop_expect_long()?;
        if v2 == 0 {
            return Err(JVMError::DivisionByZero);
        }
        let result = v1.checked_rem(v2).ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Long(result))?;
        Ok(())
    }

    pub fn lneg(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(1)?;
        let v = self.pop_expect_long()?;
        let result = v.checked_neg().ok_or(JVMError::ArithmeticOverflow)?;
        self.push(Value::Long(result))?;
        Ok(())
    }

    pub fn pop_expect_float(&mut self) -> Result<f32, JVMError> {
        match self.pop()? {
            Value::Float(v) => Ok(v),
            other => Err(JVMError::TypeMismatch {
                expected: "Float",
                found: Self::get_value_type(&other),
            }),
        }
    }

    pub fn fadd(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_float()?;
        let v1 = self.pop_expect_float()?;
        self.push(Value::Float(v1 + v2))?;
        Ok(())
    }

    pub fn fsub(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_float()?;
        let v1 = self.pop_expect_float()?;
        self.push(Value::Float(v1 - v2))?;
        Ok(())
    }

    pub fn fmul(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_float()?;
        let v1 = self.pop_expect_float()?;
        self.push(Value::Float(v1 * v2))?;
        Ok(())
    }

    pub fn fdiv(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_float()?;
        let v1 = self.pop_expect_float()?;
        if v2 == 0.0 {
            return Err(JVMError::DivisionByZero);
        }
        self.push(Value::Float(v1 / v2))?;
        Ok(())
    }

    pub fn frem(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_float()?;
        let v1 = self.pop_expect_float()?;
        if v2 == 0.0 {
            return Err(JVMError::DivisionByZero);
        }
        self.push(Value::Float(v1 % v2))?;
        Ok(())
    }

    pub fn fneg(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(1)?;
        let v = self.pop_expect_float()?;
        self.push(Value::Float(-v))?;
        Ok(())
    }

    pub fn pop_expect_double(&mut self) -> Result<f64, JVMError> {
        match self.pop()? {
            Value::Double(v) => Ok(v),
            other => Err(JVMError::TypeMismatch {
                expected: "Double",
                found: Self::get_value_type(&other),
            }),
        }
    }

    pub fn dadd(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_double()?;
        let v1 = self.pop_expect_double()?;
        self.push(Value::Double(v1 + v2))?;
        Ok(())
    }

    pub fn dsub(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_double()?;
        let v1 = self.pop_expect_double()?;
        self.push(Value::Double(v1 - v2))?;
        Ok(())
    }

    pub fn dmul(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_double()?;
        let v1 = self.pop_expect_double()?;
        self.push(Value::Double(v1 * v2))?;
        Ok(())
    }

    pub fn ddiv(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_double()?;
        let v1 = self.pop_expect_double()?;
        if v2 == 0.0 {
            return Err(JVMError::DivisionByZero);
        }
        self.push(Value::Double(v1 / v2))?;
        Ok(())
    }

    pub fn drem(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(2)?;
        let v2 = self.pop_expect_double()?;
        let v1 = self.pop_expect_double()?;
        if v2 == 0.0 {
            return Err(JVMError::DivisionByZero);
        }
        self.push(Value::Double(v1 % v2))?;
        Ok(())
    }

    pub fn dneg(&mut self) -> Result<(), JVMError> {
        self.ensure_operands(1)?;
        let v = self.pop_expect_double()?;
        self.push(Value::Double(-v))?;
        Ok(())
    }
}
