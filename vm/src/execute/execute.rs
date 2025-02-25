use crate::jvm_error::JVMError;
use crate::runtime::*;
use crate::vm::VM;
use parser::instruction::Operation;

#[derive(Debug)]
pub enum ExecutionResult {
    Continue,
    Invoke(Frame),
    Return(Option<Value>),
}

impl Stack {
    pub async fn execute_current_frame(&mut self, vm: &VM) -> Result<(), JVMError> {
        if self.frames.is_empty() {
            return Err(JVMError::NoFrame);
        }
        let frame_index = self.frames.len() - 1;
        while self.frames[frame_index].pc < self.frames[frame_index].code.code.len() {
            let operation = self.frames[frame_index]
                .code
                .get_operation_at_index(self.frames[frame_index].pc);
            match self.frames[frame_index]
                .execute_instruction(&operation, vm)
                .await?
            {
                ExecutionResult::Continue => {
                    self.frames[frame_index].pc += 1;
                }
                ExecutionResult::Invoke(new_frame) => {
                    self.push_frame(new_frame)?;
                    let fut = Box::pin(self.execute_current_frame(vm));
                    fut.await?;
                    self.frames[frame_index].pc += 1;
                }
                ExecutionResult::Return(return_value) => {
                    if frame_index == 0 {
                        println!("{:?}", self.frames[frame_index].locals);
                        return Ok(());
                    }
                    println!("{:?}", self.frames[frame_index].locals);
                    self.pop_frame()?;
                    if let Some(value) = return_value {
                        self.frames[frame_index - 1].push(value)?;
                    }
                    return Ok(());
                }
            }
        }
        println!("{:?}", self.frames[frame_index].locals);
        Ok(())
    }
}
impl Frame {
    pub async fn execute_instruction(
        &mut self,
        operation: &Operation,
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
            Operation::Ldc(index) => self.ldc(*index)?,
            Operation::Ldcw(index1, index2) => {
                self.ldc_w(((*index1 as u16) << 8) | *index2 as u16)?
            }
            Operation::Ldc2w(index1, index2) => {
                self.ldc2_w(((*index1 as u16) << 8) | *index2 as u16)?
            }

            // Integer Arithmetic
            Operation::Iadd => self.iadd()?,
            Operation::Isub => self.isub()?,
            Operation::Imul => self.imul()?,
            Operation::Idiv => self.idiv()?,
            Operation::Irem => self.irem()?,
            Operation::Ineg => self.ineg()?,
            Operation::Iinc(index, value) => self.iinc(*index, *value as i8)?,

            // Long Arithmetic
            Operation::Ladd => self.ladd()?,
            Operation::Lsub => self.lsub()?,
            Operation::Lmul => self.lmul()?,
            Operation::Ldiv => self.ldiv()?,
            Operation::Lrem => self.lrem()?,
            Operation::Lneg => self.lneg()?,

            // Float Arithmetic
            Operation::Fadd => self.fadd()?,
            Operation::Fsub => self.fsub()?,
            Operation::Fmul => self.fmul()?,
            Operation::Fdiv => self.fdiv()?,
            Operation::Frem => self.frem()?,
            Operation::Fneg => self.fneg()?,

            // Double Arithmetic
            Operation::Dadd => self.dadd()?,
            Operation::Dsub => self.dsub()?,
            Operation::Dmul => self.dmul()?,
            Operation::Ddiv => self.ddiv()?,
            Operation::Drem => self.drem()?,
            Operation::Dneg => self.dneg()?,

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

            //Invoke statements
            Operation::Invokestatic(index1, index2) => self.invokestatic(((*index1 as u16) << 8) | *index2 as u16, vm).await?,

            _ => {
                println!("Instruction not implemented: {:?}", operation);
                ExecutionResult::Continue
            }
        };
        Ok(return_op_type)
    }
}
