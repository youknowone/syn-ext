#[cfg(feature = "parsing")]
use crate::attribute::AttributeExt;
use crate::ident::GetIdent;
use crate::path::GetPath;
use std::collections::HashMap as Map;
use std::convert::TryFrom;
use syn::ext::IdentExt;
use syn::parse::Parser;
use syn::{
    punctuated::Punctuated, token, Attribute, Error, Expr, ExprLit, Ident, Lit, Meta as Meta2,
    MetaNameValue, Path, Result,
};

#[derive(Clone)]
pub enum Meta1 {
    Path(Path),
    List(MetaList1),
    NameValue(MetaNameValue),
}

#[cfg(feature = "parsing")]
impl syn::parse::Parse for Meta1 {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let path = input.call(parse_meta_path)?;
        if input.peek(token::Paren) {
            Ok(Meta1::List(MetaList1::parse_meta_list_after_path(
                path, input,
            )?))
        } else if input.peek(syn::Token![=]) {
            Ok(Meta1::NameValue(MetaNameValue {
                path,
                eq_token: input.parse()?,
                value: degroup(input.parse()?),
            }))
        } else {
            Ok(Meta1::Path(path))
        }
    }
}

#[cfg(feature = "parsing")]
fn degroup(mut expr: Expr) -> Expr {
    while let Expr::Group(group) = expr {
        expr = *group.expr
    }
    expr
}

impl TryFrom<Meta2> for Meta1 {
    type Error = syn::Error;

    fn try_from(meta: Meta2) -> std::result::Result<Self, Self::Error> {
        Ok(match meta {
            Meta2::Path(path) => Meta1::Path(path),
            Meta2::List(list) => Meta1::List(MetaList1 {
                path: list.path,
                paren_token: match list.delimiter {
                    syn::MacroDelimiter::Paren(paren) => paren,
                    other => return Err(syn::Error::new(other.span().open(), "expected paren")),
                },
                nested: PunctuatedNestedMeta::parse_terminated.parse2(list.tokens)?,
            }),
            Meta2::NameValue(nv) => Meta1::NameValue(nv),
        })
    }
}

impl quote::ToTokens for Meta1 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Meta1::Path(path) => path.to_tokens(tokens),
            Meta1::List(list) => list.to_tokens(tokens),
            Meta1::NameValue(nv) => nv.to_tokens(tokens),
        }
    }
}

impl Meta1 {
    pub fn path(&self) -> &Path {
        match self {
            Meta1::Path(path) => path,
            Meta1::List(list) => &list.path,
            Meta1::NameValue(nv) => &nv.path,
        }
    }
}

#[cfg(feature = "parsing")]
fn parse_meta_path(input: syn::parse::ParseStream) -> Result<Path> {
    Ok(Path {
        leading_colon: input.parse()?,
        segments: {
            let mut segments = Punctuated::new();
            while input.peek(Ident::peek_any) {
                let ident = Ident::parse_any(input)?;
                segments.push_value(syn::PathSegment::from(ident));
                if !input.peek(syn::Token![::]) {
                    break;
                }
                let punct = input.parse()?;
                segments.push_punct(punct);
            }
            if segments.is_empty() {
                return Err(input.error("expected path"));
            } else if segments.trailing_punct() {
                return Err(input.error("expected path segment"));
            }
            segments
        },
    })
}

#[derive(Clone)]
pub struct MetaList1 {
    pub path: Path,
    pub paren_token: token::Paren,
    pub nested: PunctuatedNestedMeta,
}

#[cfg(feature = "parsing")]
impl MetaList1 {
    fn parse_meta_list_after_path(path: Path, input: syn::parse::ParseStream) -> Result<Self> {
        let content;
        Ok(MetaList1 {
            path,
            paren_token: syn::parenthesized!(content in input),
            nested: PunctuatedNestedMeta::parse_terminated(&content)?,
        })
    }
}
#[cfg(feature = "parsing")]
impl syn::parse::Parse for MetaList1 {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let path = input.call(parse_meta_path)?;
        Self::parse_meta_list_after_path(path, input)
    }
}

impl quote::ToTokens for MetaList1 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.path.to_tokens(tokens);
        self.paren_token
            .surround(tokens, |tokens| self.nested.to_tokens(tokens));
    }
}

#[derive(Clone)]
pub enum NestedMeta {
    Meta(Meta1),
    Lit(Lit),
}

#[cfg(feature = "parsing")]
impl syn::parse::Parse for NestedMeta {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        if input.peek(Lit) && !(input.peek(syn::LitBool) && input.peek2(syn::Token![=])) {
            input.parse().map(NestedMeta::Lit)
        } else if input.peek(Ident::peek_any)
            || input.peek(syn::Token![::]) && input.peek3(Ident::peek_any)
        {
            input.parse().map(NestedMeta::Meta)
        } else {
            Err(input.error("expected identifier or literal"))
        }
    }
}

impl quote::ToTokens for NestedMeta {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            NestedMeta::Meta(meta) => meta.to_tokens(tokens),
            NestedMeta::Lit(lit) => lit.to_tokens(tokens),
        }
    }
}

/// Shortcut type for [syn::MetaList::nested]
pub type PunctuatedNestedMeta = Punctuated<NestedMeta, token::Comma>;

/// Extension for [syn::Meta]
pub trait MetaExt {
    /// Returns `true` if self matches [syn::Meta::Path]
    fn is_path(&self) -> bool;
    /// Returns `true` if self matches [syn::Meta::List]
    fn is_list(&self) -> bool;
    /// Returns `true` if self matches [syn::Meta::NameValue]
    fn is_name_value(&self) -> bool;
    /// Returns `true` if the content matches `doc = <string lit>`
    fn is_doc(&self) -> bool;

    /// Promotes to empty [syn::Meta::List] with given `paren` if [syn::Meta::Path]
    ///
    /// A [syn::Meta::Path] value can be regarded as an empty [syn::Meta::List].
    /// `promote` means converting [syn::Meta::Path] to an actual empty [syn::Meta::List].
    fn promote_to_list(&mut self, paren: token::Paren) -> Result<&mut MetaList1>;

    /// Returns [syn::MetaList] of [syn::Meta::List]; Otherwise `Err`
    fn list(&self) -> Result<&MetaList1>;
    /// Returns [syn::MetaList] of [syn::Meta::List]; Otherwise `Err`
    fn list_mut(&mut self) -> Result<&mut MetaList1>;

    /// Returns [syn::MetaNameValue] of [syn::Meta::NameValue]; Otherwise `Err`
    fn name_value(&self) -> Result<&MetaNameValue>;
    /// Returns [syn::MetaNameValue] of [syn::Meta::NameValue]; Otherwise `Err`
    fn name_value_mut(&mut self) -> Result<&mut MetaNameValue>;

    /// Returns content of `doc = <content>`
    fn doc(&self) -> Result<String>;
}

pub(crate) fn err_promote_to_list(meta: &Meta1) -> Error {
    Error::new_spanned(
        meta,
        "Only Path can be promoted and List is accepted as non-promoted",
    )
}

impl MetaExt for Meta1 {
    fn is_path(&self) -> bool {
        matches!(self, Meta1::Path(_))
    }
    fn is_list(&self) -> bool {
        matches!(self, Meta1::List(_))
    }
    fn is_name_value(&self) -> bool {
        matches!(self, Meta1::NameValue(_))
    }
    fn is_doc(&self) -> bool {
        self.name_value().is_ok_and(|v| {
            v.path.is_ident("doc")
                && matches!(
                    v.value,
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(_),
                        ..
                    })
                )
        })
    }

    fn promote_to_list(&mut self, paren: token::Paren) -> Result<&mut MetaList1> {
        let path = match self {
            Meta1::Path(path) => path.clone(),
            Meta1::List(metalist) => return Ok(metalist),
            other => return Err(err_promote_to_list(other)),
        };
        *self = Meta1::List(MetaList1 {
            path,
            paren_token: paren,
            nested: PunctuatedNestedMeta::new(),
        });
        self.list_mut()
    }

    fn list(&self) -> Result<&MetaList1> {
        match self {
            Meta1::List(ref list) => Ok(list),
            other => Err(Error::new_spanned(other, "Not a List meta")),
        }
    }
    fn list_mut(&mut self) -> Result<&mut MetaList1> {
        match self {
            Meta1::List(ref mut list) => Ok(list),
            other => Err(Error::new_spanned(other, "Not a List meta")),
        }
    }

    fn name_value(&self) -> Result<&MetaNameValue> {
        match self {
            Meta1::NameValue(ref name) => Ok(name),
            other => Err(Error::new_spanned(other, "Not a NameValue meta")),
        }
    }
    fn name_value_mut(&mut self) -> Result<&mut MetaNameValue> {
        match self {
            Meta1::NameValue(ref mut name) => Ok(name),
            other => Err(Error::new_spanned(other, "Not a NameValue meta")),
        }
    }

    fn doc(&self) -> Result<String> {
        let name_value = self.name_value()?;
        if !name_value.path.is_ident("doc") {
            return Err(Error::new_spanned(name_value, "Not a doc meta"));
        }
        match &name_value.value {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit), ..
            }) => Ok(lit.value().trim().to_owned()),
            other => Err(Error::new_spanned(other, "Doc meta expects string literal")),
        }
    }
}

// where  M: 'a + std::borrow::Borrow<Meta>
type IndexMetaRef<M> = (usize, M);
type MultiMetaMap<'a, K, M> = Map<K, Vec<IndexMetaRef<M>>>;
type UniqueMetaMap<'a, K, M> = Map<K, IndexMetaRef<M>>;

/// Constructs and returns map from [syn::Meta] iterator
pub trait MetaIteratorExt<'a, M>
where
    M: 'a + std::borrow::Borrow<Meta1>,
{
    /// Constructs and returns a multi-value map from [syn::Meta] iterator
    fn to_multi_map<K, KF>(self, path_to_key: KF) -> Result<MultiMetaMap<'a, K, M>>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>;

    /// Constructs and returns a unique-value map from [syn::Meta] iterator. `Err` if duplicates.
    fn to_unique_map<K, KF>(self, path_to_key: KF) -> Result<UniqueMetaMap<'a, K, M>>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>;
}

// fn to_multi_map<'a, I, M, K, KF>(iter: I, path_to_key: KF) -> Result<MultiMetaMap<'a, K, M>>
// where
//     M: 'a + std::borrow::Borrow<Meta>,
//     I: std::iter::Iterator<Item = IndexMetaRef<M>>,
//     K: std::hash::Hash + Eq,
//     KF: Fn(&Path) -> Result<Option<K>>,
// {
//     let mut map: Map<K, Vec<_>> = Map::new();
//     for (i, meta) in iter {
//         let path = meta.borrow().path();
//         let key = if let Some(key) = path_to_key(path)? {
//             key
//         } else {
//             continue;
//         };
//         map.entry(key).or_default().push((i, meta))
//     }
//     Ok(map)
// }

impl<'a, I, M> MetaIteratorExt<'a, M> for I
where
    M: 'a + std::borrow::Borrow<Meta1>,
    I: std::iter::Iterator<Item = IndexMetaRef<M>>,
{
    // easier KF with traits?
    // KF: Err(_) for error, Ok(None) to skip, Ok(Some(_)) to push

    fn to_multi_map<K, KF>(self, path_to_key: KF) -> Result<MultiMetaMap<'a, K, M>>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>,
    {
        let mut map: Map<K, Vec<_>> = Map::new();
        for (i, meta) in self {
            let path = meta.borrow().path();
            let key = if let Some(key) = path_to_key(path)? {
                key
            } else {
                continue;
            };
            map.entry(key).or_default().push((i, meta))
        }
        Ok(map)
    }

    fn to_unique_map<K, KF>(self, path_to_key: KF) -> Result<UniqueMetaMap<'a, K, M>>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>,
    {
        let mut map = Map::new();
        for (i, meta) in self {
            let path = meta.borrow().path();
            let key = if let Some(key) = path_to_key(path)? {
                key
            } else {
                continue;
            };
            if let Some((_, removed)) = map.insert(key, (i, meta)) {
                return Err(Error::new_spanned(
                    removed.borrow(),
                    "this attribute path must be unique in the attribute",
                ));
            }
        }
        Ok(map)
    }
}

/// experimental
#[allow(clippy::type_complexity)]
pub trait NestedMetaRefIteratorExt<'a, M>
where
    M: 'a + std::borrow::Borrow<Meta1>,
{
    // fn to_map_and_lits<K, KF, MT, MF, MI>(
    //     &'a self,
    //     path_to_key: KF,
    //     mata_to_map: MF,
    // ) -> Result<(MT, Vec<(usize, &'a Lit)>)>
    // where
    //     K: std::hash::Hash + Eq,
    //     KF: Fn(&Path) -> Result<Option<K>>,
    //     MF: Fn(MI, KF) -> Result<MT>,
    //     MI: std::iter::Iterator<Item = (usize, &'a Meta)>;

    fn to_multi_map_and_lits<K, KF>(
        self,
        path_to_key: KF,
    ) -> Result<(MultiMetaMap<'a, K, M>, Vec<(usize, &'a Lit)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>;

    fn to_unique_map_and_lits<K, KF>(
        self,
        path_to_key: KF,
    ) -> Result<(UniqueMetaMap<'a, K, M>, Vec<(usize, &'a Lit)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>;
}

#[allow(clippy::type_complexity)]
impl<'a, I> NestedMetaRefIteratorExt<'a, &'a Meta1> for I
where
    I: std::iter::IntoIterator<Item = &'a NestedMeta>,
{
    // easier KF with traits?
    // KF: Err(_) for error, Ok(None) to skip, Ok(Some(_)) to push

    // fn to_map_and_lits<K, KF, MT, MF, MI>(
    //     &'a self,
    //     path_to_key: KF,
    //     mata_iter_to_map: MF,
    // ) -> Result<(MT, Vec<(usize, &'a Lit)>)>
    // where
    //     K: std::hash::Hash + Eq,
    //     KF: Fn(&Path) -> Result<Option<K>>,
    //     MF: Fn(MI, KF) -> Result<MT>,
    //     MI: std::iter::Iterator<Item = (usize, &'a Meta)>,
    // {
    //     let mut metas: Vec<(_, &'a _)> = Vec::new();
    //     let mut lits = Vec::new();

    //     for (i, nmeta) in self.iter().enumerate() {
    //         match nmeta {
    //             NestedMeta::Meta(meta) => metas.push((i, meta)),
    //             NestedMeta::Lit(lit) => lits.push((i, lit)),
    //         }
    //     }

    //     let map = mata_iter_to_map(metas.into_iter(), path_to_key)?;
    //     Ok((map, lits))
    // }

    fn to_multi_map_and_lits<K, KF>(
        self,
        path_to_key: KF,
    ) -> Result<(MultiMetaMap<'a, K, &'a Meta1>, Vec<(usize, &'a Lit)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>,
    {
        let mut metas = Vec::new();
        let mut lits = Vec::new();

        for (i, nmeta) in self.into_iter().enumerate() {
            match nmeta {
                NestedMeta::Meta(meta) => metas.push((i, meta)),
                NestedMeta::Lit(lit) => lits.push((i, lit)),
            }
        }

        let map = metas.into_iter().to_multi_map(path_to_key)?;
        Ok((map, lits))
    }

    fn to_unique_map_and_lits<K, KF>(
        self,
        path_to_key: KF,
    ) -> Result<(UniqueMetaMap<'a, K, &'a Meta1>, Vec<(usize, &'a Lit)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>,
    {
        let mut metas = Vec::new();
        let mut lits = Vec::new();

        for (i, nmeta) in self.into_iter().enumerate() {
            match nmeta {
                NestedMeta::Meta(meta) => metas.push((i, meta)),
                NestedMeta::Lit(lit) => lits.push((i, lit)),
            }
        }
        let map = metas.into_iter().to_unique_map(path_to_key)?;
        Ok((map, lits))
    }
}

/// experimental
#[allow(clippy::type_complexity)]
pub trait NestedMetaIteratorExt<'a> {
    fn into_multi_map_and_lits<K, KF>(
        self,
        path_to_key: KF,
    ) -> Result<(MultiMetaMap<'a, K, Meta1>, Vec<(usize, Lit)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>;

    fn into_unique_map_and_lits<K, KF>(
        self,
        path_to_key: KF,
    ) -> Result<(UniqueMetaMap<'a, K, Meta1>, Vec<(usize, Lit)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>;
}

#[allow(clippy::type_complexity)]
impl<'a, I> NestedMetaIteratorExt<'a> for I
where
    I: std::iter::IntoIterator<Item = NestedMeta>,
{
    fn into_multi_map_and_lits<K, KF>(
        self,
        path_to_key: KF,
    ) -> Result<(MultiMetaMap<'a, K, Meta1>, Vec<(usize, Lit)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>,
    {
        let mut metas = Vec::new();
        let mut lits = Vec::new();

        for (i, nmeta) in self.into_iter().enumerate() {
            match nmeta {
                NestedMeta::Meta(meta) => metas.push((i, meta)),
                NestedMeta::Lit(lit) => lits.push((i, lit)),
            }
        }

        let map = metas.into_iter().to_multi_map(path_to_key)?;
        Ok((map, lits))
    }

    fn into_unique_map_and_lits<K, KF>(
        self,
        path_to_key: KF,
    ) -> Result<(UniqueMetaMap<'a, K, Meta1>, Vec<(usize, Lit)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>,
    {
        let mut metas = Vec::new();
        let mut lits = Vec::new();

        for (i, nmeta) in self.into_iter().enumerate() {
            match nmeta {
                NestedMeta::Meta(meta) => metas.push((i, meta)),
                NestedMeta::Lit(lit) => lits.push((i, lit)),
            }
        }
        let map = metas.into_iter().to_unique_map(path_to_key)?;
        Ok((map, lits))
    }
}

/// experimental
#[allow(clippy::type_complexity)]
pub trait MetaAttributeExt<'a> {
    fn to_multi_map_and_attrs<K, KF>(
        self,
        path_to_key: KF,
    ) -> Result<(MultiMetaMap<'a, K, Meta1>, Vec<(usize, &'a Attribute)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>;

    fn to_unique_map_and_attrs<K, KF>(
        self,
        path_to_key: KF,
    ) -> Result<(UniqueMetaMap<'a, K, Meta1>, Vec<(usize, &'a Attribute)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>;
}

#[cfg(feature = "parsing")]
#[allow(clippy::type_complexity)]
impl<'a, I> MetaAttributeExt<'a> for I
where
    I: std::iter::IntoIterator<Item = &'a Attribute>,
{
    // easier KF with traits?
    // KF: Err(_) for error, Ok(None) to skip, Ok(Some(_)) to push

    // fn to_map_and_attrs<K, KF, MT, MF, MI>(
    //     &'a self,
    //     path_to_key: KF,
    //     mata_iter_to_map: MF,
    // ) -> Result<(MT, Vec<(usize, &'a Attribute)>)>
    // where
    //     K: std::hash::Hash + Eq,
    //     KF: Fn(&Path) -> Result<Option<K>>,
    //     MF: Fn(MI, KF) -> Result<MT>,
    //     MI: std::iter::Iterator<Item = IndexMetaRef<Meta>>,
    // {
    //     let mut metas = Vec::new();
    //     let mut attrs = Vec::new();

    //     for (i, attr) in self.iter().enumerate() {
    //         match attr.parse_meta() {
    //             Ok(meta) => metas.push((i, meta)),
    //             Err(_) => attrs.push((i, attr)),
    //         }
    //     }

    //     let map = mata_iter_to_map(metas.into_iter(), path_to_key)?;
    //     Ok((map, attrs))
    // }

    fn to_multi_map_and_attrs<K, KF>(
        self,
        path_to_key: KF,
    ) -> Result<(MultiMetaMap<'a, K, Meta1>, Vec<(usize, &'a Attribute)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>,
    {
        let mut metas = Vec::new();
        let mut attrs = Vec::new();

        for (i, attr) in self.into_iter().enumerate() {
            match attr.parse_meta() {
                Ok(meta) => metas.push((i, meta)),
                Err(_) => attrs.push((i, attr)),
            }
        }

        let map = metas.into_iter().to_multi_map(path_to_key)?;
        Ok((map, attrs))
    }

    fn to_unique_map_and_attrs<K, KF>(
        self,
        path_to_key: KF,
    ) -> Result<(UniqueMetaMap<'a, K, Meta1>, Vec<(usize, &'a Attribute)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>,
    {
        let mut metas = Vec::new();
        let mut attrs = Vec::new();

        for (i, attr) in self.into_iter().enumerate() {
            match attr.parse_meta() {
                Ok(meta) => metas.push((i, meta)),
                Err(_) => attrs.push((i, attr)),
            }
        }
        let map = metas.into_iter().to_unique_map(path_to_key)?;
        Ok((map, attrs))
    }
}

impl GetPath for NestedMeta {
    /// Get path if [syn::NestedMeta::Meta]; Otherwise `None`
    fn get_path(&self) -> Option<&Path> {
        match self {
            NestedMeta::Meta(meta) => Some(meta.path()),
            NestedMeta::Lit(_) => None,
        }
    }
}

impl GetIdent for Meta1 {
    /// Get ident of [syn::Meta::path]
    fn get_ident(&self) -> Option<&Ident> {
        self.path().get_ident()
    }
}
