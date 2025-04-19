use crate::jvm_error::JVMError;
use crate::runtime::*;
use crate::state::{Header, MessageData, GLOBAL_BOOL, SERVER_STATE};
use crate::vm::{convert_instructions, VM};
use parser::instruction::Operation;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug)]
pub enum ExecutionResult {
    Continue,
    Invoke(Frame),
    Return(Option<Value>),
    Throw(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SerValue {
    Uninitialized,
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Reference(String),
}

pub fn serialize_vec(values: Vec<Value>) -> Vec<SerValue> {
    values
        .into_iter()
        .map(|v| match v {
            Value::Default => SerValue::Uninitialized,
            Value::Int(i) => SerValue::Int(i),
            Value::Long(l) => SerValue::Long(l),
            Value::Float(f) => SerValue::Float(f),
            Value::Double(d) => SerValue::Double(d),
            Value::Reference(ref opt_obj) => match opt_obj {
                Some(obj) => {
                    let object_id = obj.header.borrow().object_id;
                    SerValue::Reference(format!("Object Id: {}", object_id))
                }
                None => SerValue::Reference("None".to_string()),
            },
        })
        .collect()
}

impl Stack {
    pub async fn execute_current_frame(&mut self, vm: &VM) -> Result<(), JVMError> {
        if self.frames.is_empty() {
            return Err(JVMError::NoFrame);
        }
        let mut frame_index = self.frames.len() - 1;
        //                println!("{:?}", self.frames[frame_index].method_name_des);
        //println!("{:?}", self.frames[frame_index].locals);
        while self.frames[frame_index].pc < self.frames[frame_index].code.code.len() {
            let ser_locals = serialize_vec(self.frames[frame_index].locals.clone());
            let ser_operands = serialize_vec(self.frames[frame_index].operands.clone());
            {
                let flag = GLOBAL_BOOL.lock().unwrap();
                if *flag {
                    let json_frame = MessageData {
                        header: Header::DATA,
                        json: json!({"header": "frame", "name": self.frames[frame_index].method_name_des.name, "pc": self.frames[frame_index].pc, "locals": ser_locals, "operands": ser_operands}).to_string(),
                    };
                    {
                        let mut queue = SERVER_STATE.lock().unwrap();
                        queue.push_back(json_frame);
                    }
                }
            }
            let operation = self.frames[frame_index]
                .code
                .get_operation_at_index(self.frames[frame_index].pc);
            {
                let flag = GLOBAL_BOOL.lock().unwrap();
                if *flag {
                    let json_cei = MessageData {
                        header: Header::DATA,
                        json: json!({"header": "cei", "value": format!("{:?}", operation)})
                            .to_string(),
                    };
                    {
                        let mut queue = SERVER_STATE.lock().unwrap();
                        queue.push_back(json_cei);
                    }
                }
            }
            let stack_snapshot = self.clone();
            match self.frames[frame_index]
                .execute_instruction(&operation, &stack_snapshot, vm)
                .await?
            {
                ExecutionResult::Continue => {
                    self.frames[frame_index].pc += 1;
                }
                ExecutionResult::Invoke(new_frame) => {
                    {
                        let flag = GLOBAL_BOOL.lock().unwrap();
                        if *flag {
                            let code = convert_instructions(new_frame.code.code.clone());
                            let stack_json = MessageData {
                        header: Header::DATA,
                        json: json!({"header": "stack", "name": new_frame.method_name_des.name, "action": "push", "locals": new_frame.locals.len(), "operands": new_frame.operands.len(), "code": code}).to_string(),
                    };
                            {
                                let mut queue = SERVER_STATE.lock().unwrap();
                                queue.push_back(stack_json);
                            }
                        }
                    }
                    self.push_frame(new_frame)?;
                    let fut = Box::pin(self.execute_current_frame(vm));
                    fut.await?;
                    self.frames[frame_index].pc += 1;
                }
                ExecutionResult::Return(return_value) => {
                    if frame_index == 0 {
                        {
                            let flag = GLOBAL_BOOL.lock().unwrap();
                            if *flag {
                                let stack_json = MessageData {
                                    header: Header::DATA,
                                    json: json!({"header": "stack", "action": "pop"}).to_string(),
                                };
                                {
                                    let mut queue = SERVER_STATE.lock().unwrap();
                                    queue.push_back(stack_json);
                                }
                                let eof = MessageData {
                                    header: Header::EOF,
                                    json: String::new(),
                                };
                                {
                                    let mut queue = SERVER_STATE.lock().unwrap();
                                    queue.push_back(eof);
                                }
                            }
                        }
                        return Ok(());
                    }
                    {
                        let flag = GLOBAL_BOOL.lock().unwrap();
                        if *flag {
                            let stack_json = MessageData {
                                header: Header::DATA,
                                json: json!({"header": "stack", "action": "pop"}).to_string(),
                            };
                            {
                                let mut queue = SERVER_STATE.lock().unwrap();
                                queue.push_back(stack_json);
                            }
                        }
                    }
                    //println!("{:?}", self.frames[frame_index].locals);
                    self.pop_frame()?;
                    if let Some(value) = return_value {
                        self.frames[frame_index - 1].push(value)?;
                    }
                    return Ok(());
                }
                ExecutionResult::Throw(exception) => {
                    if let Some(handler_pc) = self.frames[frame_index]
                        .find_exception_handler(&exception, vm)
                        .await
                    {
                        self.frames[frame_index].operands.clear();
                        self.frames[frame_index].push(Value::Reference(None))?;
                        self.frames[frame_index].pc = self.frames[frame_index]
                            .code
                            .get_index_at_address(handler_pc as u32);
                    } else if frame_index > 0 {
                        self.pop_frame()?;
                        frame_index -= 1;
                        if let Some(handler_pc) = self.frames[frame_index]
                            .find_exception_handler(&exception, vm)
                            .await
                        {
                            self.frames[frame_index].operands.clear();
                            self.frames[frame_index].push(Value::Reference(None))?;
                            self.frames[frame_index].pc = self.frames[frame_index]
                                .code
                                .get_index_at_address(handler_pc as u32);
                        } else {
                            if frame_index == 0 {
                                return Err(JVMError::UncaughtException(exception));
                            }
                        }
                    } else {
                        return Err(JVMError::UncaughtException(exception));
                    }
                    /*
                    let exception_obj = match &exception {
                        Value::Reference(Some(obj)) => obj,
                        _ => unreachable!("athrow ensures this is a valid reference"),
                    };
                    if let Some(handler_pc) = self.frames[frame_index]
                        .find_exception_handler(exception_obj, vm)
                        .await
                    {
                        self.frames[frame_index].operands.clear();
                        self.frames[frame_index].push(exception.clone())?;
                        self.frames[frame_index].pc = self.frames[frame_index]
                            .code
                            .get_index_at_address(handler_pc as u32);
                    } else if frame_index > 0 {
                        self.pop_frame()?;
                        self.frames[frame_index - 1].push(exception)?;
                        frame_index -= 1;
                    } else {
                        let exception_class = match exception {
                            Value::Reference(Some(obj)) => obj
                                .class
                                .as_ref()
                                .map(|class| class.class_name.clone())
                                .unwrap_or_default(),
                            _ => "Unknown".to_string(),
                        };
                        return Err(JVMError::UncaughtException(exception_class));
                    }
                    */
                }
            }
        }
        Ok(())
    }
}
impl Frame {
    pub async fn execute_instruction(
        &mut self,
        operation: &Operation,
        stack: &Stack,
        vm: &VM,
    ) -> Result<ExecutionResult, JVMError> {
        let return_op_type = match operation {
            // Load Instructions
            Operation::Iload(index) => self.iload(*index)?,
            Operation::Lload(index) => self.lload(*index)?,
            Operation::Fload(index) => self.fload(*index)?,
            Operation::Dload(index) => self.dload(*index)?,
            Operation::Aload(index) => self.aload(*index)?,
            Operation::Iload0 => self.iload(0)?,
            Operation::Iload1 => self.iload(1)?,
            Operation::Iload2 => self.iload(2)?,
            Operation::Iload3 => self.iload(3)?,
            Operation::Lload0 => self.lload(0)?,
            Operation::Lload1 => self.lload(1)?,
            Operation::Lload2 => self.lload(2)?,
            Operation::Lload3 => self.lload(3)?,
            Operation::Fload0 => self.fload(0)?,
            Operation::Fload1 => self.fload(1)?,
            Operation::Fload2 => self.fload(2)?,
            Operation::Fload3 => self.fload(3)?,
            Operation::Dload0 => self.dload(0)?,
            Operation::Dload1 => self.dload(1)?,
            Operation::Dload2 => self.dload(2)?,
            Operation::Dload3 => self.dload(3)?,
            Operation::Aload0 => self.aload(0)?,
            Operation::Aload1 => self.aload(1)?,
            Operation::Aload2 => self.aload(2)?,
            Operation::Aload3 => self.aload(3)?,

            // Store Instructions
            Operation::Istore(index) => self.istore(*index)?,
            Operation::Lstore(index) => self.lstore(*index)?,
            Operation::Fstore(index) => self.fstore(*index)?,
            Operation::Dstore(index) => self.dstore(*index)?,
            Operation::Astore(index) => self.astore(*index)?,
            Operation::Istore0 => self.istore(0)?,
            Operation::Istore1 => self.istore(1)?,
            Operation::Istore2 => self.istore(2)?,
            Operation::Istore3 => self.istore(3)?,
            Operation::Lstore0 => self.lstore(0)?,
            Operation::Lstore1 => self.lstore(1)?,
            Operation::Lstore2 => self.lstore(2)?,
            Operation::Lstore3 => self.lstore(3)?,
            Operation::Fstore0 => self.fstore(0)?,
            Operation::Fstore1 => self.fstore(1)?,
            Operation::Fstore2 => self.fstore(2)?,
            Operation::Fstore3 => self.fstore(3)?,
            Operation::Dstore0 => self.dstore(0)?,
            Operation::Dstore1 => self.dstore(1)?,
            Operation::Dstore2 => self.dstore(2)?,
            Operation::Dstore3 => self.dstore(3)?,
            Operation::Astore0 => self.astore(0)?,
            Operation::Astore1 => self.astore(1)?,
            Operation::Astore2 => self.astore(2)?,
            Operation::Astore3 => self.astore(3)?,

            // Constant Instructions
            Operation::Aconstnull => self.aconst_null()?,
            Operation::Iconstm1 => self.iconst(-1)?,
            Operation::Iconst0 => self.iconst(0)?,
            Operation::Iconst1 => self.iconst(1)?,
            Operation::Iconst2 => self.iconst(2)?,
            Operation::Iconst3 => self.iconst(3)?,
            Operation::Iconst4 => self.iconst(4)?,
            Operation::Iconst5 => self.iconst(5)?,
            Operation::Lconst0 => self.lconst(0)?,
            Operation::Lconst1 => self.lconst(1)?,
            Operation::Fconst0 => self.fconst(0.0)?,
            Operation::Fconst1 => self.fconst(1.0)?,
            Operation::Fconst2 => self.fconst(2.0)?,
            Operation::Dconst0 => self.dconst(0.0)?,
            Operation::Dconst1 => self.dconst(1.0)?,
            Operation::Bipush(byte) => self.bipush(*byte as i8)?,
            Operation::Sipush(index1, index2) => {
                self.sipush(((*index1 as i16) << 8) | *index2 as i16)?
            }
            Operation::Ldc(index) => self.ldc(*index, stack, vm).await?,
            Operation::Ldcw(index1, index2) => {
                self.ldc_w(((*index1 as u16) << 8) | *index2 as u16, stack, vm)
                    .await?
            }
            Operation::Ldc2w(index1, index2) => {
                self.ldc2_w(stack, ((*index1 as u16) << 8) | *index2 as u16)?
            }

            //Shifting and bit-wise operations
            Operation::Ishl => self.ishl()?,
            Operation::Ishr => self.ishr()?,
            Operation::Iushr => self.iushr()?,
            Operation::Lshl => self.lshl()?,
            Operation::Lshr => self.lshr()?,
            Operation::Lushr => self.lushr()?,
            Operation::Ior => self.ior()?,
            Operation::Lor => self.lor()?,
            Operation::Ixor => self.ixor()?,
            Operation::Lxor => self.lxor()?,
            Operation::Iand => self.iand()?,
            Operation::Land => self.land()?,

            // Integer Arithmetic
            Operation::Iadd => self.iadd()?,
            Operation::Isub => self.isub()?,
            Operation::Imul => self.imul()?,
            Operation::Idiv => self.idiv(vm).await?,
            Operation::Irem => self.irem()?,
            Operation::Ineg => self.ineg()?,
            Operation::Iinc(index, value) => self.iinc(*index, *value as i8)?,

            // Long Arithmetic
            Operation::Ladd => self.ladd()?,
            Operation::Lsub => self.lsub()?,
            Operation::Lmul => self.lmul()?,
            Operation::Ldiv => self.ldiv(vm).await?,
            Operation::Lrem => self.lrem()?,
            Operation::Lneg => self.lneg()?,

            // Float Arithmetic
            Operation::Fadd => self.fadd()?,
            Operation::Fsub => self.fsub()?,
            Operation::Fmul => self.fmul()?,
            Operation::Fdiv => self.fdiv(vm).await?,
            Operation::Frem => self.frem()?,
            Operation::Fneg => self.fneg()?,
            Operation::Fcmpg => self.fcmpg()?,
            Operation::Fcmpl => self.fcmpl()?,

            // Double Arithmetic
            Operation::Dadd => self.dadd()?,
            Operation::Dsub => self.dsub()?,
            Operation::Dmul => self.dmul()?,
            Operation::Ddiv => self.ddiv(vm).await?,
            Operation::Drem => self.drem()?,
            Operation::Dneg => self.dneg()?,
            Operation::Dcmpg => self.dcmpg()?,
            Operation::Dcmpl => self.dcmpl()?,

            // Comparison branches
            Operation::Ifeq(index1, index2) => {
                self.ifeq(((*index1 as i16) << 8) | *index2 as i16)?
            }
            Operation::Ifne(index1, index2) => {
                self.ifne(((*index1 as i16) << 8) | *index2 as i16)?
            }
            Operation::Iflt(index1, index2) => {
                self.iflt(((*index1 as i16) << 8) | *index2 as i16)?
            }
            Operation::Ifge(index1, index2) => {
                self.ifge(((*index1 as i16) << 8) | *index2 as i16)?
            }
            Operation::Ifgt(index1, index2) => {
                self.ifgt(((*index1 as i16) << 8) | *index2 as i16)?
            }
            Operation::Ifle(index1, index2) => {
                self.ifle(((*index1 as i16) << 8) | *index2 as i16)?
            }

            // Reference comparison
            Operation::Ifnull(index1, index2) => {
                self.ifnull(((*index1 as i16) << 8) | *index2 as i16)?
            }
            Operation::Ifnonnull(index1, index2) => {
                self.ifnonnull(((*index1 as i16) << 8) | *index2 as i16)?
            }

            // Integer comparison
            Operation::Ificmpeq(index1, index2) => {
                self.if_icmpeq(((*index1 as i16) << 8) | *index2 as i16)?
            }
            Operation::Ificmpne(index1, index2) => {
                self.if_icmpne(((*index1 as i16) << 8) | *index2 as i16)?
            }
            Operation::Ificmplt(index1, index2) => {
                self.if_icmplt(((*index1 as i16) << 8) | *index2 as i16)?
            }
            Operation::Ificmpge(index1, index2) => {
                self.if_icmpge(((*index1 as i16) << 8) | *index2 as i16)?
            }
            Operation::Ificmpgt(index1, index2) => {
                self.if_icmpgt(((*index1 as i16) << 8) | *index2 as i16)?
            }
            Operation::Ificmple(index1, index2) => {
                self.if_icmple(((*index1 as i16) << 8) | *index2 as i16)?
            }

            // Reference comparison
            Operation::Ifacmpeq(index1, index2) => {
                self.if_acmpeq(((*index1 as i16) << 8) | *index2 as i16)?
            }
            Operation::Ifacmpne(index1, index2) => {
                self.if_acmpne(((*index1 as i16) << 8) | *index2 as i16)?
            }

            // Unconditional branches
            Operation::Goto(index1, index2) => {
                self.branch_16(((*index1 as i16) << 8) | *index2 as i16)?
            }
            Operation::Gotow(index1, index2, index3, index4) => self.branch_32(
                ((*index1 as i32) << 24)
                    | ((*index2 as i32) << 16)
                    | ((*index3 as i32) << 8)
                    | *index4 as i32,
            )?,

            // Switch statements
            Operation::Tableswitch(default_offset, low, high, jump_offsets) => {
                self.table_switch(*default_offset, *low, *high, jump_offsets)?
            }
            Operation::Lookupswitch(default_offset, _, pairs) => {
                self.lookup_switch(*default_offset, pairs)?
            }

            //Return statements
            Operation::Return => self.return_void()?,
            Operation::Ireturn => self.return_int()?,
            Operation::Lreturn => self.return_long()?,
            Operation::Freturn => self.return_float()?,
            Operation::Dreturn => self.return_double()?,
            Operation::Areturn => self.return_reference()?,

            //array instructions
            Operation::Newarray(atype) => self.newarray(*atype, stack, vm).await?,
            Operation::Anewarray(index1, index2) => {
                self.anewarray(((*index1 as u16) << 8) | *index2 as u16, stack, vm)
                    .await?
            }

            Operation::Multianewarray(index1, index2, dimension) => {
                self.multi_anew_array(
                    ((*index1 as u16) << 8) | *index2 as u16,
                    *dimension,
                    stack,
                    vm,
                )
                .await?
            }

            Operation::Iaload => self.array_load("I".to_string(), vm).await?,
            Operation::Laload => self.array_load("J".to_string(), vm).await?,
            Operation::Faload => self.array_load("F".to_string(), vm).await?,
            Operation::Daload => self.array_load("D".to_string(), vm).await?,
            Operation::Aaload => self.array_load("L".to_string(), vm).await?,
            Operation::Baload => self.array_load("B".to_string(), vm).await?,
            Operation::Caload => self.array_load("C".to_string(), vm).await?,
            Operation::Saload => self.array_load("S".to_string(), vm).await?,
            Operation::Iastore => self.array_store("I".to_string(), vm).await?,
            Operation::Lastore => self.array_store("J".to_string(), vm).await?,
            Operation::Fastore => self.array_store("F".to_string(), vm).await?,
            Operation::Dastore => self.array_store("D".to_string(), vm).await?,
            Operation::Aastore => self.array_store("L".to_string(), vm).await?,
            Operation::Bastore => self.array_store("B".to_string(), vm).await?,
            Operation::Castore => self.array_store("C".to_string(), vm).await?,
            Operation::Sastore => self.array_store("S".to_string(), vm).await?,
            Operation::Arraylength => self.arraylength().await?,

            //objects instructions
            Operation::New(index1, index2) => {
                self.execute_new(((*index1 as u16) << 8) | *index2 as u16, stack, vm)
                    .await?
            }
            Operation::Dup => self.dup()?,
            Operation::Dupx1 => self.dup_x1()?,
            Operation::Dupx2 => self.dup_x2()?,
            Operation::Dup2 => self.dup2()?,
            Operation::Dup2x1 => self.dup2_x1()?,
            Operation::Dup2x2 => self.dup2_x2()?,
            Operation::Putfield(index1, index2) => {
                self.putfield(((*index1 as u16) << 8) | *index2 as u16)
                    .await?
            }
            Operation::Getfield(index1, index2) => {
                self.getfield(((*index1 as u16) << 8) | *index2 as u16)
                    .await?
            }
            Operation::Putstatic(index1, index2) => {
                self.putstatic(((*index1 as u16) << 8) | *index2 as u16, vm)
                    .await?
            }
            Operation::Getstatic(index1, index2) => {
                self.getstatic(((*index1 as u16) << 8) | *index2 as u16, vm)
                    .await?
            }

            //Invoke statements
            Operation::Invokestatic(index1, index2) => {
                self.invokestatic(((*index1 as u16) << 8) | *index2 as u16, vm)
                    .await?
            }
            Operation::Invokespecial(index1, index2) => {
                self.invokespecial(((*index1 as u16) << 8) | *index2 as u16, vm)
                    .await?
            }
            Operation::Invokevirtual(index1, index2) => {
                self.invokevirtual(((*index1 as u16) << 8) | *index2 as u16, vm)
                    .await?
            }
            Operation::Invokeinterface(index1, index2, _, _) => {
                self.invokeinterface(((*index1 as u16) << 8) | *index2 as u16, vm)
                    .await?
            }

            //convert instructions
            Operation::I2l => self.convert("I".to_string(), "L".to_string())?,
            Operation::I2f => self.convert("I".to_string(), "F".to_string())?,
            Operation::I2d => self.convert("I".to_string(), "D".to_string())?,
            Operation::L2i => self.convert("L".to_string(), "I".to_string())?,
            Operation::L2f => self.convert("L".to_string(), "F".to_string())?,
            Operation::L2d => self.convert("L".to_string(), "D".to_string())?,
            Operation::F2i => self.convert("F".to_string(), "I".to_string())?,
            Operation::F2l => self.convert("F".to_string(), "L".to_string())?,
            Operation::F2d => self.convert("F".to_string(), "D".to_string())?,
            Operation::D2i => self.convert("D".to_string(), "I".to_string())?,
            Operation::D2l => self.convert("D".to_string(), "L".to_string())?,
            Operation::D2f => self.convert("D".to_string(), "F".to_string())?,
            Operation::I2b => self.convert("I".to_string(), "B".to_string())?,
            Operation::I2c => self.convert("I".to_string(), "C".to_string())?,
            Operation::I2s => self.convert("I".to_string(), "S".to_string())?,

            //Execption
            Operation::Athrow => self.athrow(vm).await?,

            //Pop Nop
            Operation::Nop => ExecutionResult::Continue,
            Operation::Pop => {
                self.pop()?;
                ExecutionResult::Continue
            }
            Operation::Pop2 => {
                match self.operands.last() {
                    Some(Value::Long(_)) | Some(Value::Double(_)) => {
                        self.pop()?;
                    }
                    _ => {
                        self.pop()?;
                        self.pop()?;
                    }
                }
                ExecutionResult::Continue
            }

            //check cast and instance of
            Operation::Checkcast(index1, index2) => {
                self.checkcast(((*index1 as u16) << 8) | *index2 as u16, vm)
                    .await?
            }
            Operation::Instanceof(index1, index2) => {
                self.checkcast(((*index1 as u16) << 8) | *index2 as u16, vm)
                    .await?
            }
            _ => {
                println!("Instruction not implemented: {:?}", operation);
                ExecutionResult::Continue
            }
        };
        //println!("{:?}", operation);
        Ok(return_op_type)
    }
}
