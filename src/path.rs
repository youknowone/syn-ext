use crate::ident::GetIdent;
use syn::{Ident, Path};

/// Shortcut to get [syn::Path] from various types
pub trait GetPath {
    /// Returns [syn::Path] from object if possible
    fn get_path(&self) -> Option<&Path>;
}

impl<T> GetIdent for T
where
    T: GetPath,
{
    /// Any [crate::ext::GetPath] automatically implements [crate::ext::GetIdent]
    fn get_ident(&self) -> Option<&Ident> {
        self.get_path().and_then(|p| p.get_ident())
    }
}
