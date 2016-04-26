
extern crate phf;
extern crate num;
extern crate unicode_xid;
extern crate lalrpop_util;
extern crate oil_repr;


pub mod grammar;
pub mod ast;

mod tok;
mod ser;


use std::path::Path;

pub fn compile<P: AsRef<Path>>(input: P) -> Result<oil_repr::OilLibrary, ()> {
    let mut f = try!(File::open(input.as_ref()));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    let ast = grammar::parse_grammar(input);
    // Run passes
    // ...
    ()
}
