use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use quote::quote;
use syn::{
    ItemFn,
    parse_macro_input,
};


#[proc_macro_attribute]
pub fn rstest(attr: TokenStream, item: TokenStream) -> TokenStream {
    // the "full" rstest syntax looks like
    //
    // #[rstest]
    // #[case(...)]
    // #[case(...)]
    // #[tokio::test]
    // async fn test_blah(#[case] ...)
    //
    // We want to support a similar usage for test_log, so with this library the syntax looks like
    //
    // #[rstest(tokio::test)]
    // #[case(...)]
    // #[case(...)]
    // async fn test_blah(#[case] ...)
    //
    // This needs to be desugared into the following:
    //
    // #[rstest]
    // #[case(...)]
    // #[case(...)]
    // #[test_log::test]
    // #[tokio::test]

    let ItemFn { attrs, vis, sig, block } = parse_macro_input!(item as ItemFn);
    let internal_attr = if attr.is_empty() {
        quote! {}
    } else {
        let attr_tokens: Tokens = attr.into();
        quote! { #[ #attr_tokens ] }
    };
    quote! {
        #[ ::test_log::test(::rstest::rstest) ]
        #(#attrs)*
        #internal_attr
        #vis #sig #block
    }
    .into()
}
