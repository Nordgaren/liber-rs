#![doc = include_str!("../README.md")]

use inhert_macros_core::derive_macro_impl;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_derive(CSEzTask)]
pub fn inherit_cs_ez_task(input: TokenStream) -> TokenStream {
    derive_macro_impl(input.into()).into()
}
