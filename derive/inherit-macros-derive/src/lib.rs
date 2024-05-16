#![doc = include_str!("../README.md")]

use inherit_macros_core::{inherit_cs_ez_task_attr_impl, inherit_cs_ez_task_impl};
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

/// Automatically implements most of what the user needs to "inherit" the `CSEzTask` C++ class. The structure
/// inheriting this class must be `#[repr(C)]`, end with the name `Type`, and the first field in the structure
/// must be of type `CSEzTaskType`. This will generate a wrapper class of the same name as the type being
/// derived, minus the `Type` ending. Example: `MyCustomTaskType` will generate a new object called
/// `MyCustomTask`. It will also generate a `VTable` type for this class with the name `VTable` at the
/// end, instead of `Type`.
///
/// # Generated
/// A list of code generated for your inheritance.
///
/// > Newtype class that wraps `CppClass<C>` with `C` being your custom type.
/// > Structure for `VTable` implementation of your custom type with the same name as your type, but
/// `VTable` instead of `Type`, as well as the associated implementation of the `VTable` for your type.
/// > `Deref` implementations that deref the newtype into the underlying `CppClass<C>` as well as a
/// a implementation that derefs your custom type into `CSEzTaskType`. This is to mimic the C++ inheritance.
/// api.
/// > A function called `new` for both your custom type and generated newtype which take in all fields
/// in the user defined structure after the CSEzTaskType. This does not support user defined newtypes/tuple
/// types with additional fields (beyond the required CSEzTaskType).
/// > An implementation for the `DLRuntimeClassTrait` trait and a `DLRuntimeClass` for your class.
/// > A compile time check to enforce that your class implements `CSEzTaskTrait`
#[proc_macro_error]
#[proc_macro_derive(CSEzTask)]
pub fn inherit_cs_ez_task(input: TokenStream) -> TokenStream {
    inherit_cs_ez_task_impl(input.into()).into()
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn cs_ez_task(attr: TokenStream, item: TokenStream) -> TokenStream {
    inherit_cs_ez_task_attr_impl(attr.into(), item.into()).into()
}
