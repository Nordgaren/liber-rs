#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::TokenStream;

pub fn derive_macro_impl(_input: TokenStream) -> TokenStream {
    //panic!("cargo:warning={input:?}");
    TokenStream::new()
}
