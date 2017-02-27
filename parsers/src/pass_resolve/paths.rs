use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::rc::Rc;
use std::io::Read;

use ast::Package;
use grammar::parse_grammar;
use super::Error;

pub struct OilProgram(HashMap<PathBuf, Rc<Package>>);

impl OilProgram {

    fn new() -> OilProgram {
        OilProgram(HashMap::new())
    }

    fn resolve_from_root_ast(&mut self, root_path: PathBuf, root_package: Package) {
        self.0.insert(root_path, Rc::new(root_package));

        // TODO: Traverse imports.
        // TODO: Panic on global import (path that do not start with a dot)
        // TODO: What should be done with font / images files??
    }
}

pub fn pass_resolve_packages_from_root<P: AsRef<Path>>(root_file: P) -> Result<OilProgram, Error> {
    let mut program = OilProgram::new();

    let (root_package, root_path) = try!(open_oil_file(root_file));

    program.resolve_from_root_ast(root_path, root_package);

    Ok(program)
}

fn open_oil_file<P: AsRef<Path>>(oil_file: P) -> Result<(Package, PathBuf), Error> {
    let oil_file = oil_file.as_ref();
    let (mut f, file_path) = if oil_file.ends_with(".oil") {
        (File::open(oil_file)?, oil_file.into())
    } else {
        let mut path = PathBuf::from(oil_file);
        path.push(".oil");
        (File::open(path.clone())?, path)
    };
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok((parse_grammar(&s)?, file_path))
}
