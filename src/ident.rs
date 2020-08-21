use syn::Ident;

/// Shortcut to get [syn::Ident](struct@syn::Ident) from various types
pub trait GetIdent {
    /// Returns reference of ident if its [syn::Path] is [syn::Ident](struct@syn::Ident); Otherwise None
    ///
    /// Any [crate::ext::GetPath] also implements `GetIdent`.
    fn get_ident(&self) -> Option<&Ident>;
}
