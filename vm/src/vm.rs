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
    // pc stores vector index not byte address so it is easier to get next opcode
    // but linear search is performed in case of jump
    // in which case we need to find vector index from byte adress
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

        //print all operation
        //this is howy you get opcode at certain address
        // note: 29 is valid address only for current main method
        //       let t = self.code.get_operation_at_address(29);
        //        println!(" 29 {t:?}");

        loop {
            let operation = self.code.get_operation_at_index(self.pc as usize);
            println!("{operation:?}");
            //execution
            match operation {
                _ => (),
            }
            self.pc += 1; //increase pc (vector index)
            if self.pc as usize >= self.code.code.len() {
                break;
            }
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
