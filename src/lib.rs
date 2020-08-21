#[cfg(any(feature = "derive", feature = "full"))]
mod attribute;
mod ident;
#[cfg(feature = "full")]
mod item;
#[cfg(any(feature = "derive", feature = "full"))]
mod meta;
mod path;
mod punctuated;
mod shared;

/// use syn_ext::ext::*;  // Namespace module for extension traits.
///
/// There are many ext items to keep in track for human. Try `*`.
pub mod ext {
    // only extension traits can be named here
    mod basic {
        pub use crate::ident::GetIdent;
        pub use crate::punctuated::PunctuatedExt;
    }
    #[cfg(any(feature = "derive", feature = "full"))]
    mod derive {
        #[cfg(any(feature = "parsing"))]
        pub use crate::attribute::AttributeExt;
        pub use crate::meta::MetaExt;
        pub use crate::path::GetPath;
    }
    #[cfg(feature = "full")]
    mod full {
        pub use crate::item::{ItemExt, ItemModExt};
    }

    pub use basic::*;
    #[cfg(any(feature = "derive", feature = "full"))]
    pub use derive::*;
    #[cfg(feature = "full")]
    pub use full::*;
}

pub mod types {
    #[cfg(any(feature = "derive", feature = "full"))]
    pub use crate::meta::PunctuatedNestedMeta;
}
