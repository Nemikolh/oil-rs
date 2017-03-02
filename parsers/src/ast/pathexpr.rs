use lalrpop_intern::InternedString;
use super::{PathExpr, Constant};

// Encode a path such as:
// - a.b.c
// - a?b?c
// - a.b?c?
// - a?b.c
// - a.b[0]?
// - a[0].c
// ...
pub type LinPath = Vec<PathItem>;

pub fn into_linpath(mut expr: &PathExpr) -> Option<LinPath> {

    let mut res = vec![];
    let mut path_item = reset_path_item();

    fn reset_path_item() -> PathItem {
        PathItem {
            prop: PropKind::Int(0),
            is_optional: false,
        }
    }

    loop {
        match *expr {
            PathExpr::Variable(ref name) => {
                path_item.prop = PropKind::Str(name.name.clone());
                res.push(path_item);
                break;
            }
            PathExpr::Option { ref opt, .. } => {
                path_item.is_optional = true;
                expr = opt;
            }
            PathExpr::Index { ref map, ref index, .. } => {
                if let Constant::Integer(index) = *index {
                    path_item.prop = PropKind::Int(index as usize);
                    res.push(path_item);
                    path_item = reset_path_item();
                    expr = map;
                }
            }
            PathExpr::PropertyAccess { ref map, ref property, .. } => {
                path_item.prop = PropKind::Str(property.name.clone());
                res.push(path_item);
                path_item = reset_path_item();
                expr = map;
            }
        }
    }

    res.reverse();

    Some(res)
}

#[derive(Debug, PartialEq, Eq)]
pub struct PathItem {
    // Discriminate between the two possible property
    // access:
    //  - an index for an Array-like such as `[0]`
    //  - a string for a Product-like such as `b`
    pub prop: PropKind,
    // This is not used for traversing the `StoreType` tree
    // but is needed for validating the correctness of the
    // path.
    pub is_optional: bool,
}

impl PathItem {

    pub fn new(prop: PropKind, is_optional: bool) -> PathItem {
        PathItem {
            prop: prop,
            is_optional: is_optional,
        }
    }

    pub fn prop_str(&self) -> String {
        if let PropKind::Str(ref st) = self.prop {
            st.to_string()
        } else {
            panic!("This PathItem is not a String.");
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PropKind {
    Int(usize),
    Str(InternedString),
}