use std::rc::Rc;
use ast::OpCode;
use ast::Expr;
use store::StoreType;


use self::render::Render;


pub struct IR {
    /// List of assignment that the IR is made of.
    instructions: Vec<Rc<AssignIR>>,
}

pub struct AssignIR {
    /// The left operand.
    leftop: VarIR,
    /// The right operand. (an expression)
    rightop: ExprIR,
}

/// The type of a variable is always Option<_>.
/// This is neat because it means that we always handle
/// the possibility of no value quite easily.
pub struct VarIR {
    /// The generated id of that variable.
    /// A name will be derived from the id.
    id: u32,
}

pub enum ExprIR {
    /// A path access.
    Path(PathIR),
    /// A binary expression
    BinaryOp(Rc<AssignIR>, OpCode, Rc<AssignIR>),
    /// A constant
    Constant(i32),
}


pub enum PathIR {
    Match { path: String, children: Vec<PathIR> },
    IntoIter { path: String, then: Box<PathIR> },
    Variant { path: String, name: String, then: Box<PathIR> },
    Some { path: String },
    VariantNone,
}

/// Compile the provided expression using the store information
/// provided.
pub fn compile(expression: &Expr, store: &StoreType) -> Result<String, ()> {
    let ir = ast_folder::into_ir(expression, store)?;
    let mut res = String::new();
    // FIXME
    let _ = ir.render(&mut res);
    Ok(res)
}

mod ast_folder;
mod store_folder;
mod render;

#[cfg(test)]
mod tests;

impl IR {
    fn new() -> IR {
        IR { instructions: vec![] }
    }
}