#![cfg(test)]

use crate::inherit_cs_ez_task_impl;
use quote::quote;

#[test]
fn test() {
    let after = inherit_cs_ez_task_impl(quote!());
    assert_ne!(after.to_string(), "");
}
