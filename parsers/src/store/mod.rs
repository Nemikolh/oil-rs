use std::collections::HashMap;
use std::vec::Vec;

pub mod folder;

/// Type of a store. As seen by the oil compiler.
/// This type is built from a concrete Rust type.
/// Because it is used to perform type checking
/// on the oil code and code generation, different
/// Rust type can map to the same thing.
///
/// For now all path in the store are assumed to be
/// reachable (no privacy issues). A couple of types
/// are known to the oil compiler:
///  - `Vec<_>`
///  - `HashMap<String, _>`
///  - `Option<_>` | `Result<_, *>`
///  - Any `struct` will become a Product
///
/// Later, once a StoreType instance will by constructed by
/// parsing the Rust source, we will consider using some traits
/// instead to deduce the possible access pattern of a particular
/// type. An idea might be to have a look at `IntoIterator` and friends.
///
/// For now, we will ignore this issue and assume that some compile
/// step is providing us an instance of `StoreType`.
#[derive(Debug, Clone, PartialEq)]
pub enum StoreType {
    /// String is mostly about `&str` and anything that can dereference to it.
    /// So it includes `String` and `Cow<&str>` and anything really that someone could write.
    String,
    /// Numbers. Includes [iuf]XX but they might be converted into weird things (String mostly?).
    /// TODO: We will need another pass at this, in a few places the type might matter.
    Number,
    /// Option are value that might be absent. So in oil, their presence must be checked.
    /// In Rust, type such as Option<_> or Result<_, *> or anything that implements IntoIterator
    /// could be used (*hint* for codegen :)). However, once we parse RustCode, it might end-up
    /// only be Option<_> and Result<_, *>. Unless we create our own trait...
    Option(Box<StoreType>),
    /// Product are mostly created from struct where we inspect all individual fields and aggregate
    /// them. If oil encounters fields where the type is either unkown to `oil` or can't - read shouldn't
    /// be inspected then the field is ignored. In some case this product might be empty. And that's fine.
    /// Enum could also lead to Product but Union seems to fit them better.
    Struct {
        fields: HashMap<String, StoreType>,
    },
    /// Rust tuples, tuples struct, or fixed length array will all end up being
    /// processed as an Array. That is, an indexable object with a compile-time known length.
    Array {
        elements: Vec<StoreType>,
    },
    /// This is how Vec<_> and friends are converted. But really, this is just about IntoIterator as for
    /// Option. The only difference, is that while reading data from it, we won't stop after the first element.
    UnboundedArray {
        element_type: Box<StoreType>,
    },
    /// Rust enums can be any of their variant. It means that for the oil type checker, all variant can be
    /// present but only some fields can be present at the same time. Beside, the complete set of fields is
    /// finite. So we can type check a lot using that information.
    /// The additional String for the variant here is not necessary for type checking itself. It is crucial
    /// for codegen though as without it the associated `match` can't be spelled out. :)
    Enum {
        variants: Vec<EnumVariant>,
    },
}

/// UnionVariant represent a particular variant of the StoreType::Union seen above.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    /// The name of the variant. Ignored during type checked. Only used during codegen.
    pub name: String,
    /// The type of the variant. Having a Union here is impossible (for now)
    /// In the future we might allow a type such as:
    ///
    /// ```rust
    /// enum A { A { a: u32, }, B }
    /// enum C { V1(A), V2 { b: u32 } }
    /// ```
    ///
    /// To allow usage such as (where `a` as type `C`):
    ///
    ///  - `a.b?`
    ///  - `a.a?`
    ///
    pub variant_type: VariantType,
}

/// The type of a variant in an enum.
#[derive(Debug, Clone, PartialEq)]
pub enum VariantType {
    /// A struct like variant. This is similar to StoreType::Struct.
    Struct {
        fields: HashMap<String, StoreType>,
    },
    /// A tuple variant.
    Tuple {
        elements: Vec<StoreType>,
    },
}