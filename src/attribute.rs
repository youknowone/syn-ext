use crate::ident::GetIdent;
use crate::meta::{self, MetaExt};
use syn::{
    parse_quote, punctuated::Punctuated, token::Paren, Attribute, Ident, Meta, MetaList, Result,
};

impl GetIdent for Attribute {
    fn get_ident(&self) -> Option<&Ident> {
        self.path.get_ident()
    }
}

#[cfg(feature = "parsing")]
pub trait AttributeExt {
    fn from_meta<M>(meta: M) -> Self
    where
        M: IntoAttribute;

    fn try_meta_mut<F, R>(&mut self, f: F) -> Result<R>
    where
        F: Fn(&mut Meta) -> Result<R>;

    fn promoted_list(&self) -> Result<MetaList>;

    fn try_promoted_list_mut<F, R>(&mut self, paren: Paren, f: F) -> Result<R>
    where
        F: Fn(&mut MetaList) -> Result<R>;
}

#[cfg(feature = "parsing")]
impl AttributeExt for Attribute {
    fn from_meta<M>(meta: M) -> Self
    where
        M: IntoAttribute,
    {
        meta.into_attribute()
    }

    /// Edit meta and build self from it
    fn try_meta_mut<F, R>(&mut self, f: F) -> Result<R>
    where
        F: Fn(&mut Meta) -> Result<R>,
    {
        let mut meta = self.parse_meta()?;
        let result = f(&mut meta)?;
        *self = Self::from_meta(meta);
        Ok(result)
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
        F: Fn(&mut MetaList) -> Result<R>,
    {
        self.try_meta_mut(|meta| {
            let metalist = meta.promote_to_list(paren)?;
            f(metalist)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_quote_eq;
    use quote::quote;

    fn test_meta_round_trip(attr: Attribute) -> Result<()> {
        let meta = attr.parse_meta()?;
        let created = Attribute::from_meta(meta);
        assert_eq!(
            quote! { #attr }.to_string(),
            quote! { #created }.to_string()
        );
        Ok(())
    }

    #[test]
    fn run_test_meta_round_trip() {
        use syn::parse_quote;
        test_meta_round_trip(parse_quote! { #[cfg(test)] }).unwrap();
        test_meta_round_trip(parse_quote! { #[feature = "full"] }).unwrap();
        test_meta_round_trip(parse_quote! { #[cfg(all(a,b,any(c,d)))] }).unwrap();
        test_meta_round_trip(parse_quote! { #[a(b="1",d)] }).unwrap();
        test_meta_round_trip(parse_quote! { #[abc::de::ef] }).unwrap();
    }

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
    fn test_promoted_list() {
        let attr: Attribute = parse_quote! { #[derive] };
        let list = attr.promoted_list().unwrap();
        assert_quote_eq!(attr.path, list.path);
        assert!(list.nested.is_empty());
    }
}

// pub trait AttributesExt {
//     fn filter_name<'a, P>(
//         &'a self,
//         name: &'static str,
//     ) -> std::iter::Filter<std::slice::Iter<'a, Attribute>, P>
//     where
//         P: FnMut(&&'a syn::Attribute) -> bool;
// }

// impl AttributesExt for [Attribute] {
//     fn filter_name<'a, P>(
//         &'a self,
//         name: &'static str,
//     ) -> std::iter::Filter<std::slice::Iter<'a, Attribute>, P>
//     where
//         P: FnMut(&&'a syn::Attribute) -> bool,
//     {
//         let mut p = |attr: &&'a Attribute| {
//             attr.path
//                 .get_ident()
//                 .map_or(false, |ident| ident.to_string() == name)
//         };
//         self.iter().filter(p)
//     }
// }

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
