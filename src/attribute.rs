use crate::meta::PunctuatedNestedMeta;
use syn::{parse_quote, Attribute, Meta, MetaList};

thread_local! {
    static EMPTY_META_NESTED: PunctuatedNestedMeta = PunctuatedNestedMeta::new();
}

pub trait AttributeExt {
    fn from_meta<M>(meta: M) -> Self
    where
        M: IntoAttribute;
}

impl AttributeExt for Attribute {
    fn from_meta<M>(meta: M) -> Self
    where
        M: IntoAttribute,
    {
        meta.into_attribute()
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
