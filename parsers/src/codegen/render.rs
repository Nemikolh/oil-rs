use ast::OpCode;

use super::PathIR;
use super::ExprIR;
use super::AssignIR;
use super::IR;
use std::fmt::Error;
use std::fmt::Write;

pub trait Render {

    fn render(&self, output: &mut String) -> Result<(), Error>;
}


impl Render for PathIR {

    fn render(&self, output: &mut String) -> Result<(), Error> {
        match *self {
            PathIR::Match { ref path, ref children } => {
                writeln!(output, "match {} {{", path)?;
                for child in children {
                    child.render(output)?;
                }
                writeln!(output, "}}")?;
                Ok(())
            }
            PathIR::IntoIter { ref path, ref then } => {
                writeln!(output, "match IntoIterator::into_iter({}).next() {{", path)?;
                writeln!(output, "Some({}) => {{", path)?;
                then.render(output)?;
                writeln!(output, "}}")?;
                writeln!(output, "None => None,")?;
                writeln!(output, "}}")?;
                Ok(())
            }
            PathIR::Variant { ref path, ref name, ref then } => {
                writeln!(output, "{} {{ ref {} }} => {{", name, path)?;
                then.render(output)?;
                writeln!(output, "}}")?;
                Ok(())
            }
            PathIR::VariantNone => {
                writeln!(output, "_ => None,")?;
                Ok(())
            }
            PathIR::Some { ref path } => {
                writeln!(output, "Some({})", path)?;
                Ok(())
            }
        }
    }
}

impl Render for IR {

    fn render(&self, output: &mut String) -> Result<(), Error> {
        for instruction in &self.instructions {
            instruction.render(output)?;
        }
        Ok(())
    }
}

impl Render for ExprIR {

    fn render(&self, output: &mut String) -> Result<(), Error> {
        match *self {
            ExprIR::Path(ref p) => p.render(output),
            ExprIR::BinaryOp(ref left, op, ref right) => {

                if left.leftop.id == right.leftop.id {

                    writeln!(output, "if let Some(__tmp{}) = __tmp{} {{",
                        left.leftop.id, left.leftop.id)?;
                    write!(output, "Some(__tmp{}", left.leftop.id)?;
                    op.render(output)?;
                    write!(output, "__tmp{})", left.leftop.id)?;
                    writeln!(output, "}} else {{ None }}")?;
                    Ok(())
                } else {

                    writeln!(output, "if let Some(__tmp{}) = __tmp{} {{",
                        left.leftop.id, left.leftop.id)?;
                    writeln!(output, "if let Some(__tmp{}) = __tmp{} {{",
                        right.leftop.id, right.leftop.id)?;

                    // The actual meat of the binary operation.
                    write!(output, "Some(__tmp{}", left.leftop.id)?;
                    op.render(output)?;
                    write!(output, "__tmp{})", right.leftop.id)?;

                    writeln!(output, "}} else {{ None }}")?;
                    writeln!(output, "}} else {{ None }}")?;
                    Ok(())
                }
            }
            ExprIR::Constant(constant) => {
                writeln!(output, "{}", constant)?;
                Ok(())
            }
        }
    }
}


impl Render for AssignIR {

    fn render(&self, output: &mut String) -> Result<(), Error> {
        write!(output, "let __tmp{} = ", self.leftop.id)?;
        self.rightop.render(output)?;
        writeln!(output, ";")?;
        Ok(())
    }
}


impl Render for OpCode {

    fn render(&self, output: &mut String) -> Result<(), Error> {
        let _ = match *self {
            OpCode::Add => write!(output, "+"),
            OpCode::Sub => write!(output, "-"),
            OpCode::Or => write!(output, "||"),
            OpCode::Mul => write!(output, "*"),
            OpCode::Div => write!(output, "/"),
            // TODO: this is most likely wrong.
            OpCode::Pow => write!(output, "^"),
            OpCode::Mod => write!(output, "%"),
            OpCode::And => write!(output, "&&"),
            // TODO: The eq operator in oil should be more permissive regarding the type.
            OpCode::EqEq => write!(output, "=="),
            OpCode::NotEq => write!(output, "!="),
            OpCode::LessThan => write!(output, "<"),
            OpCode::GreaterThan => write!(output, ">"),
            OpCode::LessThanOrEq => write!(output, "<="),
            OpCode::GreaterThanOrEq => write!(output, ">="),
        };
        Ok(())
    }
}