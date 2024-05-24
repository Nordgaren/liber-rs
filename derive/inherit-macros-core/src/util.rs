use quote::ToTokens;
use syn::Field;

pub fn check_field_name(first: &Field, name: &str) -> bool {
    first
        .ty
        .to_token_stream()
        .to_string()
        .ends_with(name)
}