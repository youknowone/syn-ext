use crate::ident::GetIdent;
use crate::meta::{self, MetaExt};
use syn::{
    parse_quote, punctuated::Punctuated, token::Paren, Attribute, Ident, Meta, MetaList, Result,
};

impl GetIdent for Attribute {
    /// Get ident of the [syn::Attribute::path] field.
    fn get_ident(&self) -> Option<&Ident> {
        self.path.get_ident()
    }
}

/// Extension for [syn::Attribute]
#[cfg(feature = "parsing")]
pub trait AttributeExt {
    /// Constructs and returns a new [syn::Attribute] from [syn::Meta]
    fn from_meta<M>(meta: M) -> Self
    where
        M: IntoAttribute;

    /// Takes a closure and calls it with parsed meta. After call, applys back the manipulated [syn::Meta].
    ///
    /// 1. Try [syn::Attribute::parse_meta]; return if `Err`
    /// 2. Run `f`
    /// 3. Apply back the manipulated [syn::Meta] by `f`.
    /// 4. Return the result of `f`.
    ///
    /// Note: Even `f` returns `Err`, meta will be made into self.
    fn try_meta_mut<F, R>(&mut self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Meta) -> Result<R>;

    /// Returns a fake promoted list value of [syn::MetaList].
    ///
    /// If [syn::Meta::List], return inner [syn::MetaList].
    /// If [syn::Meta::Path], return a fake [syn::MetaList] with default paren and empty nested.
    /// Otherwise return `Err`
    fn promoted_list(&self) -> Result<MetaList>;

    /// Takes a closure and calls it with promoted list of parsed meta. After call, applys back the manipulated [syn::MetaList].
    ///
    /// 1. Try [syn::Attribute::parse_meta]; return if `Err`
    /// 2. Promote to [syn::Meta::List] if [syn::Meta::Path]
    /// 3. Run `f` to inner [syn::MetaList]
    /// 4. Apply back the manipulated [syn::Meta] by `f`.
    /// 5. Return the result of `f`.
    fn try_promoted_list_mut<F, R>(&mut self, paren: Paren, f: F) -> Result<R>
    where
        F: FnOnce(&mut MetaList) -> Result<R>;
}

#[cfg(feature = "parsing")]
impl AttributeExt for Attribute {
    fn from_meta<M>(meta: M) -> Self
    where
        M: IntoAttribute,
    {
        meta.into_attribute()
    }

    fn try_meta_mut<F, R>(&mut self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Meta) -> Result<R>,
    {
        let mut meta = self.parse_meta()?;
        let result = f(&mut meta);
        *self = Self::from_meta(meta);
        Ok(result?)
    }

    fn promoted_list(&self) -> Result<MetaList> {
        match self.parse_meta()? {
            Meta::Path(path) => Ok(MetaList {
                path,
                paren_token: Default::default(),
                nested: Punctuated::new(),
            }),
            Meta::List(metalist) => Ok(metalist),
            other => Err(meta::err_promote_to_list(&other)),
        }
    }

    fn try_promoted_list_mut<F, R>(&mut self, paren: Paren, f: F) -> Result<R>
    where
        F: FnOnce(&mut MetaList) -> Result<R>,
    {
        self.try_meta_mut(|meta| {
            let metalist = meta.promote_to_list(paren)?;
            f(metalist)
        })
    }
}

/// Extension for `std::iter::Iterator<[syn::Attribute]>`
#[cfg(feature = "parsing")]
pub trait AttributeIteratorExt {
    // fn doc_items(&self) -> impl std::iter::Iterator<Item=String>;
    /// Constructs and returns doc comment string by joining doc from multiple attrs
    fn doc(self) -> Option<String>;

    // fn filter_name<'a, P>(
    //     &'a self,
    //     name: &'static str,
    // ) -> std::iter::Filter<std::slice::Iter<'a, Attribute>, P>
    // where
    //     P: FnMut(&&'a syn::Attribute) -> bool;
}

#[cfg(feature = "parsing")]
impl<'a, I> AttributeIteratorExt for I
where
    I: std::iter::IntoIterator<Item = &'a Attribute>,
{
    fn doc(self) -> Option<String> {
        let items: Vec<_> = self
            .into_iter()
            .filter_map(|attr| attr.parse_meta().ok().and_then(|m| m.doc().ok()))
            .collect();
        if items.is_empty() {
            None
        } else {
            Some(items.join("\n"))
        }
    }

    // fn filter_name<'a, P>(
    //     &'a self,
    //     name: &'static str,
    // ) -> std::iter::Filter<std::slice::Iter<'a, Attribute>, P>
    // where
    //     P: FnMut(&&'a syn::Attribute) -> bool,
    // {
    //     let mut p = |attr: &&'a Attribute| {
    //         attr.path
    //             .get_ident()
    //             .map_or(false, |ident| ident.to_string() == name)
    //     };
    //     self.iter().filter(p)
    // }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_quote_eq;
    use quote::quote;

    #[cfg(feature = "parsing")]
    fn test_meta_round_trip(attr: Attribute) -> Result<()> {
        let meta = attr.parse_meta()?;
        let created = Attribute::from_meta(meta);
        assert_eq!(
            quote! { #attr }.to_string(),
            quote! { #created }.to_string()
        );
        Ok(())
    }

    #[cfg(feature = "parsing")]
    #[test]
    fn run_test_meta_round_trip() {
        use syn::parse_quote;
        test_meta_round_trip(parse_quote! { #[cfg(test)] }).unwrap();
        test_meta_round_trip(parse_quote! { #[feature = "full"] }).unwrap();
        test_meta_round_trip(parse_quote! { #[cfg(all(a,b,any(c,d)))] }).unwrap();
        test_meta_round_trip(parse_quote! { #[a(b="1",d)] }).unwrap();
        test_meta_round_trip(parse_quote! { #[abc::de::ef] }).unwrap();
    }

    #[cfg(feature = "parsing")]
    #[test]
    fn test_try_meta_mut() {
        let mut attr: Attribute = parse_quote! { #[cfg(test)] };
        attr.try_meta_mut(|meta| match meta {
            Meta::List(metalist) => {
                metalist.path = parse_quote! { newcfg };
                Ok(())
            }
            _ => unreachable!(),
        })
        .unwrap();
        let expected: Attribute = parse_quote! { #[newcfg(test)] };
        assert_quote_eq!(attr, expected);

        attr.try_meta_mut(|meta| match meta {
            Meta::List(metalist) => {
                metalist.nested.pop();
                metalist.nested.push(parse_quote!(a));
                metalist.nested.push(parse_quote!(b = "c"));
                metalist.nested.push(parse_quote!("d"));
                Ok(())
            }
            _ => unreachable!(),
        })
        .unwrap();
        let expected: Attribute = parse_quote! { #[newcfg(a, b="c", "d")] };
        assert_quote_eq!(attr, expected);
    }

    #[test]
    #[cfg(feature = "parsing")]
    fn test_promoted_list() {
        let attr: Attribute = parse_quote! { #[derive] };
        let list = attr.promoted_list().unwrap();
        assert_quote_eq!(attr.path, list.path);
        assert!(list.nested.is_empty());
    }

    #[cfg(all(feature = "parsing", feature = "full"))]
    #[test]
    fn test_doc() {
        let func: syn::ItemFn = parse_quote! {
            #[derive]
            /// doc line 1
            #[test]
            #[doc = "doc line 2"]
            #[cfg]
            fn f() {}
        };
        let doc = func.attrs.doc().unwrap();
        assert_eq!(doc, "doc line 1\ndoc line 2");
    }
}

pub trait IntoAttribute {
    fn into_attribute(self) -> Attribute;
}

impl IntoAttribute for Meta {
    fn into_attribute(self) -> Attribute {
        parse_quote!( #[ #self ] )
    }
}

impl IntoAttribute for MetaList {
    fn into_attribute(self) -> Attribute {
        Meta::List(self).into_attribute()
    }
}
