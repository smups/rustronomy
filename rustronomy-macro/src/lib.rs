extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(MetaDataContainer)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
