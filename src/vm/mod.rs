use oil_repr::bytecode::{Bytecode, OpCode, Instruction};
use oil_repr::bytecode::{Value, NULL_VAL};

pub type Error = ();

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VM {
    stack: Vec<Value>,
}

impl VM {

    pub fn new() -> VM {
        VM { stack: Vec::new() }
    }

    pub fn execute(&mut self, bytecode: &Bytecode, context: ()) -> Result<Value, Error> {
        let mut instr_iter = bytecode.instr_iter();
        let mut skip = 0u64;
        loop {
            if let Some(Instruction { op, val }) = instr_iter.next() {
                // Skip instruction?
                if skip > 0 {
                    skip -= 1;
                    continue;
                }
                match op {
                    // Return instruction.
                    OpCode::Ret => return self.return_to_callee(),
                    // Skip instruction
                    OpCode::SkipnIfNZ => {
                        skip = val.raw_bits();
                        continue
                    }
                    // Var access stack push
                    OpCode::VarAccess => {
                        unimplemented!();
                    }
                    // Const stack push
                    OpCode::Const => self.stack.push(val),
                    // Binary operations on numbers a @op b
                    op @ OpCode::Add |
                    op @ OpCode::Sub |
                    op @ OpCode::Mul |
                    op @ OpCode::Div |
                    op @ OpCode::Pow |
                    op @ OpCode::Mod => {
                        let b = self.stack.pop().unwrap_or(Value::from(0.0));
                        let a = self.stack.pop().unwrap_or(Value::from(0.0));
                        // Perform operation
                        let res = VM::bin_op(try!(a.as_number()), try!(b.as_number()), op);
                        self.stack.push(Value::from(res));
                    }
                    op @ OpCode::EqEq |
                    op @ OpCode::NotEq |
                    op @ OpCode::LessThan |
                    op @ OpCode::GreaterThan |
                    op @ OpCode::LessThanOrEq |
                    op @ OpCode::GreaterThanOrEq => {
                        let b = try!(self.stack.pop().ok_or(()));
                        let a = try!(self.stack.pop().ok_or(()));
                        // Perform operation
                        let res = VM::cmp_op(try!(a.as_number()), try!(b.as_number()), op);
                        self.stack.push(Value::from(res));
                    }
                    op @ OpCode::Or |
                    op @ OpCode::And => {
                        let b = self.stack.pop().unwrap_or(Value::from(false));
                        let a = self.stack.pop().unwrap_or(Value::from(false));
                        if op == OpCode::And {
                            self.stack.push(Value::from(a.as_bool() && b.as_bool()));
                        } else {
                            self.stack.push(Value::from(a.as_bool() || b.as_bool()));
                        }
                    }
                    OpCode::Not => {
                        if let Some(val) = self.stack.pop() {
                            self.stack.push(Value::from(!val.as_bool()));
                        }
                    }
                    OpCode::Sign => {
                        if let Some(val) = self.stack.pop() {
                            let val = try!(val.as_number());
                            self.stack.push(Value::from(-val));
                        }
                    }
                }
            } else {
                return self.return_to_callee();
            }
        }
    }

    fn return_to_callee(&mut self) -> Result<Value, Error> {
        if self.stack.len() > 1 {
            Err(())
        } else {
            Ok(self.stack.pop().unwrap_or(NULL_VAL))
        }
    }

    #[inline]
    fn bin_op(a: f64, b: f64, op: OpCode) -> f64 {
        match op {
            OpCode::Add => a + b,
            OpCode::Sub => a - b,
            OpCode::Mul => a * b,
            OpCode::Div => a / b,
            OpCode::Pow => a.powf(b),
            OpCode::Mod => a % b,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn cmp_op(a: f64, b: f64, op: OpCode) -> bool {
        match op {
            OpCode::EqEq => a == b,
            OpCode::NotEq => a != b,
            OpCode::LessThan => a < b,
            OpCode::GreaterThan => a > b,
            OpCode::LessThanOrEq => a <= b,
            OpCode::GreaterThanOrEq => a >= b,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test;
