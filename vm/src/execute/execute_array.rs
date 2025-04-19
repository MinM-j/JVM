use super::execute::ExecutionResult;
use crate::jvm_error::JVMError;
use crate::object::{Object,ObjectKind};
use crate::runtime::*;
use crate::vm::VM;
use parser::constant_pool::{ConstantClassInfo, ConstantInfo};
use std::sync::Arc;

impl Frame {
    pub async fn newarray(&mut self, atype: u8, stack:&Stack,vm: &VM) -> Result<ExecutionResult, JVMError> {
        let length = self.pop_expect_int()?;
        let element_type = match atype {
            4 => "Z",
            5 => "C",
            6 => "F",
            7 => "D",
            8 => "B",
            9 => "S",
            10 => "I",
            11 => "J",
            _ => return Err(JVMError::Other(format!("Invalid array type: {}", atype))),
        };
        let fut = Box::pin(vm.allocate_array(stack, element_type, length as usize));
        let array_ref = fut.await?;
        self.push(array_ref)?;
        Ok(ExecutionResult::Continue)
    }

    pub async fn anewarray(&mut self, index: u16, stack:&Stack,vm: &VM) -> Result<ExecutionResult, JVMError> {
        let length = self.pop_expect_int()? as usize;
        let cp_entry = self.constant_pool.get_entry(index).ok_or_else(|| {
            JVMError::ConstantPoolIndexOutOfBounds {
                index,
                max: self.constant_pool.get_len(),
            }
        })?;
        let element_type = match cp_entry {
            ConstantInfo::Class(ConstantClassInfo(name_index)) => self
                .constant_pool
                .get_underlying_string_from_utf8_index(*name_index)
                .ok_or_else(|| JVMError::Other(format!("Invalid name_index {}", name_index)))?,
            _ => {
                return Err(JVMError::InvalidConstantType {
                    expected: "Class",
                    found: "other",
                })
            }
        };
        let fut = Box::pin(vm.allocate_array(stack, &element_type, length));
        let array_ref = fut.await?;
        self.push(array_ref)?;
        Ok(ExecutionResult::Continue)
    }

    pub async fn array_load(&mut self, data_type: String, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let index = self.pop_expect_int()? as usize;
        let array_ref = self.pop_expect_reference()?;
        match array_ref {
            Some(array) => {
                let is_valid = match &array.kind {
                    ObjectKind::ArrayInstance { element_type, .. } => {
                        if data_type == "L" {
                            element_type.starts_with("L") || element_type.starts_with("[")
                        } else {
                            *element_type == data_type
                        }
                    }
                    _ => false,
                };
                if !is_valid {
                    let found = match &array.kind {
                        ObjectKind::ArrayInstance { element_type, .. } => element_type.clone(),
                        _ => "non-array".to_string(),
                    };
                    return Err(JVMError::TypeMismatch {
                        expected: if data_type == "L" {
                            "L... or [...".to_string()
                        } else {
                            data_type
                        },
                        found,
                    });
                }
                let value = match array.get_element(index) {
                    Ok(sth) => sth,
                    _ => {
                        //let fut = Box::pin(vm.allocate_object("/java/lang/ArrayIndexOutOfBoundsException"));
                        //let exception = fut.await?;
                        return Ok(ExecutionResult::Throw("java/lang/ArrayIndexOutOfBoundsException".to_string()));
                    }
                };
                self.push(value.clone())?;
                Ok(ExecutionResult::Continue)
            }
            None => Err(JVMError::NullReference),
        }
    }

    pub async fn array_store(&mut self, _data_type: String, vm: &VM) -> Result<ExecutionResult, JVMError> {
        let value = match _data_type.as_str() {
            "I" => self.pop_expect_int().map(Value::Int)?,
            "J" => self.pop_expect_long().map(Value::Long)?,
            "F" => self.pop_expect_float().map(Value::Float)?,
            "D" => self.pop_expect_double().map(Value::Double)?,
            "L" => self.pop_expect_reference().map(Value::Reference)?,
            "B" => self.pop_expect_int().map(Value::Int)?,
            "C" => self.pop_expect_int().map(Value::Int)?,
            "S" => self.pop_expect_int().map(Value::Int)?,
            _ => {
                return Err(JVMError::Other(format!(
                    "Unsupported array data type: {}",
                    _data_type
                )))
            }
        };
        let index = self.pop_expect_int()? as usize;
        let array_ref = self.pop_expect_reference()?;

        match array_ref {
            Some(array) => {
                let is_valid = match &array.kind {
                    ObjectKind::ArrayInstance { element_type, .. } => {
                        if _data_type == "L" {
                            element_type.starts_with("L") || element_type.starts_with("[")
                        } else {
                            *element_type == _data_type
                        }
                    }
                    _ => false,
                };
                if !is_valid {
                    let found = match &array.kind {
                        ObjectKind::ArrayInstance { element_type, .. } => element_type.clone(),
                        _ => "non-array".to_string(),
                    };
                    return Err(JVMError::TypeMismatch {
                        expected: if _data_type == "L" {
                            "L... or [...".to_string()
                        } else {
                            _data_type.clone()
                        },
                        found,
                    });
                }

                match (_data_type.as_str(), &value) {
                    ("B", Value::Int(v)) if *v < i8::MIN as i32 || *v > i8::MAX as i32 => {
                        return Err(JVMError::TypeMismatch {
                            expected: "B (byte)".to_string(),
                            found: "out of range".to_string(),
                        });
                    }
                    ("C", Value::Int(v)) if *v < 0 || *v > u16::MAX as i32 => {
                        return Err(JVMError::TypeMismatch {
                            expected: "C (char)".to_string(),
                            found: "out of range".to_string(),
                        });
                    }
                    ("S", Value::Int(v)) if *v < i16::MIN as i32 || *v > i16::MAX as i32 => {
                        return Err(JVMError::TypeMismatch {
                            expected: "S (short)".to_string(),
                            found: "out of range".to_string(),
                        });
                    }
                    ("L", Value::Reference(Some(ref_obj))) => {
                        if let ObjectKind::ArrayInstance { element_type, .. } = &array.kind {
                            let expected_type = element_type
                                .strip_prefix("L")
                                .and_then(|s| s.strip_suffix(";"))
                                .unwrap_or(element_type);
                            if let Some(ref_class) = ref_obj.class.as_ref() {
                                if !self.is_compatible_class(ref_class, expected_type) {
                                    return Err(JVMError::TypeMismatch {
                                        expected: expected_type.to_string(),
                                        found: ref_class.class_name.clone(),
                                    });
                                }
                            }
                        }
                    }
                    _ => {}
                }
                /*
                array.set_element(index, value).map_err(|_| {
                        let fut = Box::pin(vm.allocate_object("/java/lang/ArrayIndexOutOfBoundsException"));
                    let exception = tokio::runtime::Handle::current().block_on(fut)?;
                        return Ok(ExecutionResult::Throw(exception));
                });
*/
                let _ = match array.set_element(index, value) {
                    Ok(()) => Ok::<ExecutionResult, JVMError>(ExecutionResult::Continue),
                    Err(_) => {
                        //let fut = Box::pin(vm.allocate_object("/java/lang/ArrayIndexOutOfBoundsException"));
                        //let exception = fut.await?;
                        return Ok(ExecutionResult::Throw("java/lang/ArrayIndexOutOfBoundsException".to_string()));
                    }
                };
                Ok(ExecutionResult::Continue)
            }
            None => Err(JVMError::NullReference),
        }
    }

    pub async fn arraylength(&mut self) -> Result<ExecutionResult, JVMError> {
        let array_ref = self.pop_expect_reference()?;
        match array_ref {
            Some(array) => {
                if let ObjectKind::ArrayInstance { length, .. } = &array.kind {
                    self.push(Value::Int(*length as i32))?;
                    Ok(ExecutionResult::Continue)
                } else {
                    return Err(JVMError::Other(
                        "arraylength on non-array object".to_string(),
                    ));
                }
            }
            None => Err(JVMError::NullReference),
        }
    }

    pub async fn multi_anew_array(&mut self, index: u16, dimensions: u8, stack: &Stack, vm: &VM) -> Result<ExecutionResult, JVMError> {
        if dimensions == 0 || dimensions > 255 {
            return Err(JVMError::Other(format!("Invalid dimensions for MultiANewArray: {}", dimensions)));
        }

        let mut sizes = Vec::new();
        for _ in 0..dimensions {
            let size = self.pop_expect_int()?;
            if size < 0 {
                return Err(JVMError::Other(format!("Negative array size: {}", size)));
            }
            sizes.push(size as usize);
        }
        sizes.reverse(); 

        let array_type = self.constant_pool
            .get_underlying_string_from_constant_class_info_index(index)
            .ok_or_else(|| JVMError::Other(format!("Invalid class index: {}", index)))?;
        let snap = array_type.clone();

        if !snap.starts_with('[') {
            return Err(JVMError::Other(format!("Invalid array type: {}", array_type)));
        }

        let array_ref = self.create_multi_array(stack, vm, &snap, &sizes, dimensions as usize).await?;
        self.push(Value::Reference(Some(array_ref)))?;
        Ok(ExecutionResult::Continue)
    }

    /*
    async fn create_multi_array(
        &mut self,
        stack: &Stack,
        vm: &VM,
        array_type: &str,
        sizes: &[usize],
        dims_to_init: usize,
    ) -> Result<Arc<Object>, JVMError> {
        if dims_to_init == 0 || sizes.is_empty() {
            return Err(JVMError::Other("Invalid dimensions or sizes for array".to_string()));
        }

        let size = sizes[0];
        let element_type = &array_type[1..]; 
        println!("{dims_to_init}");
        println!("{element_type}");

        if dims_to_init == 1 {
            let array_obj = Object::new_array(None, size, array_type);
            let array_ref = Arc::new(array_obj);
            let mut heap = vm.heap.write().await;
            match heap.free_head {
                Some(index) => {
                    heap.young_count += 1;
                    heap.take_slot(index, Arc::clone(&array_ref));
                    Ok(array_ref)
                }
                None => {
                    heap.run_minor_gc(stack, vm).await?;
                    match heap.free_head {
                        Some(index) => {
                            heap.young_count += 1;
                            heap.take_slot(index, Arc::clone(&array_ref));
                            Ok(array_ref)
                        }
                        None => Err(JVMError::Other("Heap exhausted after minor GC".to_string())),
                    }
                }
            }
        } else {
            let array_obj = Object::new_array(None, size, array_type);
            let array_ref = Arc::new(array_obj);
            let mut heap = vm.heap.write().await;
            match heap.free_head {
                Some(index) => {
                    heap.young_count += 1;
                    heap.take_slot(index, Arc::clone(&array_ref));
                }
                None => {
                    heap.run_minor_gc(stack, vm).await?;
                    match heap.free_head {
                        Some(index) => {
                            heap.young_count += 1;
                            heap.take_slot(index, Arc::clone(&array_ref));
                        }
                        None => return Err(JVMError::Other("Heap exhausted after minor GC".to_string())),
                    }
                }
            }

            let sub_sizes = &sizes[1..];
            for i in 0..size {
                let fut = Box::pin(self.create_multi_array(stack, vm, element_type, sub_sizes, dims_to_init - 1));
                let sub_array = fut.await?;
                array_ref.set_element(i, Value::Reference(Some(sub_array)))?;
            }
            Ok(array_ref)
        }
    }
    */
    async fn create_multi_array(
        &mut self,
        stack: &Stack,
        vm: &VM,
        array_type: &str,
        sizes: &[usize],
        dims_to_init: usize,
    ) -> Result<Arc<Object>, JVMError> {
        if dims_to_init == 0 || sizes.is_empty() {
            return Err(JVMError::Other("Invalid dimensions or sizes for array".to_string()));
        }

        let size = sizes[0];
        let element_type = &array_type[1..];

        let fut = Box::pin(vm.allocate_array(stack, element_type, size as usize));
        let array_ref = fut.await?;

        if dims_to_init > 1 {
            let sub_sizes = &sizes[1..];
            if let Value::Reference(Some(array_obj)) = &array_ref {
                for i in 0..size {
                    let fut = Box::pin(self.create_multi_array(stack, vm, element_type, sub_sizes, dims_to_init - 1));
                    let sub_array = fut.await?;
                    array_obj.set_element(i, Value::Reference(Some(sub_array)))?;
                }
            } else {
                return Err(JVMError::Other("Invalid array reference".to_string()));
            }
        }

        match array_ref {
            Value::Reference(Some(obj)) => Ok(obj),
            _ => Err(JVMError::Other("Expected reference value".to_string())),
        }
    }
}
