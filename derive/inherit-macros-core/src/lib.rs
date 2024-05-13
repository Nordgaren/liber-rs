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
    let fields = check_and_get_fields(&input)?;

    let tokenstream = inherit_cs_easy_task(ident, fields);
    Ok(tokenstream)
}

fn inherit_cs_easy_task(ident: String, fields: &Fields) -> TokenStream {
    let field_list = fields.to_token_stream();
    eprintln!("{field_list}");

    let class_name = &ident[..ident.len() - 4];
    let class_name_type_ident = format_ident!("{class_name}Type");
    let class_name_ident = format_ident!("{class_name}");
    let vtable_name = format_ident!("{class_name}VTable");
    let mut tokenstream = quote! {
         impl std::ops::Deref for #class_name_type_ident {
            type Target = liber_rs::from::CS::CSEzTaskType;

            fn deref(&self) -> &Self::Target {
                &self.task
            }
        }

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
        pub struct #vtable_name {
            get_runtime_class: extern "C" fn(&#class_name_ident) -> &'static liber_rs::from::DLRF::DLRuntimeClass,
            destructor: extern "C" fn(&#class_name_ident),
            execute: extern "C" fn(&#class_name_ident, &FD4TaskData),
            eztask_execute: extern "C" fn(&#class_name_ident, &FD4TaskData),
            register_task: extern "C" fn(&#class_name_ident, task_group: CSTaskGroup),
            free_task: extern "C" fn(&#class_name_ident),
        }
        // impl<C: liber_rs::VTable> std::ops::Deref for #vtable_name<C> {
        //     type Target = liber_rs::from::CS::CSEzTaskVTable<C>;
        //
        //     fn deref(&self) -> &Self::Target {
        //          &self.cs_ez_task_vtable
        //     }
        // }
    };
    let impls = quote! {
        impl #class_name_ident {
            pub fn new() -> Self {
                let task_type = liber_rs::from::CS::CSEzTaskType::new(std::ptr::null_mut());
                Self(liber_rs::CppClass::<
                    #class_name_type_ident,
                >::from_data(#class_name_type_ident { task: task_type }))
            }
        }
        impl #vtable_name {
            pub const fn new() -> Self {
                unsafe {
                    Self {
                        get_runtime_class: <#class_name_ident as liber_rs::from::FD4::DLRuntimeClassTrait>::get_runtime_class,
                        destructor: <#class_name_ident as liber_rs::from::FD4::FD4ComponentBaseTrait>::destructor,
                        execute: <#class_name_ident as liber_rs::from::FD4::FD4TaskBaseTrait>::execute,
                        eztask_execute: <#class_name_ident as liber_rs::from::CS::CSEzTaskTrait>::eztask_execute,
                        register_task: <#class_name_ident as liber_rs::from::CS::CSEzTaskTrait>::register_task,
                        free_task: <#class_name_ident as liber_rs::from::CS::CSEzTaskTrait>::free_task,
                    }
                }
            }
        }
        impl liber_rs::VTable for #class_name_type_ident {
            type Table = #vtable_name;
            const TABLE: &'static Self::Table = &#vtable_name::new();
        }
    };
    let reflection = quote! {
        impl liber_rs::from::FD4::DLRuntimeClassTrait for #class_name_ident {
            extern "C" fn get_runtime_class(&self) -> &'static liber_rs::from::DLRF::DLRuntimeClass {
                static DL_RUNTIME_CLASS: liber_rs::from::DLRF::DLRuntimeClass =
                    liber_rs::from::DLRF::DLRuntimeClass::from_data(liber_rs::from::DLRF::DLRuntimeClassType::new(
                        liber_rs::cstr!(#class_name),
                        liber_rs::widecstr!(#class_name),
                    ));
                &DL_RUNTIME_CLASS
            }
        }
    };
    let checks = quote! {
        const _:() = {
            const fn is_cs_ez_task(t: &dyn liber_rs::from::CS::CSEzTaskTrait) {}
            const fn new_test_type() -> #class_name_ident {
                unsafe { core::mem::MaybeUninit::uninit().assume_init() }
            }
            let t = new_test_type();
            is_cs_ez_task(&t);
            std::mem::forget(t);
        };
    };

    tokenstream.append_all(&[vtable, impls, reflection, checks]);
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
            };
        }
    }
    Err(Error::new(
        input.span(),
        "Could not find `repr` attribute. Type must have `#[repr(C)]` attribute",
    ))
}

fn check_and_get_fields(input: &DeriveInput) -> Result<&Fields, Error> {
    let fields = match &input.data {
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

            &s.fields
        }
        _ => {
            return Err(Error::new(
                input.span(),
                "Only structures are supported for inheriting `CSEzTask`.",
            ))
        }
    };

    Ok(fields)
}
