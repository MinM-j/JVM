use super::execute::ExecutionResult;
use crate::jvm_error::JVMError;
use crate::runtime::*;
impl Frame {
    pub fn convert(
        &mut self,
        source_type: String,
        target_type: String,
    ) -> Result<ExecutionResult, JVMError> {
        let value = match source_type.as_str() {
            "I" => self.pop_expect_int().map(Value::Int)?,
            "L" => self.pop_expect_long().map(Value::Long)?,
            "F" => self.pop_expect_float().map(Value::Float)?,
            "D" => self.pop_expect_double().map(Value::Double)?,
            _ => {
                return Err(JVMError::Other(format!(
                    "Unsupported source type: {}",
                    source_type
                )))
            }
        };

        let converted = match (source_type.as_str(), target_type.as_str()) {
            ("I", "L") => match value {
                Value::Int(v) => Value::Long(v as i64),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for I to L conversion"
                    )))
                }
            },
            ("I", "F") => match value {
                Value::Int(v) => Value::Float(v as f32),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for I to F conversion"
                    )))
                }
            },
            ("I", "D") => match value {
                Value::Int(v) => Value::Double(v as f64),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for I to D conversion"
                    )))
                }
            },
            ("L", "I") => match value {
                Value::Long(v) => Value::Int(v as i32),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for L to I conversion"
                    )))
                }
            },
            ("L", "F") => match value {
                Value::Long(v) => Value::Float(v as f32),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for L to F conversion"
                    )))
                }
            },
            ("L", "D") => match value {
                Value::Long(v) => Value::Double(v as f64),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for L to D conversion"
                    )))
                }
            },
            ("F", "I") => match value {
                Value::Float(v) => Value::Int(v as i32),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for F to I conversion"
                    )))
                }
            },
            ("F", "L") => match value {
                Value::Float(v) => Value::Long(v as i64),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for F to L conversion"
                    )))
                }
            },
            ("F", "D") => match value {
                Value::Float(v) => Value::Double(v as f64),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for F to D conversion"
                    )))
                }
            },
            ("D", "I") => match value {
                Value::Double(v) => Value::Int(v as i32),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for D to I conversion"
                    )))
                }
            },
            ("D", "L") => match value {
                Value::Double(v) => Value::Long(v as i64),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for D to L conversion"
                    )))
                }
            },
            ("D", "F") => match value {
                Value::Double(v) => Value::Float(v as f32),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for D to F conversion"
                    )))
                }
            },
            ("I", "B") => match value {
                Value::Int(v) => Value::Int((v as i8) as i32),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for I to B conversion"
                    )))
                }
            },
            ("I", "C") => match value {
                Value::Int(v) => Value::Int((v as u16) as i32),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for I to C conversion"
                    )))
                }
            },
            ("I", "S") => match value {
                Value::Int(v) => Value::Int((v as i16) as i32),
                _ => {
                    return Err(JVMError::Other(format!(
                        "Invalid source value for I to S conversion"
                    )))
                }
            },
            _ => {
                return Err(JVMError::Other(format!(
                    "Unsupported conversion from {} to {}",
                    source_type, target_type
                )))
            }
        };

        self.push(converted)?;
        Ok(ExecutionResult::Continue)
    }
}
