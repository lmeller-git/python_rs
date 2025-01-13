//TODO: standard python list, walrus operator?

mod macro_utils;
use macro_utils::{
    comp::Comprehension,
    func::LambdaFunc,
    list::List,
    scope::{Scoper, Setter},
};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn comp(input: TokenStream) -> TokenStream {
    let c = parse_macro_input!(input as Comprehension);
    quote! {#c}.into()
}

#[proc_macro]
pub fn lambda(input: TokenStream) -> TokenStream {
    let c = parse_macro_input!(input as LambdaFunc);
    quote! {#c}.into()
}

#[proc_macro]
pub fn list(input: TokenStream) -> TokenStream {
    let c = parse_macro_input!(input as List);
    quote! {#c}.into()
}

#[proc_macro]
pub fn set(input: TokenStream) -> TokenStream {
    let c = parse_macro_input!(input as Setter);
    quote! {#c}.into()
}

#[proc_macro]
pub fn scoped(input: TokenStream) -> TokenStream {
    let c = parse_macro_input!(input as Scoper);
    quote! {#c}.into()
}
