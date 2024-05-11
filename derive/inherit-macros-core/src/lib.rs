#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::spanned::Spanned;
use syn::{parse2, Data, DeriveInput, Error, Fields};

pub fn inherit_cs_ez_task_impl(input: TokenStream) -> TokenStream {
    inherit_cs_ez_task_internal(input).unwrap_or_else(|e| e.to_compile_error())
}

fn inherit_cs_ez_task_internal(input: TokenStream) -> Result<TokenStream, Error> {
    let input = parse2::<DeriveInput>(input)?;

    let ident = get_structure_name(&input)?;

    is_repr_c(&input)?;
    check_structure(&input)?;

    let tokenstream = inherit_cs_easy_task(ident);
    Ok(tokenstream)
}

fn inherit_cs_easy_task(ident: String) -> TokenStream {
    let class_name = &ident[..ident.len() - 4];
    let class_name_type_ident = format_ident!("{class_name}Type");
    let class_name_ident = format_ident!("{class_name}");
    let vtable_name = format_ident!("{class_name}VTable");
    let mut tokenstream = quote! {
        #[repr(transparent)]
        pub struct #class_name_ident(liber_rs::CppClass<#class_name_type_ident>);

        impl std::ops::Deref for #class_name_ident {
            type Target = liber_rs::CppClass<#class_name_type_ident>;

            fn deref(&self) -> &Self::Target {
                &self.0
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
    tokenstream
}

fn get_structure_name(input: &DeriveInput) -> Result<String, Error> {
    let ident = input.ident.to_string();
    if &ident[ident.len() - 4..] != "Type" {
        return Err(Error::new(
            input.ident.span(),
            "Derive must be used on a type that ends with the word 'Type'",
        ));
    }

    Ok(ident)
}

fn is_repr_c(input: &DeriveInput) -> Result<TokenStream, Error> {
    has_repr_c_attr(input)?;

    // For the future. Need to make compile time checks for all fields of structure being `#[repr(C)]`.
    // Might use bytemuck if I can, but I think that could possibly be broken if I re-export the derive
    // macro or the trait I need.
    Ok(TokenStream::new())
}

fn has_repr_c_attr(input: &DeriveInput) -> Result<(), Error> {
    let attrs = &input.attrs;
    for attr in attrs {
        if attr.path.to_token_stream().to_string().to_lowercase() == "repr" {
            let tokens = attr.tokens.to_string();
            return if tokens.to_uppercase() == "(C)" {
                Ok(())
            } else {
                Err(Error::new(
                    input.span(),
                    format!(
                        "Found `#[repr{}]` attribute. Type must have `#[repr(C)]` attribute",
                        tokens
                    ),
                ))
            }
        }
    }
    Err(Error::new(
        input.span(),
        "Could not find `repr` attribute. Type must have `#[repr(C)]` attribute",
    ))
}

fn check_structure(input: &DeriveInput) -> Result<(), Error> {
    match &input.data {
        Data::Struct(s) => {
            let first = match &s.fields {
                Fields::Named(n) => n.named.first().unwrap(),
                Fields::Unnamed(u) => u.unnamed.first().unwrap(),
                Fields::Unit => {
                    return Err(Error::new(input.span(), "Unit types are not supported."))
                }
            };

            if first.ty.to_token_stream().to_string() != "CSEzTaskType" {
                return Err(Error::new(
                    first.ty.span(),
                    "First field of a class that inherits `CSEzTask` MUST be of type `CSEzTaskType`. Additional fields can go AFTER this field.",
                ));
            }
        }
        _ => {
            return Err(Error::new(
                input.span(),
                "Only structures are supported for inheriting `CSEzTask`.",
            ))
        }
    }

    Ok(())
}
