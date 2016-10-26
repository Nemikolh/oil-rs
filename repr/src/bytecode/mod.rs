
// Re-export Value type.
pub use self::value::Value;
pub use self::value::NULL_VAL;
pub use self::iter::InstrIter;
pub use self::check::Error;

mod value;
mod iter;
mod check;

#[cfg(test)]
mod tests;

const SIGN_BIT: u64 = 1 << 63;
const QNAN: u64 = 0x7ffc000000000000;

const TAG_NAN: u64 = 0;
const TAG_NULL: u64 = 1;
const TAG_FALSE: u64 = 2;
const TAG_TRUE: u64 = 3;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Bytecode {
    text: Vec<String>,
    // Little endian encoding
    code: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpCode {
    // Constant stack push (f64)
    Const,
    // Var access push (u64)
    VarAccess,
    // Unary operations
    Not,
    Sign, // Change sign of the stack value
    // Binary operations on numbers (f32)
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    // Binary operations on booleans
    Or,
    And,
    EqEq,
    NotEq,
    LessThan,
    GreaterThan,
    LessThanOrEq,
    GreaterThanOrEq,
    // Control flow operations
    SkipnIfNZ,  // Skip n (u64) instructions if non zero
    Ret,        // Return current stack value
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Instruction {
    pub op: OpCode,
    pub val: Value,
}
