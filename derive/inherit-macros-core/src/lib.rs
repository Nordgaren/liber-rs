#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};
use syn::{parse2, DeriveInput, Error};

pub fn inherit_cs_ez_task_impl(input: TokenStream) -> TokenStream {
    let input = match parse2::<DeriveInput>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };

    let ident = input.ident.to_string();
    if &ident[ident.len() - 4..] != "Type" {
        return Error::new(
            input.ident.span(),
            "Derive must be used on a type that ends with the word 'Type'",
        )
            .to_compile_error();
    }



    let class_name = &ident[..ident.len() - 4];
    let class_name_type_ident = format_ident!("{class_name}Type");
    let class_name_ident = format_ident!("{class_name}");
    let vtable_name = format_ident!("{class_name}VTable");
    let mut tokenstream = quote! {
        #[repr(transparent)]
        pub struct #class_name_ident(liber_rs::CppClass<#class_name_type_ident>);

        impl std::ops::Deref for #class_name_ident {
            type Target = liber_rs::from::CS::CSEzTaskType;

            fn deref(&self) -> &Self::Target {
                &self.0.task
            }
        }
    };
    let vtable = quote! {
        pub struct #vtable_name<C: liber_rs::VTable> {
            cs_ez_task_vtable: liber_rs::from::CS::CSEzTaskVTable<C>,
        }
        impl<C: liber_rs::VTable> std::ops::Deref for #vtable_name<C> {
            type Target = liber_rs::from::CS::CSEzTaskVTable<C>;

            fn deref(&self) -> &Self::Target {
                 &self.cs_ez_task_vtable
            }
        }
    };
    let impls = quote! {
        impl #class_name_type_ident {
            pub fn new(task_group: liber_rs::from::CS::CSTaskGroup) -> Self {
                Self { task: CSEzTaskType::new(task_group) }
            }
        }
        impl #class_name_ident {
            pub fn new(task_group: liber_rs::from::CS::CSTaskGroup) -> Self {
                Self(liber_rs::CppClass::<#class_name_type_ident>::new(
                    #class_name_type_ident::new(task_group),
                ))
            }
        }
        impl #vtable_name<#class_name_type_ident> {
            pub const fn new( ) -> Self {
                Self { cs_ez_task_vtable: unsafe { std::mem::transmute(liber_rs::from::CS::CSEzTaskVTable::new()) } }
            }
        }
        impl liber_rs::VTable for #class_name_type_ident {
            type Table = #vtable_name<#class_name_type_ident>;
            const TABLE: &'static Self::Table = &#vtable_name::new();
        }
    };
    let reflection = quote! {
        impl liber_rs::from::FD4::DLRuntimeClass for #class_name_ident {
            extern "C" fn get_runtime_class(&self) -> &'static liber_rs::from::DLRF::DLRuntimeClass {
                static DL_RUNTIME_CLASS: liber_rs::from::DLRF::DLRuntimeClass =
                    liber_rs::from::DLRF::DLRuntimeClass::new(liber_rs::from::DLRF::DLRuntimeClassType::new(
                        liber_rs::cstr!(#class_name),
                        liber_rs::widecstr!(#class_name),
                    ));
                &DL_RUNTIME_CLASS
            }
        }
    };
    tokenstream.append_all(&[vtable, impls, reflection]);
    eprintln!("{}", tokenstream);

    tokenstream
}
