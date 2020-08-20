use crate::meta::PunctuatedNestedMeta;
use syn::{parse_quote, Attribute, Meta, MetaList, Result};

thread_local! {
    static EMPTY_META_NESTED: PunctuatedNestedMeta = PunctuatedNestedMeta::new();
}

pub trait AttributeExt {
    fn from_meta<M>(meta: M) -> Self
    where
        M: IntoAttribute;

    fn try_mut_as_meta<F, R>(&mut self, f: F) -> Result<R>
    where
        F: Fn(&mut Meta) -> Result<R>;
}

impl AttributeExt for Attribute {
    fn from_meta<M>(meta: M) -> Self
    where
        M: IntoAttribute,
    {
        meta.into_attribute()
    }

    fn try_mut_as_meta<F, R>(&mut self, f: F) -> Result<R>
    where
        F: Fn(&mut Meta) -> Result<R>,
    {
        let mut meta = self.parse_meta()?;
        let result = f(&mut meta)?;
        *self = Self::from_meta(meta);
        Ok(result)
    }
}

pub trait IntoAttribute {
    fn into_attribute(self) -> Attribute;
}

impl IntoAttribute for Meta {
    fn into_attribute(self) -> Attribute {
        parse_quote!( #[ self ] )
    }
}

impl IntoAttribute for MetaList {
    fn into_attribute(self) -> Attribute {
        Meta::List(self).into_attribute()
    }
}
