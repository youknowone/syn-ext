use crate::ident::GetIdent;
use crate::path::GetPath;
use crate::shared::{thread_local_ref, SharedEmpty};
use std::collections::HashMap as Map;
use syn::{
    punctuated::Punctuated, token, Error, Ident, Lit, Meta, MetaList, MetaNameValue, NestedMeta,
    Path, Result,
};

/// The nested field of Meta
pub type PunctuatedNestedMeta = Punctuated<NestedMeta, token::Comma>;

pub trait MetaExt {
    fn is_path(&self) -> bool;
    fn is_list(&self) -> bool;
    fn is_name_value(&self) -> bool;

    fn promote_to_list(&mut self, paren: token::Paren) -> Result<&mut MetaList>;

    fn list(&self) -> Result<&MetaList>;
    fn list_mut(&mut self) -> Result<&mut MetaList>;

    fn name_value(&self) -> Result<&MetaNameValue>;
    fn name_value_mut(&mut self) -> Result<&mut MetaNameValue>;
}

pub(crate) fn err_promote_to_list(meta: &Meta) -> Error {
    Error::new_spanned(
        meta,
        "Only Path can be promoted and List is accepted as non-promoted",
    )
}

impl MetaExt for Meta {
    fn is_path(&self) -> bool {
        matches!(self, Meta::Path(_))
    }
    fn is_list(&self) -> bool {
        matches!(self, Meta::List(_))
    }
    fn is_name_value(&self) -> bool {
        matches!(self, Meta::NameValue(_))
    }

    fn promote_to_list(&mut self, paren: token::Paren) -> Result<&mut MetaList> {
        let path = match self {
            Meta::Path(path) => path.clone(),
            Meta::List(metalist) => return Ok(metalist),
            other => return Err(err_promote_to_list(other)),
        };
        *self = Meta::List(MetaList {
            path,
            paren_token: paren,
            nested: PunctuatedNestedMeta::new(),
        });
        self.list_mut()
    }

    fn list(&self) -> Result<&MetaList> {
        match self {
            Meta::List(ref list) => Ok(list),
            other => Err(Error::new_spanned(other, "Not a List meta")),
        }
    }
    fn list_mut(&mut self) -> Result<&mut MetaList> {
        match self {
            Meta::List(ref mut list) => Ok(list),
            other => Err(Error::new_spanned(other, "Not a List meta")),
        }
    }

    fn name_value(&self) -> Result<&MetaNameValue> {
        match self {
            Meta::NameValue(ref name) => Ok(name),
            other => Err(Error::new_spanned(other, "Not a NameValue meta")),
        }
    }
    fn name_value_mut(&mut self) -> Result<&mut MetaNameValue> {
        match self {
            Meta::NameValue(ref mut name) => Ok(name),
            other => Err(Error::new_spanned(other, "Not a NameValue meta")),
        }
    }
}

impl GetPath for NestedMeta {
    fn get_path(&self) -> Option<&Path> {
        match self {
            NestedMeta::Meta(meta) => Some(meta.path()),
            NestedMeta::Lit(_) => None,
        }
    }
}

impl GetIdent for Meta {
    fn get_ident(&self) -> Option<&Ident> {
        self.path().get_ident()
    }
}

thread_local! {
    static EMPTY_META_NESTED: Punctuated<syn::NestedMeta, syn::token::Comma> = Punctuated::new();
}

impl SharedEmpty for Punctuated<syn::NestedMeta, syn::token::Comma> {
    fn empty_ref() -> &'static Self {
        unsafe { thread_local_ref(&EMPTY_META_NESTED) }
    }
}
