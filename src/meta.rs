use syn::{punctuated::Punctuated, token::Comma, Meta, NestedMeta};

pub(crate) type PunctuatedNestedMeta = Punctuated<NestedMeta, Comma>;

thread_local! {
    static EMPTY_META_NESTED: PunctuatedNestedMeta = PunctuatedNestedMeta::new();
}

pub trait MetaExt {
    fn is_list_like(&self) -> bool;
    fn get_list_like_nested(&self) -> Option<&PunctuatedNestedMeta>;

    fn is_name_value_like(&self) -> bool;
}

fn empty_meta_nested() -> &'static PunctuatedNestedMeta {
    let ptr = EMPTY_META_NESTED.with(|nested| nested as *const _);
    unsafe {
        // Safety: Read-only thread-local always has the same value
        &*ptr
    }
}

impl MetaExt for Meta {
    fn is_list_like(&self) -> bool {
        match self {
            Meta::Path(_) => true,
            Meta::List(_) => true,
            Meta::NameValue(_) => false,
        }
    }

    fn is_name_value_like(&self) -> bool {
        match self {
            Meta::Path(_) => true,
            Meta::List(_) => false,
            Meta::NameValue(_) => true,
        }
    }

    fn get_list_like_nested(&self) -> Option<&PunctuatedNestedMeta> {
        match self {
            Meta::Path(_) => Some(empty_meta_nested()),
            Meta::List(list) => Some(&list.nested),
            Meta::NameValue(_) => None,
        }
    }
}
