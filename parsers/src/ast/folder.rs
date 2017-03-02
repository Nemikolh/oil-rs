use super::{Expr, PathExpr, OpCode};
use super::{Sign, Constant};


pub trait ExprFolder {

    type FoldType;

    fn fold_constant(&mut self, value: &Constant)
        -> Self::FoldType;

    fn fold_binaryop(&mut self, left: &Expr, op: OpCode, right: &Expr)
        -> Self::FoldType;

    fn fold_path_expression(&mut self, pe: &PathExpr)
        -> Self::FoldType;

    fn fold_not(&mut self, expr: &Expr)
        -> Self::FoldType;

    fn fold_signed(&mut self, sign: Sign, expr: &Expr)
        -> Self::FoldType;

    fn fold(&mut self, expr: &Expr)
        -> Self::FoldType
    {
        match *expr {
            Expr::Constant(ref value) => self.fold_constant(value),
            Expr::PathExpr(ref pe) => self.fold_path_expression(pe),
            Expr::BinaryOp(ref left, op, ref right) =>
                self.fold_binaryop(left, op, right),
            Expr::Not(ref e) => self.fold_not(e),
            Expr::Signed(s, ref e) => self.fold_signed(s, e),
        }
    }
}