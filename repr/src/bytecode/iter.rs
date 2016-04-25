use std::mem;

use bytecode::{Bytecode, Instruction};
use bytecode::{Value, NULL_VAL};
use bytecode::OpCode;

pub struct InstrIter<'a> {
    bytecode: &'a Bytecode,
    cursor: usize,
}

impl Bytecode {

    pub fn instr_iter(&self) -> InstrIter {
        InstrIter {
            bytecode: self,
            cursor: 0
        }
    }
}

impl<'a> Iterator for InstrIter<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Instruction> {

        #[inline]
        fn instr(op: OpCode, val: Value) -> Instruction {
            Instruction {
                op: op,
                val: val,
            }
        }

        if let Some(op) = self.bytecode.code.get(self.cursor).cloned() {
            let op = u8::from_le(op);
            self.cursor += 1;
            let cursor = self.cursor;
            // XXX: Is there a way to obtain the number of variant?
            if op >= 18u8 {
                return None;
            }
            let inst = match unsafe { mem::transmute::<u8, OpCode>(op) } {
                // Constant stack push (f32)
                // Var access push (u32)
                op @ OpCode::Const |
                op @ OpCode::VarAccess => {
                    let ref v = self.bytecode.code;
                    let arr: [u8; 8] = [
                        v[cursor+0], v[cursor+1], v[cursor+2], v[cursor+3],
                        v[cursor+4], v[cursor+5], v[cursor+6], v[cursor+7]
                    ];
                    self.cursor += 8;
                    let number = unsafe { mem::transmute::<[u8;8], u64>(arr) };
                    let number = u64::from_le(number);
                    if op == OpCode::Const {
                        let number = unsafe { mem::transmute::<u64, f64>(number) };
                        instr(op, Value::from(number))
                    } else {
                        instr(op, Value::from(number))
                    }
                }
                op => instr(op, NULL_VAL)
            };
            Some(inst)
        } else {
            None
        }
    }
}
