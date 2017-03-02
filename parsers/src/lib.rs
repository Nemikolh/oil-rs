
#[macro_use]
extern crate log;
extern crate phf;
extern crate num;
extern crate unicode_xid;
extern crate lalrpop_util;
extern crate lalrpop_intern;
extern crate oil_repr;

use std::io;

#[cfg_attr(test, macro_use)]
#[cfg(test)]
mod util_test;

pub mod grammar;
pub mod codegen;
pub mod ast;

mod tok;
mod store;
mod pass_resolve;


use std::path::Path;
use pass_resolve::pass_resolve_packages_from_root;

pub fn compile<P: AsRef<Path>>(input: P) -> Result<oil_repr::OilLibrary, CompileError> {
    let _ = try!(pass_resolve_packages_from_root(input));
    // let mut ast = try!(grammar::parse_grammar(&s));
    // try!(pass_resolve::pass_resolve_names(&mut ast));
    Ok(())
}


pub enum CompileError {
    IoError(io::Error),
    ParseError,
    NameResolution(pass_resolve::Error),
}

impl From<io::Error> for CompileError {
    fn from(err: io::Error) -> CompileError {
        CompileError::IoError(err)
    }
}

impl From<pass_resolve::Error> for CompileError {
    fn from(err: pass_resolve::Error) -> CompileError {
        CompileError::NameResolution(err)
    }
}

impl<'input> From<grammar::Error<'input>> for CompileError {
    fn from(_: grammar::Error<'input>) -> CompileError {
        CompileError::ParseError
    }
}
