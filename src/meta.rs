use crate::ident::GetIdent;
use crate::path::GetPath;
use crate::shared::{thread_local_ref, SharedEmpty};
use std::collections::HashMap as Map;
use syn::{
    punctuated::Punctuated, token, Attribute, Error, Ident, Lit, Meta, MetaList, MetaNameValue,
    NestedMeta, Path, Result,
};

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
    fn promote_to_list(&mut self, paren: token::Paren) -> Result<&mut MetaList>;

    /// Returns [syn::MetaList] of [syn::Meta::List]; Otherwise `Err`
    fn list(&self) -> Result<&MetaList>;
    /// Returns [syn::MetaList] of [syn::Meta::List]; Otherwise `Err`
    fn list_mut(&mut self) -> Result<&mut MetaList>;

    /// Returns [syn::MetaNameValue] of [syn::Meta::NameValue]; Otherwise `Err`
    fn name_value(&self) -> Result<&MetaNameValue>;
    /// Returns [syn::MetaNameValue] of [syn::Meta::NameValue]; Otherwise `Err`
    fn name_value_mut(&mut self) -> Result<&mut MetaNameValue>;

    /// Returns content of `doc = <content>`
    fn doc(&self) -> Result<String>;
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
    fn is_doc(&self) -> bool {
        self.name_value().map_or(false, |v| {
            v.path.is_ident("doc") && matches!(v.lit, Lit::Str(_))
        })
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

    fn doc(&self) -> Result<String> {
        let name_value = self.name_value()?;
        if !name_value.path.is_ident("doc") {
            return Err(Error::new_spanned(name_value, "Not a doc meta"));
        }
        match &name_value.lit {
            Lit::Str(lit) => Ok(lit.value().trim().to_owned()),
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
    M: 'a + std::borrow::Borrow<Meta>,
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
    M: 'a + std::borrow::Borrow<Meta>,
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
    M: 'a + std::borrow::Borrow<Meta>,
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
impl<'a, I> NestedMetaRefIteratorExt<'a, &'a Meta> for I
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
    ) -> Result<(MultiMetaMap<'a, K, &'a Meta>, Vec<(usize, &'a Lit)>)>
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
    ) -> Result<(UniqueMetaMap<'a, K, &'a Meta>, Vec<(usize, &'a Lit)>)>
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
    ) -> Result<(MultiMetaMap<'a, K, Meta>, Vec<(usize, Lit)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>;

    fn into_unique_map_and_lits<K, KF>(
        self,
        path_to_key: KF,
    ) -> Result<(UniqueMetaMap<'a, K, Meta>, Vec<(usize, Lit)>)>
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
    ) -> Result<(MultiMetaMap<'a, K, Meta>, Vec<(usize, Lit)>)>
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
    ) -> Result<(UniqueMetaMap<'a, K, Meta>, Vec<(usize, Lit)>)>
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
    ) -> Result<(MultiMetaMap<'a, K, Meta>, Vec<(usize, &'a Attribute)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>;

    fn to_unique_map_and_attrs<K, KF>(
        self,
        path_to_key: KF,
    ) -> Result<(UniqueMetaMap<'a, K, Meta>, Vec<(usize, &'a Attribute)>)>
    where
        K: std::hash::Hash + Eq,
        KF: Fn(&Path) -> Result<Option<K>>;
}

#[cfg(any(feature = "parsing"))]
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
    ) -> Result<(MultiMetaMap<'a, K, Meta>, Vec<(usize, &'a Attribute)>)>
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
    ) -> Result<(UniqueMetaMap<'a, K, Meta>, Vec<(usize, &'a Attribute)>)>
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

impl GetIdent for Meta {
    /// Get ident of [syn::Meta::path]
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
