use oil_repr::bytecode::{Bytecode, OpCode};
use oil_repr::bytecode::{Value};



#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VM {
    stack: Vec<Value>,
}

impl VM {

    pub fn execute(&mut self, bytecode: &Bytecode, context: ()) -> StackValue {
        loop {
        }
    }
}
