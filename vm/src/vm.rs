use super::class_manager::{ClassManager, Resolved};
use parser::attribute::Code;
use parser::consant_pool::ConstantPool;
use parser::consant_pool::MethodInfo;
use parser::types::*;

pub struct VM<'a> {
    class_manager: ClassManager<'a>,
    stack: Stack<'a>,
    // pc is undefined in case of native method, but no need to worry about that now
    //pc:
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self {
            class_manager: ClassManager::default(),
            stack: Stack::default(),
        }
    }
    pub fn start(&mut self, class: &str, args: Vec<Value>) {
        // idk what Resolved::ClassToInitialize means
        let temp_class = self.class_manager.get_or_resolve_class(class).unwrap();
        let class = temp_class.get_class();

        //let class = if let Resolved::Loaded(class) = temp_class {
        //class
        //} else {
        //todo!();
        //};
        //

        let main_method = class
            .methods
            .iter()
            .find(|method| method.is_main(&class.constants))
            .expect("couldnot find main method");

        let frame = StackFrame::new(main_method, &class.constants, args);
        self.stack.execute_frame(frame);
    }
}

#[derive(Default)]
pub struct Stack<'a> {
    frames: Vec<StackFrame<'a>>,
}
impl<'a> Stack<'a> {
    pub fn push_frame(&mut self, frame: StackFrame<'a>) {
        self.frames.push(frame);
    }
    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }

    fn execute_frame(&mut self, frame: StackFrame<'a>) {
        self.push_frame(frame);
        self.frames.iter_mut().last().unwrap().execute();
    }
}
pub struct StackFrame<'a> {
    locals: Vec<Value>,
    operand: Vec<Value>,
    cp: &'a ConstantPool,
    code: &'a Code,
    pc: U4,
}

impl<'a> StackFrame<'a> {
    fn new(method: &'a MethodInfo, cp: &'a ConstantPool, args: Vec<Value>) -> Self {
        let code = method
            .get_code_attribute()
            .expect("Code attribute not found in method");
        let mut locals = args;
        locals.reserve(code.max_locals as usize - locals.len());
        Self {
            locals: Vec::with_capacity(code.max_locals as usize),
            operand: Vec::with_capacity(code.max_stack as usize),
            pc: 0,
            cp,
            code,
        }
    }

    fn execute(&mut self) {
        println!("executing frame");
        loop {
            let operation = self.code.get_operation_at(self.pc);
            println!("{operation:?}");
            match operation {
                parser::instruction::Operation::D2f => (),
                _ => (),
            }
            break;
        }
    }
}

pub enum Value {
    Uninitialized,
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}
