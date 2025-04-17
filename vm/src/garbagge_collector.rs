use super::heap::Heap;
use super::heap::Slot;
use super::jvm_error::JVMError;
use super::object::Object;
use super::object::ObjectKind;
use super::runtime::*;
use super::vm::VM;
use std::sync::Arc;

impl Heap {
    pub async fn run_minor_gc(&mut self, stack: &Stack,vm: &VM) -> Result<(), JVMError> {
        println!("Minorgc");
        self.mark_from_roots(&stack);

        let mut new_free_head = None;
        let mut last_free = None;
        for i in 0..self.objects.len() {
            let should_free = {
                if let Slot::Occupied(obj) = &self.objects[i] {
                    let header = obj.header.borrow();
                    header.generation == 0 && !header.mark
                } else {
                    false
                }
            };
            if should_free {
                self.objects[i] = Slot::Free {
                    next: None,
                    prev: last_free,
                };
                if let Some(prev) = last_free {
                    if let Slot::Free { next, .. } = &mut self.objects[prev] {
                        *next = Some(i);
                    }
                }
                last_free = Some(i);
                if new_free_head.is_none() {
                    new_free_head = Some(i);
                }
                self.young_count -= 1;
            } else if let Slot::Occupied(obj) = &self.objects[i] {
                let mut header = obj.header.borrow_mut();
                if header.generation == 0 && header.mark {
                    header.generation = 1;
                    self.young_count -= 1;
                    self.old_count += 1;
                }
            }
        }
        self.free_head = new_free_head;
        Ok(())
    }

    pub async fn run_major_gc(&mut self, stack: &Stack,vm: &VM) -> Result<(), JVMError> {
        self.mark_from_roots(&stack);

        let mut new_free_head: Option<usize> = None;
        let mut last_free: Option<usize> = None;
        let mut new_free_head = None;
        let mut last_free = None;
        for i in 0..self.objects.len() {
            let should_free = {
                if let Slot::Occupied(obj) = &self.objects[i] {
                    !obj.header.borrow().mark
                } else {
                    false
                }
            };
            if should_free {
                self.objects[i] = Slot::Free {
                    next: None,
                    prev: last_free,
                };
                if let Some(prev) = last_free {
                    if let Slot::Free { next, .. } = &mut self.objects[prev] {
                        *next = Some(i);
                    }
                }
                last_free = Some(i);
                if new_free_head.is_none() {
                    new_free_head = Some(i);
                }
                if let Slot::Occupied(obj) = &self.objects[i] {
                    if obj.header.borrow().generation == 0 {
                        self.young_count -= 1;
                    } else {
                        self.old_count -= 1;
                    }
                }
            }
        }
        self.free_head = new_free_head;
        Ok(())
    }

    fn mark_from_roots(&self, stack: &Stack) {
        for slot in &self.objects {
            if let Slot::Occupied(obj) = slot {
                obj.header.borrow_mut().mark = false;
            }
        }

        for frame in &stack.frames {
            for value in &frame.operands {
                if let Value::Reference(Some(obj)) = value {
                    Self::mark_object(obj);
                }
            }
            for value in &frame.locals {
                if let Value::Reference(Some(obj)) = value {
                    Self::mark_object(obj);
                }
            }
        }
    }

    fn mark_object(obj: &Arc<Object>) {
        let mut header = obj.header.borrow_mut();
        if !header.mark {
            header.mark = true;
            drop(header);

            match &obj.kind {
                ObjectKind::ClassInstance { fields } => {
                    for value in &*fields.borrow() {
                        if let Value::Reference(Some(ref_obj)) = value {
                            Self::mark_object(&ref_obj);
                        }
                    }
                }
                ObjectKind::ArrayInstance { elements, .. } => {
                    for value in &*elements.borrow() {
                        if let Value::Reference(Some(ref_obj)) = value {
                            Self::mark_object(&ref_obj);
                        }
                    }
                }
            }
        }
    }
}
