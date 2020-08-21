#[macro_export]
macro_rules! assert_quote_eq {
    ($q1:expr, $q2:expr) => {{
        let v1 = &$q1;
        let v2 = &$q2;
        assert_eq!(quote! { #v1 }.to_string(), quote! { #v2 }.to_string());
    }};
}
