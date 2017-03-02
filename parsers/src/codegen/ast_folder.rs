use std::rc::Rc;

use ast::{Expr, PathExpr, Constant, OpCode, Sign};
use ast::pathexpr::into_linpath;
use ast::folder::ExprFolder;

use store::StoreType;
use store::folder::StoreTypeFolder;

use super::store_folder::CompileFolder;
use super::{IR, AssignIR, VarIR, ExprIR};


/// Convert the provided expression
/// into an IR.
pub fn into_ir(expression: &Expr, store: &StoreType) -> Result<IR, ()> {
    let mut folder = ExprIntoIRFolder::new(store);
    folder.fold(expression);
    Ok(folder.ir)
}


struct ExprIntoIRFolder<'a> {
    ir: IR,
    var_counter: u32,
    store_type: &'a StoreType,
}

impl<'a> ExprIntoIRFolder<'a> {

    fn new(store: &'a StoreType) -> ExprIntoIRFolder {
        ExprIntoIRFolder {
            ir: IR::new(),
            var_counter: 0,
            store_type: store,
        }
    }

    fn nid(&mut self) -> u32 {
        let res = self.var_counter;
        self.var_counter += 1;
        res
    }

    #[inline(always)]
    fn add_assign(&mut self, assign: AssignIR) -> Rc<AssignIR> {
        let assignment = Rc::new(assign);
        self.ir.instructions.push(assignment.clone());
        assignment
    }
}


impl<'a> ExprFolder for ExprIntoIRFolder<'a> {

    type FoldType = Rc<AssignIR>;

    fn fold_not(&mut self, _expr: &Expr) -> Self::FoldType {
        unimplemented!()
    }

    fn fold_signed(&mut self, _sign: Sign, _expr: &Expr) -> Self::FoldType {
        unimplemented!()
    }

    fn fold_constant(&mut self, value: &Constant) -> Self::FoldType {
        match *value {
            Constant::Integer(value) => {
                let id = self.nid();
                self.add_assign(AssignIR {
                    leftop: VarIR { id: id },
                    rightop: ExprIR::Constant(value)
                })
            }
            Constant::Boolean(_value) => {
                unimplemented!()
            }
            Constant::Float(_value) => {
                unimplemented!()
            }
            Constant::StrLit(_value) => {
                unimplemented!()
            }
        }
    }

    fn fold_binaryop(&mut self, left: &Expr, op: OpCode, right: &Expr)
        -> Self::FoldType
    {
        let id = self.nid();
        let left = self.fold(left);
        let right = self.fold(right);
        self.add_assign(AssignIR {
            leftop: VarIR { id: id },
            rightop: ExprIR::BinaryOp(left, op, right),
        })
    }

    fn fold_path_expression(&mut self, pe: &PathExpr)
        -> Self::FoldType
    {
        let id = self.nid();
        let linpath = into_linpath(pe).unwrap();
        let mut path_ir_folder = CompileFolder;
        let path_ir = path_ir_folder.fold(&self.store_type, &linpath, 0).unwrap();
        self.add_assign(AssignIR {
            leftop: VarIR { id: id },
            rightop: ExprIR::Path(path_ir),
        })
    }
}