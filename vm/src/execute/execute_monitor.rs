use super::execute::ExecutionResult;
use crate::jvm_error::JVMError;
use crate::runtime::*;
use crate::vm::VM;
use std::sync::Arc;

impl Frame {
    async fn monitor_enter(&mut self, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let ref_value = self.pop()?;
        match ref_value {
            Value::Reference(None) => Err(JVMError::NullReference),
            Value::Reference(Some(obj)) => {
                let monitor = &obj.monitor;
                let _guard = monitor
                    .lock()
                    .map_err(|_| JVMError::Other("Failed to acquire monitor".to_string()))?;
                Ok(ExecutionResult::Continue)
            }
            _ => Err(JVMError::Other(
                "MonitorEnter requires a reference type".to_string(),
            )),
        }
    }

    async fn monitor_exit(&mut self, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let ref_value = self.pop()?;
        match ref_value {
            Value::Reference(None) => Err(JVMError::NullReference),
            Value::Reference(Some(obj)) => {
                let monitor = &obj.monitor;
                if monitor.try_lock().is_ok() {
                    return Err(JVMError::IllegalMonitorStateException(
                        "Current thread does not own the monitor".to_string(),
                    ));
                }
                drop(
                    monitor
                        .lock()
                        .map_err(|_| JVMError::Other("Failed to release monitor".to_string()))?,
                );
                Ok(ExecutionResult::Continue)
            }
            _ => Err(JVMError::Other(
                "MonitorExit requires a reference type".to_string(),
            )),
        }
    }
}
