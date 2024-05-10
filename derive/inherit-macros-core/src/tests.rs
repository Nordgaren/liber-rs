#![cfg(test)]

use crate::derive_macro_impl;
use quote::quote;

#[test]
fn test() {
    let after = derive_macro_impl(quote!());
    assert_ne!(after.to_string(), "");
}
