use syn::{Attribute, Error, Item, ItemMod, Result};

/// Extension for [syn::Item]
pub trait ItemExt {
    /// Returns reference of inner attrs if not verbatim; otherwise `Err`
    fn attrs(&self) -> Result<&[Attribute]>;
    /// Returns mutable reference of inner attrs if not verbatim; otherwise `Err`
    fn attrs_mut(&mut self) -> Result<&mut Vec<Attribute>>;
}

impl ItemExt for Item {
    fn attrs(&self) -> Result<&[Attribute]> {
        use syn::Item::*;
        use syn::*;
        let attrs = match self {
            Const(ItemConst { ref attrs, .. }) => attrs,
            Enum(ItemEnum { ref attrs, .. }) => attrs,
            ExternCrate(ItemExternCrate { ref attrs, .. }) => attrs,
            Fn(ItemFn { ref attrs, .. }) => attrs,
            ForeignMod(ItemForeignMod { ref attrs, .. }) => attrs,
            Impl(ItemImpl { ref attrs, .. }) => attrs,
            Macro(ItemMacro { ref attrs, .. }) => attrs,
            Macro2(ItemMacro2 { ref attrs, .. }) => attrs,
            Mod(ItemMod { ref attrs, .. }) => attrs,
            Static(ItemStatic { ref attrs, .. }) => attrs,
            Struct(ItemStruct { ref attrs, .. }) => attrs,
            Trait(ItemTrait { ref attrs, .. }) => attrs,
            TraitAlias(ItemTraitAlias { ref attrs, .. }) => attrs,
            Type(ItemType { ref attrs, .. }) => attrs,
            Union(ItemUnion { ref attrs, .. }) => attrs,
            Use(ItemUse { ref attrs, .. }) => attrs,
            other => {
                return Err(Error::new_spanned(
                    other,
                    "this kind of item doesn't have attrs",
                ))
            }
        };
        Ok(attrs)
    }

    fn attrs_mut(&mut self) -> Result<&mut Vec<Attribute>> {
        use syn::Item::*;
        use syn::*;
        let attrs = match self {
            Const(ItemConst { ref mut attrs, .. }) => attrs,
            Enum(ItemEnum { ref mut attrs, .. }) => attrs,
            ExternCrate(ItemExternCrate { ref mut attrs, .. }) => attrs,
            Fn(ItemFn { ref mut attrs, .. }) => attrs,
            ForeignMod(ItemForeignMod { ref mut attrs, .. }) => attrs,
            Impl(ItemImpl { ref mut attrs, .. }) => attrs,
            Macro(ItemMacro { ref mut attrs, .. }) => attrs,
            Macro2(ItemMacro2 { ref mut attrs, .. }) => attrs,
            Mod(ItemMod { ref mut attrs, .. }) => attrs,
            Static(ItemStatic { ref mut attrs, .. }) => attrs,
            Struct(ItemStruct { ref mut attrs, .. }) => attrs,
            Trait(ItemTrait { ref mut attrs, .. }) => attrs,
            TraitAlias(ItemTraitAlias { ref mut attrs, .. }) => attrs,
            Type(ItemType { ref mut attrs, .. }) => attrs,
            Union(ItemUnion { ref mut attrs, .. }) => attrs,
            Use(ItemUse { ref mut attrs, .. }) => attrs,
            other => {
                return Err(Error::new_spanned(
                    other,
                    "this kind of item doesn't have attrs",
                ))
            }
        };
        Ok(attrs)
    }
}

/// Extension for [syn::ItemMod]
pub trait ItemModExt {
    /// Returns reference of content items without braces unless a declaration
    fn items(&self) -> Option<&[Item]>;
    /// Returns reference of content items without braces unless a declaration
    fn items_mut(&mut self) -> Option<&mut Vec<Item>>;
    /// Returns reference of content items without braces unless a declaration
    /// #[deprecated(since="0.1.1", note="Use items() instead")]
    fn unbraced_content(&self) -> Result<&[Item]>;
    /// Returns reference of content items without braces unless a declaration
    /// #[deprecated(since="0.1.1", note="Use items_mut() instead")]
    fn unbraced_content_mut(&mut self) -> Result<&mut Vec<Item>>;
}

impl ItemModExt for ItemMod {
    fn items(&self) -> Option<&[Item]> {
        if let Some((_, content)) = self.content.as_ref() {
            Some(content)
        } else {
            None
        }
    }
    fn items_mut(&mut self) -> Option<&mut Vec<Item>> {
        if let Some((_, content)) = self.content.as_mut() {
            Some(content)
        } else {
            None
        }
    }

    fn unbraced_content(&self) -> Result<&[Item]> {
        self.items()
            .ok_or_else(|| Error::new_spanned(self, "module declaration doesn't have content"))
    }
    fn unbraced_content_mut(&mut self) -> Result<&mut Vec<Item>> {
        if self.content.is_some() {
            Ok(self.items_mut().unwrap())
        } else {
            Err(Error::new_spanned(
                self,
                "module declaration doesn't have content",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_quote_eq;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_attrs() {
        let mut item: Item = parse_quote!(
            #[test]
            type A = u32;
        );
        let expected: Attribute = parse_quote!(#[test]);
        {
            let attr = &item.attrs().unwrap()[0];
            assert_quote_eq!(attr, expected);
        }
        {
            let attr = &item.attrs_mut().unwrap()[0];
            assert_quote_eq!(attr, expected);
        }
    }

    #[test]
    fn test_items() {
        let module: ItemMod = parse_quote!(
            mod m {
                static x: usize = 0;
                fn f() {}
            }
        );
        let content = module.items().unwrap();
        assert!(matches!(content[0], Item::Static(_)));
        assert!(matches!(content[1], Item::Fn(_)));
    }

    #[test]
    fn test_items_decl() {
        let module: ItemMod = parse_quote!(
            mod m;
        );
        assert!(module.items().is_err());
    }
}
