use super::jvm_error::JVMError;
use super::object::Object;
use super::runtime::Value;
use super::vm::VM;
use crate::class_loader::loaded_class::LoadedClass;
use std::sync::Arc;

#[derive(Debug)]
pub enum Slot {
    Occupied(Arc<Object>),
    Free {
        next: Option<usize>,
        prev: Option<usize>,
    },
}

#[derive(Debug)]
pub struct Heap {
    pub size: usize,
    pub objects: Vec<Slot>,
    pub free_head: Option<usize>,
    pub young_count: usize,
    pub old_count: usize,
}

impl Heap {
    pub fn new(size: usize) -> Self {
        let mut objects = Vec::with_capacity(size);
        for i in 0..size {
            objects.push(Slot::Free {
                next: if i + 1 < size { Some(i + 1) } else { None },
                prev: if i > 0 { Some(i - 1) } else { None },
            });
        }
        Heap {
            size,
            objects,
            free_head: Some(0),
            young_count: 0,
            old_count: 0,
        }
    }

    pub async fn allocate_object(&mut self, vm: &VM, class_name: &str) -> Result<Value, JVMError> {
        let class = vm.class_loader.load_class(class_name, vm).await.unwrap();
        //dbg!(&class);
        let obj = Object::new_class(Arc::clone(&class), vm).await;
        let obj_ref = Arc::new(obj);

        match self.free_head {
            Some(index) => {
                self.young_count += 1;
                self.take_slot(index, Arc::clone(&obj_ref));
                Ok(Value::Reference(Some(obj_ref)))
            }
            None => {
                self.run_minor_gc(vm).await?;
                match self.free_head {
                    Some(index) => {
                        self.young_count += 1;
                        self.take_slot(index, Arc::clone(&obj_ref));
                        Ok(Value::Reference(Some(obj_ref)))
                    }
                    None => {
                        self.run_major_gc(vm).await?;
                        match self.free_head {
                            Some(index) => {
                                self.young_count += 1;
                                self.take_slot(index, Arc::clone(&obj_ref));
                                Ok(Value::Reference(Some(obj_ref)))
                            }
                            None => Err(JVMError::Other("Heap exhausted after GC".to_string())),
                        }
                    }
                }
            }
        }
    }

    pub async fn allocate_array(
        &mut self,
        vm: &VM,
        element_type: &str,
        length: usize,
    ) -> Result<Value, JVMError> {
        let class = if element_type.starts_with("[") || element_type.len() == 1 {
            None
        } else {
            Some(vm.class_loader.load_class(element_type, vm).await.unwrap())
        };
        let obj = Object::new_array(class, length, element_type);
        let obj_ref = Arc::new(obj);

        match self.free_head {
            Some(index) => {
                self.young_count += 1;
                self.take_slot(index, Arc::clone(&obj_ref));
                Ok(Value::Reference(Some(obj_ref)))
            }
            None => {
                self.run_minor_gc(vm).await?;
                match self.free_head {
                    Some(index) => {
                        self.young_count += 1;
                        self.take_slot(index, Arc::clone(&obj_ref));
                        Ok(Value::Reference(Some(obj_ref)))
                    }
                    None => {
                        self.run_major_gc(vm).await?;
                        match self.free_head {
                            Some(index) => {
                                self.young_count += 1;
                                self.take_slot(index, Arc::clone(&obj_ref));
                                Ok(Value::Reference(Some(obj_ref)))
                            }
                            None => Err(JVMError::Other("Heap exhausted after GC".to_string())),
                        }
                    }
                }
            }
        }
    }

    pub async fn allocate_string(
        &mut self,
        vm: &VM,
        string_value: &str,
    ) -> Result<Value, JVMError> {
        let string_class = vm
            .class_loader
            .load_class("java/lang/String", vm)
            .await
            .unwrap();
        let chars: Vec<Value> = string_value.chars().map(|c| Value::Int(c as i32)).collect();
        let char_array = Object::new_array(None, chars.len(), "C");
        let char_array_ref = Arc::new(char_array);
        for (i, char_value) in chars.into_iter().enumerate() {
            char_array_ref.set_element(i, char_value)?;
        }
        let char_array_slot = match self.free_head {
            Some(index) => {
                self.young_count += 1;
                self.take_slot(index, Arc::clone(&char_array_ref));
                index
            }
            None => {
                self.run_minor_gc(vm).await?;
                match self.free_head {
                    Some(index) => {
                        self.young_count += 1;
                        self.take_slot(index, Arc::clone(&char_array_ref));
                        index
                    }
                    None => {
                        self.run_major_gc(vm).await?;
                        match self.free_head {
                            Some(index) => {
                                self.young_count += 1;
                                self.take_slot(index, Arc::clone(&char_array_ref));
                                index
                            }
                            None => {
                                return Err(JVMError::Other("Heap exhausted after GC".to_string()))
                            }
                        }
                    }
                }
            }
        };
        let string_obj = Object::new_class(string_class, vm).await;
        let string_ref = Arc::new(string_obj);
        string_ref.set_field("value", Value::Reference(Some(Arc::clone(&char_array_ref))))?;
        match self.free_head {
            Some(index) => {
                self.young_count += 1;
                self.take_slot(index, Arc::clone(&string_ref));
                Ok(Value::Reference(Some(string_ref)))
            }
            None => {
                self.run_minor_gc(vm).await?;
                match self.free_head {
                    Some(index) => {
                        self.young_count += 1;
                        self.take_slot(index, Arc::clone(&string_ref));
                        Ok(Value::Reference(Some(string_ref)))
                    }
                    None => {
                        self.run_major_gc(vm).await?;
                        match self.free_head {
                            Some(index) => {
                                self.young_count += 1;
                                self.take_slot(index, Arc::clone(&string_ref));
                                Ok(Value::Reference(Some(string_ref)))
                            }
                            None => Err(JVMError::Other("Heap exhausted after GC".to_string())),
                        }
                    }
                }
            }
        }
    }

    pub async fn allocate_class(
        &mut self,
        vm: &VM,
        loaded_class: Arc<LoadedClass>,
    ) -> Result<Value, JVMError> {
        let class_class = vm
            .class_loader
            .load_class("java/lang/Class", vm)
            .await
            .unwrap();

        let name_value = self.allocate_string(vm, &loaded_class.class_name).await?;

        let class_obj = Object::new_class(class_class, vm).await;
        let class_ref = Arc::new(class_obj);

        class_ref.set_field("name", name_value)?;

        match self.free_head {
            Some(index) => {
                self.young_count += 1;
                self.take_slot(index, Arc::clone(&class_ref));
                Ok(Value::Reference(Some(class_ref)))
            }
            None => {
                self.run_minor_gc(vm).await?;
                match self.free_head {
                    Some(index) => {
                        self.young_count += 1;
                        self.take_slot(index, Arc::clone(&class_ref));
                        Ok(Value::Reference(Some(class_ref)))
                    }
                    None => {
                        self.run_major_gc(vm).await?;
                        match self.free_head {
                            Some(index) => {
                                self.young_count += 1;
                                self.take_slot(index, Arc::clone(&class_ref));
                                Ok(Value::Reference(Some(class_ref)))
                            }
                            None => Err(JVMError::Other("Heap exhausted after GC".to_string())),
                        }
                    }
                }
            }
        }
    }

    pub fn take_slot(&mut self, index: usize, obj: Arc<Object>) {
        if let Slot::Free { next, prev } = self.objects[index] {
            if let Some(p) = prev {
                if let Slot::Free {
                    next: ref mut n, ..
                } = &mut self.objects[p]
                {
                    *n = next;
                }
            } else {
                self.free_head = next;
            }
            if let Some(n) = next {
                if let Slot::Free {
                    prev: ref mut p, ..
                } = &mut self.objects[n]
                {
                    *p = prev;
                }
            }
            self.objects[index] = Slot::Occupied(obj);
        } else {
            unreachable!("Slot should be free");
        }
    }
}
