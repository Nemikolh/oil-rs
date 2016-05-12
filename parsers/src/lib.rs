
#[macro_use]
extern crate log;
extern crate phf;
extern crate num;
extern crate unicode_xid;
extern crate lalrpop_util;
extern crate oil_repr;

#[cfg_attr(test, macro_use)]
#[cfg(test)]
mod util_test;

pub mod grammar;
pub mod ast;

mod tok;
mod ser;
mod pass_resolve;


use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Read;


pub fn compile<P: AsRef<Path>>(input: P) -> Result<oil_repr::OilLibrary, CompileError> {
    let mut f = try!(File::open(input.as_ref()));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    let mut ast = try!(grammar::parse_grammar(&s));
    try!(pass_resolve::pass_resolve_names(&mut ast));
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
