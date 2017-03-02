use lalrpop_intern::read;
use super::{StoreType, EnumVariant, VariantType};
use ast::pathexpr::{LinPath, PropKind};


pub trait VariantTypeFolder {

    type FoldType;

    fn fold_variant_field(&mut self, field_type: &StoreType, full_path: &LinPath, cur_index: usize)
        -> Option<Self::FoldType>;

    fn fold_variant_tuple(&mut self, field_type: &StoreType, full_path: &LinPath, cur_index: usize)
        -> Option<Self::FoldType>;

    fn fold_variant(&mut self, variant: &VariantType, path: &LinPath, cur_index: usize)
        -> Option<Self::FoldType>
    {
        match *variant {
            VariantType::Struct { ref fields } => {
                // Navigate further to the field.
                if let PropKind::Str(ref prop) = path[cur_index + 1].prop {
                    return read(|interner|
                        if let Some(field) = fields.get(interner.data(*prop)) {
                            self.fold_variant_field(field, path, cur_index + 1)
                        } else {
                            None
                        }
                    );
                }

                None
            }
            VariantType::Tuple { ref elements } => {
                // Navigate further to the field.
                if let PropKind::Int(index) = path[cur_index + 1].prop {
                    if let Some(field) = elements.get(index) {
                        return self.fold_variant_tuple(field, path, cur_index + 1);
                    }
                }

                None
            }
        }
    }
}

pub trait StoreTypeFolder {

    type FoldType;

    fn fold_leaf(&mut self, last_type: &StoreType, path: &LinPath)
        -> Option<Self::FoldType>;

    fn fold_option(&mut self, opt_wrapped_value: &StoreType, path: &LinPath, cur_index: usize)
        -> Option<Self::FoldType>;

    fn fold_field(&mut self, field_type: &StoreType, path: &LinPath, cur_index: usize)
        -> Option<Self::FoldType>;

    fn fold_variants(&mut self, variants: &Vec<EnumVariant>, path: &LinPath, cur_index: usize)
        -> Option<Self::FoldType>;

    // If None is returned it means the branch should be ignored.
    fn fold(&mut self, ct: &StoreType, path: &LinPath, cur_index: usize)
        -> Option<Self::FoldType>
    {
        match (ct, cur_index + 1 == path.len()) {

            // Option can be unwrapped even at the
            (&StoreType::Option(ref opt), is_last) => {
                if path[cur_index].is_optional {
                    self.fold_option(opt, path, cur_index)
                } else if is_last {
                    self.fold_leaf(ct, path)
                } else {
                    None
                }
            }

            // Fold field in struct only if its not a leaf.
            (&StoreType::Struct { ref fields }, false) => {
                // Navigate further to the field.
                if let PropKind::Str(ref prop) = path[cur_index + 1].prop {
                    return read(|interner|
                        if let Some(field) = fields.get(interner.data(*prop)) {
                            self.fold_field(field, path, cur_index + 1)
                        } else {
                            None
                        }
                    );
                }

                None
            }

            // Act exactly like a struct. The main difference:
            //  - All properties are integers.
            (&StoreType::Array { ref elements }, false) => {
                if let PropKind::Int(index) = path[cur_index + 1].prop {
                    if let Some(field) = elements.get(index) {
                        return self.fold_field(field, path, cur_index + 1);
                    }
                }

                None
            }

            // Fold variants in enum if there's a property afterwards.
            // Note: If optional is false. This branch must be ignored.
            (&StoreType::Enum { ref variants }, false) => {
                if path[cur_index + 1].is_optional {
                    self.fold_variants(variants, path, cur_index)
                } else {
                    None
                }
            }

            // An unbounded array. Any expression can be used. Only constrained:
            // They all have the same type.
            (&StoreType::UnboundedArray { .. }, false) => {
                if let PropKind::Int(_) = path[cur_index + 1].prop {

                    // This require the possibility to fold
                    // index values. This is not currently possible.
                    unimplemented!()
                } else {
                    None
                }
            }

            // If we haven't matched anything, and this is a leaf then
            // we fold the leaf.
            (_, true) => self.fold_leaf(ct, path),

            // All other possibilities mean that the path is invalid.
            _ => None,
        }
    }
}
