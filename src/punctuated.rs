use syn::punctuated::{Pair, Punctuated};

/// Extension for [syn::punctuated::Punctuated]
pub trait PunctuatedExt<T, P> {
    /// Removes and returns the element at position index, popping all elements after it and push them back.
    fn remove(&mut self, index: usize) -> Pair<T, P>
    where
        P: Default;
}

impl<T, P> PunctuatedExt<T, P> for Punctuated<T, P> {
    fn remove(&mut self, index: usize) -> Pair<T, P>
    where
        P: Default,
    {
        let mut stack = Vec::new();
        for _ in index + 1..self.len() {
            stack.push(self.pop().unwrap().into_value());
        }
        let removed = self.pop().unwrap();
        while let Some(item) = stack.pop() {
            self.push(item)
        }
        removed
    }
}

#[cfg(test)]
#[cfg(any(feature = "derive", feature = "full"))]
mod test {
    use super::*;
    use crate::assert_quote_eq;
    use crate::meta::MetaList1;
    use syn::parse_quote;

    #[test]
    fn test_remove() {
        let mut list: MetaList1 = parse_quote!(meta(a, b, c, d));
        list.nested.remove(2);
        let expected: MetaList1 = parse_quote!(meta(a, b, d));
        assert_quote_eq!(list, expected);
        list.nested.remove(0);
        let expected: MetaList1 = parse_quote!(meta(b, d));
        assert_quote_eq!(list, expected);
    }
}
