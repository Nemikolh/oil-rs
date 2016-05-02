
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


use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Read;


pub fn compile<P: AsRef<Path>>(input: P) -> Result<oil_repr::OilLibrary, CompileError> {
    let mut f = try!(File::open(input.as_ref()));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    let ast = grammar::parse_grammar(&s);
    // Run passes
    // ...
    Ok(())
}


pub enum CompileError {
    IoError(io::Error),
}

impl From<io::Error> for CompileError {
    fn from(err: io::Error) -> CompileError {
        CompileError::IoError(err)
    }
}
