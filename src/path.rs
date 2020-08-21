use crate::ident::GetIdent;
use crate::shared::{thread_local_ref, SharedEmpty};
use syn::{punctuated::Punctuated, Ident, Path};

pub trait GetPath {
    fn get_path(&self) -> Option<&Path>;
}

impl<T> GetIdent for T
where
    T: GetPath,
{
    fn get_ident(&self) -> Option<&Ident> {
        self.get_path().map(|p| p.get_ident()).flatten()
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
