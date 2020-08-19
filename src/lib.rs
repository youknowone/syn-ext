mod attribute;
mod meta;
mod punctuated;

pub use crate::attribute::AttributeExt;
pub use crate::meta::MetaExt;
pub use crate::punctuated::PunctuatedExt;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
