use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn my_macro(_: TokenStream) -> TokenStream {
    let res = quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all=snake_case)]
        enum MyStruct {
            Variant,
        }
    };

    TokenStream::from(res)
}
