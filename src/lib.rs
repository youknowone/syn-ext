//! Collections of syn shortcuts and editable interface.
//!
//! Start with `use syn_ext::ext::*;`
//! Look around extension methods in [ext module](ext/index.html)

#[cfg(any(feature = "derive", feature = "full"))]
mod attribute;
#[cfg(any(feature = "derive", feature = "full"))]
mod generics;
mod ident;
#[cfg(feature = "full")]
mod item;
#[cfg(any(feature = "derive", feature = "full"))]
mod meta;
mod path;
mod punctuated;
#[cfg(test)]
#[macro_use]
mod test;

/// `use syn_ext::ext::*`;  // Namespace module for extension traits.
///
/// Always try to use `*`.
/// The public names here are intended to be used as `*` and will be changed any time.
pub mod ext {
    // only extension traits can be named here
    mod basic {
        pub use crate::ident::GetIdent;
        pub use crate::punctuated::PunctuatedExt;
    }
    #[cfg(any(feature = "derive", feature = "full"))]
    mod derive {
        #[cfg(feature = "parsing")]
        pub use crate::attribute::{AttributeExt, AttributeIteratorExt};
        #[cfg(feature = "parsing")]
        pub use crate::meta::MetaAttributeExt;
        pub use crate::meta::{
            MetaExt, MetaIteratorExt, NestedMetaIteratorExt, NestedMetaRefIteratorExt,
        };
        pub use crate::path::GetPath;
    }
    #[cfg(feature = "full")]
    mod full {
        pub use crate::item::{ItemAttrExt, ItemLike, ItemModExt};
    }

    pub use basic::*;
    #[cfg(any(feature = "derive", feature = "full"))]
    pub use derive::*;
    #[cfg(feature = "full")]
    pub use full::*;
}

pub mod types {
    #[cfg(any(feature = "derive", feature = "full"))]
    pub use crate::meta::{Meta1 as Meta, MetaList1 as MetaList, NestedMeta, PunctuatedNestedMeta};
}
