use std::collections::HashSet;
use ast::Package;


pub type Error = ();

pub fn pass_resolve_names(ast: &mut Package) -> Result<(), Error> {
    // let mut imported_or_defined = HashSet::new();

    Ok(())
}

#[cfg(test)]
mod test;
