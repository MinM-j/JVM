use crate::jvm_error::JVMError;
use crate::runtime::*;
use parser::constant_pool::ConstantInfo;

impl Frame {
    pub fn get_constant(&self, index: u16) -> Result<&ConstantInfo, JVMError> {
        let constant_pool_len = self.constant_pool.get_len();
        if index as usize > self.constant_pool.get_len() {
            return Err(JVMError::ConstantPoolIndexOutOfBounds {
                index,
                max: constant_pool_len,
            });
        }
        self.constant_pool
            .get_entry(index)
            .ok_or(JVMError::ConstantPoolIndexOutOfBounds {
                index,
                max: constant_pool_len,
            })
    }

    pub fn load_constant(&mut self, index: u16) -> Result<(), JVMError> {
        let constant = self.get_constant(index)?;

        match constant {
            ConstantInfo::Integer(value) => self.push(Value::Int(value.0)),
            ConstantInfo::Float(value) => self.push(Value::Float(value.0)),
            //Todo! for string and class
            other => Err(JVMError::InvalidConstantType {
                expected: "integer, float, string, or class",
                found: Self::get_constant_type(&other),
            }),
        }
    }

    pub fn aconst_null(&mut self) -> Result<(), JVMError> {
        self.push(Value::Reference(None))
    }

    pub fn iconst(&mut self, value: i32) -> Result<(), JVMError> {
        self.push(Value::Int(value))
    }

    pub fn lconst(&mut self, value: i64) -> Result<(), JVMError> {
        self.push(Value::Long(value))
    }

    pub fn fconst(&mut self, value: f32) -> Result<(), JVMError> {
        self.push(Value::Float(value))
    }

    pub fn dconst(&mut self, value: f64) -> Result<(), JVMError> {
        self.push(Value::Double(value))
    }

    pub fn bipush(&mut self, byte: i8) -> Result<(), JVMError> {
        self.push(Value::Int(byte as i32))
    }

    pub fn sipush(&mut self, short: i16) -> Result<(), JVMError> {
        self.push(Value::Int(short as i32))
    }

    pub fn ldc(&mut self, index: u8) -> Result<(), JVMError> {
        self.load_constant(index as u16)
    }

    pub fn ldc_w(&mut self, index: u16) -> Result<(), JVMError> {
        self.load_constant(index)
    }

    pub fn ldc2_w(&mut self, index: u16) -> Result<(), JVMError> {
        let constant = self.get_constant(index)?;

        match constant {
            ConstantInfo::Long(value) => self.push(Value::Long(value.0)),
            ConstantInfo::Double(value) => self.push(Value::Double(value.0)),
            other => Err(JVMError::InvalidConstantType {
                expected: "long or double",
                found: Self::get_constant_type(&other),
            }),
        }
    }

    fn get_constant_type(constant: &ConstantInfo) -> &'static str {
        match constant {
            ConstantInfo::Integer(_) => "integer",
            ConstantInfo::Float(_) => "float",
            ConstantInfo::Long(_) => "long",
            ConstantInfo::Double(_) => "double",
            ConstantInfo::String(_) => "string",
            ConstantInfo::Class(_) => "class",
            ConstantInfo::Utf8(_) => "utf8",
            _ => "unknown",
        }
    }
}
