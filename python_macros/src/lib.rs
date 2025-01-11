//TODO: standard python list, walrus operator?

mod macro_utils;
use macro_utils::{comp::Comprehension, func::LambdaFunc, list::List, walrus::Walrus};
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let c = parse_macro_input!(input as Comprehension);
    quote! {#c}.into()
}

#[proc_macro]
pub fn lambda(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let c = parse_macro_input!(input as LambdaFunc);
    quote! {#c}.into()
}

#[proc_macro]
pub fn list(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let c = parse_macro_input!(input as List);
    quote! {#c}.into()
}

#[proc_macro]
pub fn walrus(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let c = parse_macro_input!(input as Walrus);
    quote! {#c}.into()
}
