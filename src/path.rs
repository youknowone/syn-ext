use crate::ident::GetIdent;
use crate::shared::{thread_local_ref, SharedEmpty};
use syn::{punctuated::Punctuated, Ident, Path};

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

thread_local! {
    static EMPTY_PATH: Path = Path::with_empty();
}

impl SharedEmpty for Path {
    fn empty_ref() -> &'static Self {
        unsafe { thread_local_ref(&EMPTY_PATH) }
    }
}

pub trait PathExt {
    /// Constructs and returns an empty path with empty segments
    fn with_empty() -> Path;
}

impl PathExt for Path {
    fn with_empty() -> Path {
        Path {
            leading_colon: None,
            segments: Punctuated::new(),
        }
    }
}
