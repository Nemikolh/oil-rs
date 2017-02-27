use std::collections::HashSet;
use lalrpop_intern::InternedString;

use ast::visit::PackageVisitor;
use ast::visit::walk;
use ast::{
    Package,
    Import,
    Class,
    ConstValue,
    SubPackage,
    ImportSymbols
};
use super::Error;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Symbols {
    defined: HashSet<InternedString>,
    imported: HashSet<InternedString>,
    exported: HashSet<InternedString>,
}

pub fn pass_resolve_symbols(ast: &mut Package) -> Result<Symbols, Error> {
    let mut symbols = Symbols::default();

    struct SymbolCollector<'a> {
        symbols: &'a mut Symbols,
    }
    impl<'a> PackageVisitor for SymbolCollector<'a> {
        fn visit_import(&mut self,  item: &Import) {
            let subpck_symbols = match item.package {
                SubPackage::Package(ref pck) => {
                    // Here we use the fact that if we have
                    // already visited the package then it is in a "borrow_mut" state
                    // so this would fail. :)
                    if let Ok(mut pck) = pck.try_borrow_mut() {
                        pass_resolve_symbols(&mut pck).unwrap()
                    } else {
                        panic!("TODO");
                    }
                }
                SubPackage::UnresolvedPath(..) =>
                    panic!("resolve_symbols must be ran after resolve_imports"),
                _ => Symbols::default(),
            };

            match &item.symbols {
                &ImportSymbols::All => self.symbols.imported.extend(subpck_symbols.exported),
                // TODO: check that values belong to defined_symbols.
                &ImportSymbols::Only(ref values) => self.symbols.imported.extend(values.iter().map(|v| v.name)),
                &ImportSymbols::AllAsIdent(_) => unimplemented!(),
            }
        }

        fn visit_class(&mut self, item: &Class) {
            self.symbols.defined.insert(item.name.name);
            if item.exported {
                self.symbols.exported.insert(item.name.name);
            }
        }

        fn visit_const(&mut self, item: &ConstValue) {
            self.symbols.defined.insert(item.name.name);
            if item.exported {
                self.symbols.exported.insert(item.name.name);
            }
        }
    }

    // Scope to reduce symbols mut lifetime.
    {
        walk(ast, &mut SymbolCollector { symbols: &mut symbols });
    }

    Ok(symbols)
}
