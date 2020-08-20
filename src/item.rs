use syn::{Attribute, Error, Item, ItemMod, Result};

pub trait ItemExt {
    fn attrs(&self) -> Result<&[Attribute]>;
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
            other => return Err(Error::new_spanned(other, "attrs not allowed for this item")),
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
            // verbatim support possible?
            other => return Err(Error::new_spanned(other, "attrs not allowed for this item")),
        };
        Ok(attrs)
    }
}

pub trait ItemModExt {
    fn unbraced_content(&self) -> Result<&[Item]>;
    fn unbraced_content_mut(&mut self) -> Result<&mut Vec<Item>>;
}

impl ItemModExt for ItemMod {
    fn unbraced_content(&self) -> Result<&[Item]> {
        if let Some((_, content)) = self.content.as_ref() {
            Ok(content)
        } else {
            Err(Error::new_spanned(
                self,
                "module declaration doesn't have content",
            ))
        }
    }
    fn unbraced_content_mut(&mut self) -> Result<&mut Vec<Item>> {
        if self.content.is_some() {
            let (_, content) = self.content.as_mut().unwrap();
            Ok(content)
        } else {
            Err(Error::new_spanned(
                self,
                "module declaration doesn't have content",
            ))
        }
    }
}
