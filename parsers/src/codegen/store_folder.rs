use ast::pathexpr::PropKind;
use ast::pathexpr::LinPath;
use store::StoreType;
use store::EnumVariant;
use store::folder::StoreTypeFolder;
use store::folder::VariantTypeFolder;

use super::PathIR;
use super::DotPath;


pub struct CompileFolder;


impl VariantTypeFolder for CompileFolder {

    type FoldType = PathIR;

    fn fold_variant_field(&mut self, field_type: &StoreType, full_path: &LinPath, cur_index: usize)
        -> Option<Self::FoldType>
    {
        self.fold(field_type, full_path, cur_index)
    }

    fn fold_variant_tuple(&mut self, field_type: &StoreType, full_path: &LinPath, cur_index: usize)
        -> Option<Self::FoldType>
    {
        self.fold(field_type, full_path, cur_index)
    }
}


impl StoreTypeFolder for CompileFolder {

    type FoldType = PathIR;

    fn fold_leaf(&mut self, last_type: &StoreType, path: &LinPath)
        -> Option<Self::FoldType>
    {
        match *last_type {
            StoreType::String | StoreType::Number => {
                if let PropKind::Str(ref st) = path.last().unwrap().prop {
                    Some(PathIR::Some { path: st.to_string() })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn fold_field(&mut self, field_type: &StoreType, full_path: &LinPath, cur_index: usize)
        -> Option<Self::FoldType>
    {
        fn concat(mut prop: String, path: DotPath) -> DotPath {
            prop += ".";
            prop += &path.path;
            DotPath::composite(prop)
        }

        fn concat_str(mut prop: String, path: String) -> String {
            prop += ".";
            prop += &path;
            prop
        }

        if let Some(folded) = self.fold(field_type, full_path, cur_index) {
            match folded {
                PathIR::Match { path, children } => Some(PathIR::Match {
                    path: concat(full_path[cur_index - 1].prop_str(), path),
                    children: children,
                }),
                PathIR::IntoIter { path, then } => Some(PathIR::IntoIter {
                    path: concat(full_path[cur_index - 1].prop_str(), path),
                    then: then,
                }),
                PathIR::Some { path } => Some(PathIR::Some {
                    path: concat_str(full_path[cur_index - 1].prop_str(), path),
                }),
                _ => unreachable!(),
            }
        } else {
            None
        }
    }

    fn fold_variants(&mut self, variants: &Vec<EnumVariant>, path: &LinPath, cur_index: usize)
        -> Option<Self::FoldType>
    {
        let mut children = Vec::new();

        for variant in variants {
            if let Some(folded) = self.fold_variant(&variant.variant_type, path, cur_index) {
                children.push(PathIR::Variant {
                    name: variant.name.clone(),
                    path: path[cur_index + 1].prop_str(),
                    then: Box::new(folded),
                });
            }
        }

        if children.len() > 0 {
            if children.len() < variants.len() {
                children.push(PathIR::VariantNone);
            }
            Some(PathIR::Match {
                path: DotPath::simple(path[cur_index].prop_str()),
                children: children,
            })
        } else {
            None
        }
    }

    fn fold_option(&mut self, opt_wrapped_value: &StoreType, path: &LinPath, cur_index: usize)
        -> Option<Self::FoldType>
    {
        if let Some(folded) = self.fold(opt_wrapped_value, path, cur_index) {
            Some(PathIR::IntoIter {
                path: DotPath::simple(path[cur_index].prop_str()),
                then: Box::new(folded),
            })
        } else {
            None
        }
    }
}

impl DotPath {
    fn composite(path: String) -> DotPath {
        DotPath {
            is_composite: true,
            path: path,
        }
    }

    fn simple(path: String) -> DotPath {
        DotPath {
            is_composite: false,
            path: path,
        }
    }
}