use super::class_loader::loaded_class::{LoadedClass, NameDes};
use parser::attribute::Code;
use parser::constant_pool::ConstantPool;
use std::{collections::HashMap, sync::Arc};
use super::jvm_error::JVMError;

#[derive(Clone, Debug)]
pub enum Value {
    Default,
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Reference(Option<Arc<Object>>),
}

#[derive(Clone, Debug)]
pub struct Object {
    pub class: Arc<LoadedClass>,
    pub fields: HashMap<String, Value>,
}

#[derive(Debug)]
pub struct Frame {
    pub constant_pool: Arc<ConstantPool>,
    pub method_name_des: NameDes,
    pub code: Arc<Code>,
    pub pc: usize,
    pub locals: Vec<Value>,
    pub operands: Vec<Value>,
}

impl Frame {
    pub fn new(class: Arc<LoadedClass>, name_des: &NameDes) -> Self {
        let temp_code = class.get_code_from_method(&name_des);
        Frame {
            constant_pool: Arc::clone(&class.constant_pool),
            method_name_des: name_des.clone(),
            code: Arc::clone(&temp_code),
            pc: 0,
            locals: vec![Value::Default; temp_code.max_locals.into()],
            operands: Vec::with_capacity(temp_code.max_stack.into()),
        }
    }

    pub fn push(&mut self, value: Value) -> Result<(), JVMError>{
        if self.operands.len() >= self.code.max_stack as usize {
            return Err(JVMError::StackOverflow);
        }
        self.operands.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Value, JVMError> {
        self.operands.pop().ok_or(JVMError::StackUnderflow)
    }

    pub fn set_local(&mut self, index: usize, value: Value) {
        if index < self.locals.len() {
            self.locals[index] = value;
        }
    }

    pub fn get_local(&mut self, index: usize) -> Option<&Value> {
        self.locals.get(index)
    }
}

#[derive(Debug)]
pub struct Stack {
    pub frames: Vec<Frame>,
    pub max_stack_size: usize,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            frames: Vec::new(),
            max_stack_size: 1024,
        }
    }

    pub fn push_frame(&mut self, frame: Frame) -> Result<(), JVMError> {
        if self.frames.len() >= self.max_stack_size {
            return Err(JVMError::StackOverflow);
        }
        self.frames.push(frame);
        Ok(())
    }

    pub fn pop_frame(&mut self) -> Result<Frame, JVMError> {
        self.frames.pop().ok_or(JVMError::StackUnderflow)
    }

    pub fn current_frame(&mut self) -> Result<&mut Frame, JVMError> {
        self.frames.last_mut().ok_or(JVMError::NoFrame)
    }
}

#[derive(Debug)]
pub struct Heap {
    pub objects: HashMap<usize, Object>,
}
