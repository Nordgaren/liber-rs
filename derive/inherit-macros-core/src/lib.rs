#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::collections::HashMap;
use std::str::FromStr;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    parse2, AttrStyle, Attribute, Data, DeriveInput, Error, Field, Fields, FieldsUnnamed,
    ItemStruct, LitStr, Path, PathSegment, Type, Visibility,
};

// Attr Macro
/// The only function outside `CSEzTaskTrait` functions that can be overwritten. `FD4TaskBaseTrait::execute`
/// is overridden by `CSEzTask` and declared `final`, so, it cannot be overridden by the inheritor.
/// The user has to implement `CSEzTaskTrait` themselves, as they have to implement `eztask_execute`.
#[derive(Debug)]
struct AttrArgs {
    destructor: Option<String>,
}

pub fn inherit_cs_ez_task_attr_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    inherit_cs_ez_task_attr_internal(attr, item).unwrap_or_else(|e| e.to_compile_error())
}

fn inherit_cs_ez_task_attr_internal(
    attr: TokenStream,
    item: TokenStream,
) -> Result<TokenStream, Error> {
    let mut input = parse2::<ItemStruct>(item)?;
    let checks = enforce_repr_c(&mut input)?;
    let fields = enforce_first_field(&mut input)?;
    let params = get_params(fields);

    let ident = input.ident;
    input.ident = format_ident!("{}Type", ident);

    let args = parse_args(attr, ident)?;
    let inherited = inherit_cs_easy_task(input.ident.to_string(), params, args);
    let mut tokenstream = input.to_token_stream();
    tokenstream.append_all([inherited, checks]);
    Ok(tokenstream)
}

fn parse_args(attr: TokenStream, ident: Ident) -> Result<AttrArgs, Error> {
    let attr = attr.to_string();
    let args = attr.split(',');
    let mut dict = HashMap::new();
    for arg in args {
        let mut split = arg.split('=');
        let func = split.next().unwrap().trim();
        let result = dict.insert(
            func,
            split
                .next()
                .unwrap_or(&format!("{ident}::{func}"))
                .trim()
                .to_string(),
        );
        if result.is_some() {
            return Err(Error::new(
                syn::spanned::Spanned::span(&attr),
                format!("{} specified twice. Can only be specified once!", func),
            ));
        }
    }

    let args = AttrArgs {
        destructor: dict.remove("destructor"),
    };
    eprintln!("{args:?}");
    Ok(args)
}

fn enforce_repr_c(input: &mut ItemStruct) -> Result<TokenStream, Error> {
    enforce_repr_c_attr(input)?;

    // For the future. Need to make compile time checks for all fields of structure being `#[repr(C)]`.
    // Might use bytemuck if I can, but I think that could possibly be broken if I re-export the derive
    // macro or the trait I need.
    Ok(TokenStream::new())
}

fn enforce_repr_c_attr(input: &mut ItemStruct) -> Result<(), Error> {
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
    input.attrs.push(Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        path: Path {
            leading_colon: None,
            segments: Punctuated::from_iter([PathSegment {
                ident: format_ident!("repr"),
                arguments: Default::default(),
            }]),
        },
        tokens: quote!((C)),
    });

    Ok(())
}

fn enforce_first_field(input: &mut ItemStruct) -> Result<&Fields, Error> {
    fn check_first_field(first: &Field) -> bool {
        first
            .ty
            .to_token_stream()
            .to_string()
            .ends_with("CSEzTaskType")
    }

    match &mut input.fields {
        Fields::Named(n) => {
            if !check_first_field(n.named.first().unwrap()) {
                n.named.insert(
                    0,
                    Field {
                        attrs: vec![],
                        vis: Visibility::Inherited,
                        ident: Some(format_ident!("task")),
                        colon_token: None,
                        ty: Type::Verbatim(quote!(liber_rs::from::CS::CSEzTaskType)),
                    },
                )
            }
        }
        Fields::Unnamed(u) => {
            if !check_first_field(u.unnamed.first().unwrap()) {
                u.unnamed.insert(
                    0,
                    Field {
                        attrs: vec![],
                        vis: Visibility::Inherited,
                        ident: None,
                        colon_token: None,
                        ty: Type::Verbatim(quote!(liber_rs::from::CS::CSEzTaskType)),
                    },
                )
            }
        }
        Fields::Unit => {
            let mut fields = FieldsUnnamed {
                paren_token: Default::default(),
                unnamed: Default::default(),
            };
            fields.unnamed.push_value(Field {
                attrs: vec![],
                vis: Visibility::Inherited,
                ident: None,
                colon_token: None,
                ty: Type::Verbatim(quote!(liber_rs::from::CS::CSEzTaskType)),
            });
            input.fields = Fields::Unnamed(fields);
        }
    };

    Ok(&input.fields)
}

// Derive Macro
struct Params {
    names: TokenStream,
    field_types: TokenStream,
    task_field_name: Option<Ident>,
}

pub fn inherit_cs_ez_task_impl(input: TokenStream) -> TokenStream {
    inherit_cs_ez_task_internal(input).unwrap_or_else(|e| e.to_compile_error())
}

fn inherit_cs_ez_task_internal(input: TokenStream) -> Result<TokenStream, Error> {
    let input = parse2::<DeriveInput>(input)?;

    let ident = get_structure_name(&input)?;

    let checks = is_repr_c(&input)?;
    let fields = check_and_get_fields(&input)?;

    let params = get_params(&fields);

    let mut tokenstream = inherit_cs_easy_task(ident, params, AttrArgs { destructor: None });
    tokenstream.append_all(checks);
    Ok(tokenstream)
}

fn inherit_cs_easy_task(ident: String, fields: Params, args: AttrArgs) -> TokenStream {
    let Params {
        names,
        field_types,
        task_field_name,
    } = fields;

    let class_name = &ident[..ident.len() - 4];
    let class_name_type_ident = format_ident!("{class_name}Type");
    let class_name_ident = format_ident!("{class_name}");
    let vtable_name = format_ident!("{class_name}VTable");
    let struct_type_specific = match task_field_name {
        None => quote! {
            impl #class_name_ident {
                pub fn new(#field_types) -> Self {
                    let task = liber_rs::from::CS::CSEzTaskType::new();
                    Self(liber_rs::CppClass::<
                        #class_name_type_ident,
                    >::from_data(#class_name_type_ident(task, #names)))
                }
            }
            impl std::ops::Deref for #class_name_type_ident {
                type Target = liber_rs::from::CS::CSEzTaskType;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        },
        Some(task_field_name) => quote! {
            impl #class_name_ident {
                pub fn new(#field_types) -> Self {
                    let #task_field_name = liber_rs::from::CS::CSEzTaskType::new();
                    Self(liber_rs::CppClass::<
                        #class_name_type_ident,
                    >::from_data(#class_name_type_ident { #task_field_name, #names }))
                }
            }
            impl std::ops::Deref for #class_name_type_ident {
                type Target = liber_rs::from::CS::CSEzTaskType;

                fn deref(&self) -> &Self::Target {
                    &self.#task_field_name
                }
            }
        },
    };

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
        #[repr(C)]
        pub struct #vtable_name {
            get_runtime_class: extern "C" fn(&#class_name_ident) -> &'static liber_rs::from::DLRF::DLRuntimeClass,
            destructor: extern "C" fn(&#class_name_ident),
            execute: extern "C" fn(&#class_name_ident, &liber_rs::from::FD4::FD4TaskData),
            eztask_execute: extern "C" fn(&#class_name_ident, &liber_rs::from::FD4::FD4TaskData),
            register_task: extern "C" fn(&#class_name_ident, task_group: liber_rs::from::CS::CSTaskGroup),
            free_task: extern "C" fn(&#class_name_ident),
        }
    };

    let fd4_component_impl = match args.destructor {
        Some(d) => {
            let str = LitStr::new(&d, Span::call_site());
            let destructor = str.parse_with(Path::parse_mod_style).unwrap();
            quote! {
                impl FD4ComponentBaseTrait for #class_name_ident {
                    extern "C" fn destructor(&self) {
                        #destructor(self);
                        unsafe { liber_rs::from::CS::CSEzTask::destructor(&*(self as *const #class_name_ident as *const liber_rs::from::CS::CSEzTask)) };
                    }
                }
            }
        }
        None => quote! {
            impl liber_rs::from::FD4::FD4ComponentBaseTrait for #class_name_ident {}
        },
    };

    let impls = quote! {
        impl #vtable_name {
            pub const fn new() -> Self {
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
        impl liber_rs::VTable for #class_name_type_ident {
            type Table = #vtable_name;
            const TABLE: &'static Self::Table = &#vtable_name::new();
        }
        impl liber_rs::from::FD4::FD4TaskBaseTrait for #class_name_ident {
            extern "C" fn execute(&self, data: &FD4TaskData) {
                self.eztask_execute(data)
            }
        }
        #fd4_component_impl
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

    tokenstream.append_all(&[struct_type_specific, vtable, impls, reflection, checks]);
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

fn check_and_get_fields(input: &DeriveInput) -> Result<Fields, Error> {
    let fields = match &input.data {
        Data::Struct(s) => {
            let first = match &s.fields {
                Fields::Named(n) => n.named.first().unwrap(),
                Fields::Unnamed(u) => u.unnamed.first().unwrap(),
                Fields::Unit => {
                    return Err(Error::new(input.span(), "Unit types are not supported."));
                }
            };

            if !first
                .ty
                .to_token_stream()
                .to_string()
                .ends_with("CSEzTaskType")
            {
                return Err(Error::new(
                    first.ty.span(),
                    "First field of a class that inherits `CSEzTask` MUST be of type `CSEzTaskType`. Additional fields can go AFTER this field.",
                ));
            }

            s.fields.clone()
        }
        _ => {
            return Err(Error::new(
                input.span(),
                "Only structures are supported for inheriting `CSEzTask`.",
            ));
        }
    };

    Ok(fields)
}

fn get_params(fields: &Fields) -> Params {
    match fields {
        Fields::Named(n) => {
            let mut fields_iter = n.named.iter();
            let task_field_name = fields_iter.next().unwrap().clone().ident.unwrap();
            let mut fields: Punctuated<Field, Comma> = Punctuated::new();
            let mut names = vec![];

            for field in fields_iter {
                fields.push(field.clone());
                names.push(field.ident.as_ref().unwrap().to_string());
            }

            Params {
                field_types: TokenStream::from_str(&fields.to_token_stream().to_string()).unwrap(),
                names: TokenStream::from_str(&names.join(", ")).unwrap(),
                task_field_name: Some(task_field_name),
            }
        }
        Fields::Unnamed(u) => {
            let mut fields_iter = u.unnamed.iter();
            fields_iter.next();
            let mut fields: Punctuated<Field, Comma> = Punctuated::new();
            let mut names = vec![];

            let count = 1;
            for field in fields_iter {
                let field_name = format!("f{count}");
                let mut new_field = field.clone();
                new_field.ident = Some(format_ident!("{field_name}"));
                fields.push(new_field);
                names.push(field_name);
            }

            Params {
                field_types: TokenStream::from_str(&fields.to_token_stream().to_string()).unwrap(),
                names: TokenStream::from_str(&names.join(", ")).unwrap(),
                task_field_name: None,
            }
        }
        Fields::Unit => unreachable!(),
    }
}
