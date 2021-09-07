use crate::ident::GetIdent;
use syn::{
    spanned::Spanned, Attribute, Ident, ImplItem, ImplItemMethod, Item, ItemFn, ItemMod, Result,
    TraitItem, TraitItemMethod,
};

/// Extension for [syn::Item]
pub trait ItemLike: Spanned {
    /// Returns reference of inner attrs if not verbatim; otherwise `Err`
    fn attrs(&self) -> Result<&[Attribute]>;
    /// Returns mutable reference of inner attrs if not verbatim; otherwise `Err`
    fn attrs_mut(&mut self) -> Result<&mut Vec<Attribute>>;
    /// Returns function-like trait of Item::Fn, ImplItem::Method or TraitItem::Method
    fn function_or_method(&self) -> Result<&dyn FunctionLike>;

    /// Returns `true` if self matches `*ItemConst`
    fn is_const(&self) -> bool;
    /// Returns `true` if self matches `*ItemType`
    fn is_type(&self) -> bool;
    /// Returns `true` if self matches `*ItemMacro`
    fn is_macro(&self) -> bool;
}

impl ItemLike for Item {
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

    fn function_or_method(&self) -> Result<&dyn FunctionLike> {
        use syn::Item::*;
        use syn::*;
        match self {
            Fn(f @ ItemFn { .. }) => Ok(f),
            other => Err(Error::new_spanned(
                other,
                "this item is not a function or method",
            )),
        }
    }

    fn is_const(&self) -> bool {
        matches!(self, Item::Const(_))
    }
    fn is_type(&self) -> bool {
        matches!(self, Item::Type(_))
    }
    fn is_macro(&self) -> bool {
        matches!(self, Item::Macro(_))
    }
}

impl ItemLike for ImplItem {
    fn attrs(&self) -> Result<&[Attribute]> {
        use syn::ImplItem::*;
        use syn::*;
        let attrs = match self {
            Const(ImplItemConst { ref attrs, .. }) => attrs,
            Method(ImplItemMethod { ref attrs, .. }) => attrs,
            Type(ImplItemType { ref attrs, .. }) => attrs,
            Macro(ImplItemMacro { ref attrs, .. }) => attrs,
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
        use syn::ImplItem::*;
        use syn::*;
        let attrs = match self {
            Const(ImplItemConst { ref mut attrs, .. }) => attrs,
            Method(ImplItemMethod { ref mut attrs, .. }) => attrs,
            Type(ImplItemType { ref mut attrs, .. }) => attrs,
            Macro(ImplItemMacro { ref mut attrs, .. }) => attrs,
            other => {
                return Err(Error::new_spanned(
                    other,
                    "this kind of item doesn't have attrs",
                ))
            }
        };
        Ok(attrs)
    }

    fn function_or_method(&self) -> Result<&dyn FunctionLike> {
        use syn::ImplItem::*;
        use syn::*;
        match self {
            Method(f @ ImplItemMethod { .. }) => Ok(f),
            other => Err(Error::new_spanned(
                other,
                "this item is not a function or method",
            )),
        }
    }

    fn is_const(&self) -> bool {
        matches!(self, ImplItem::Const(_))
    }
    fn is_type(&self) -> bool {
        matches!(self, ImplItem::Type(_))
    }
    fn is_macro(&self) -> bool {
        matches!(self, ImplItem::Macro(_))
    }
}

impl ItemLike for TraitItem {
    fn attrs(&self) -> Result<&[Attribute]> {
        use syn::TraitItem::*;
        use syn::*;
        let attrs = match self {
            Const(TraitItemConst { ref attrs, .. }) => attrs,
            Method(TraitItemMethod { ref attrs, .. }) => attrs,
            Type(TraitItemType { ref attrs, .. }) => attrs,
            Macro(TraitItemMacro { ref attrs, .. }) => attrs,
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
        use syn::TraitItem::*;
        use syn::*;
        let attrs = match self {
            Const(TraitItemConst { ref mut attrs, .. }) => attrs,
            Method(TraitItemMethod { ref mut attrs, .. }) => attrs,
            Type(TraitItemType { ref mut attrs, .. }) => attrs,
            Macro(TraitItemMacro { ref mut attrs, .. }) => attrs,
            other => {
                return Err(Error::new_spanned(
                    other,
                    "this kind of impl item doesn't have attrs",
                ))
            }
        };
        Ok(attrs)
    }

    fn function_or_method(&self) -> Result<&dyn FunctionLike> {
        use syn::TraitItem::*;
        use syn::*;
        match self {
            Method(f @ TraitItemMethod { .. }) => Ok(f),
            other => Err(Error::new_spanned(
                other,
                "this item is not a function or method",
            )),
        }
    }

    fn is_const(&self) -> bool {
        matches!(self, TraitItem::Const(_))
    }
    fn is_type(&self) -> bool {
        matches!(self, TraitItem::Type(_))
    }
    fn is_macro(&self) -> bool {
        matches!(self, TraitItem::Macro(_))
    }
}

/// Extension for `syn::*Item::attrs` using `crate::ext::ItemLike`
pub trait ItemAttrExt: ItemLike {
    /// Takes a closure and calls it with separated attrs and item, as both mutable references.
    ///
    /// 1. Try to get attrs; Otherwise `Err`
    /// 2. Split `attrs` from `self` with [std::mem::replace]
    /// 3. Call the closure `f`
    /// 4. Merge `attrs` into `self`
    ///
    /// Note: During the closure call, `attrs` in `self` is always an empty.
    /// Always access `attrs` with given closure parameter.
    ///
    /// # Panics
    /// Panics if replaced `attrs` in `self` is not empty at merge step.
    fn try_split_attr_mut<F, R>(&mut self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Vec<Attribute>, &mut Self) -> Result<R>,
    {
        let mut attrs = std::mem::take(self.attrs_mut()?);
        let result = f(&mut attrs, self);
        let _temp = std::mem::replace(self.attrs_mut().unwrap(), attrs);
        assert!(
            _temp.is_empty(),
            "attrs changed during replacement. this behavior must be a bug."
        );
        result
    }
}

impl ItemAttrExt for Item {}
impl ItemAttrExt for ImplItem {}
impl ItemAttrExt for TraitItem {}

/// Extension for [syn::ItemMod]
pub trait ItemModExt {
    /// Returns reference of content items without braces unless a declaration
    fn items(&self) -> Option<&[Item]>;
    /// Returns reference of content items without braces unless a declaration
    fn items_mut(&mut self) -> Option<&mut Vec<Item>>;
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
}

impl GetIdent for Item {
    fn get_ident(&self) -> Option<&Ident> {
        use syn::Item::*;
        use syn::UseTree::*;
        use syn::*;
        #[allow(clippy::collapsible_match)]
        let attrs = match self {
            Const(ItemConst { ref ident, .. }) => ident,
            Enum(ItemEnum { ref ident, .. }) => ident,
            ExternCrate(ItemExternCrate { ref ident, .. }) => ident,
            Fn(ItemFn { sig, .. }) => &sig.ident,
            Impl(ItemImpl { .. }) => unimplemented!(),
            Macro(ItemMacro { ref ident, .. }) => return ident.as_ref(),
            Macro2(ItemMacro2 { ref ident, .. }) => ident,
            Mod(ItemMod { ref ident, .. }) => ident,
            Static(ItemStatic { ref ident, .. }) => ident,
            Struct(ItemStruct { ref ident, .. }) => ident,
            Trait(ItemTrait { ref ident, .. }) => ident,
            TraitAlias(ItemTraitAlias { ref ident, .. }) => ident,
            Type(ItemType { ref ident, .. }) => ident,
            Union(ItemUnion { ref ident, .. }) => ident,
            Use(ItemUse { ref tree, .. }) => match tree {
                Name(UseName { ident }) => ident,
                _ => return None,
            },
            _ => return None,
        };
        Some(attrs)
    }
}

impl GetIdent for ImplItem {
    fn get_ident(&self) -> Option<&Ident> {
        use syn::ImplItem::*;
        use syn::*;
        let ident = match self {
            Const(ImplItemConst { ref ident, .. }) => ident,
            Method(ImplItemMethod { sig, .. }) => &sig.ident,
            Type(ImplItemType { ref ident, .. }) => ident,
            Macro(ImplItemMacro {
                mac: syn::Macro { path, .. },
                ..
            }) => return path.get_ident(),
            _ => return None,
        };
        Some(ident)
    }
}

impl GetIdent for TraitItem {
    fn get_ident(&self) -> Option<&Ident> {
        use syn::TraitItem::*;
        use syn::*;
        let ident = match self {
            Const(TraitItemConst { ref ident, .. }) => ident,
            Method(TraitItemMethod { sig, .. }) => &sig.ident,
            Type(TraitItemType { ref ident, .. }) => ident,
            Macro(TraitItemMacro {
                mac: syn::Macro { path, .. },
                ..
            }) => return path.get_ident(),
            _ => return None,
        };
        Some(ident)
    }
}

/// Extension for [syn::ItemFn] and [syn::ImplItemMethod]
pub trait FunctionLike: Spanned {
    /// Returns reference of attrs
    fn attrs(&self) -> &[Attribute];
    /// Returns mutable reference of attrs
    fn attrs_mut(&mut self) -> &mut Vec<Attribute>;
    /// Return reference of vis
    fn vis(&self) -> &syn::Visibility;

    fn sig(&self) -> &syn::Signature;
    fn block(&self) -> Option<&syn::Block>;
}

impl FunctionLike for ItemFn {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
    fn attrs_mut(&mut self) -> &mut Vec<Attribute> {
        &mut self.attrs
    }
    fn vis(&self) -> &syn::Visibility {
        &self.vis
    }
    fn sig(&self) -> &syn::Signature {
        &self.sig
    }
    fn block(&self) -> Option<&syn::Block> {
        Some(&self.block)
    }
}

impl FunctionLike for ImplItemMethod {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
    fn attrs_mut(&mut self) -> &mut Vec<Attribute> {
        &mut self.attrs
    }
    fn vis(&self) -> &syn::Visibility {
        &self.vis
    }
    fn sig(&self) -> &syn::Signature {
        &self.sig
    }
    fn block(&self) -> Option<&syn::Block> {
        Some(&self.block)
    }
}

impl FunctionLike for TraitItemMethod {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
    fn attrs_mut(&mut self) -> &mut Vec<Attribute> {
        &mut self.attrs
    }
    fn vis(&self) -> &syn::Visibility {
        &syn::Visibility::Inherited
    }
    fn sig(&self) -> &syn::Signature {
        &self.sig
    }
    fn block(&self) -> Option<&syn::Block> {
        self.default.as_ref()
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
        assert!(module.items().is_none());
    }

    #[test]
    fn test_function_like() {
        let function: ItemFn = parse_quote!(
            fn f(a: u8) -> Result<()> {}
        );
        let method: ImplItemMethod = parse_quote!(
            fn f(a: u8) -> Result<()> {}
        );
        assert_eq!(quote!(#function).to_string(), quote!(#method).to_string());
    }
}
