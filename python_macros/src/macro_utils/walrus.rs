// x := 5
// -> let x = 5; within expressions
//TODO
use quote::{quote, ToTokens};
use syn::{parse::Parse, Expr, Ident, Token};

pub struct Walrus {
    var: Ident,
    expr: Expr,
}

impl Parse for Walrus {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let var = input.parse()?;
        _ = input.parse::<Token![=]>()?;
        let expr = input.parse()?;
        Ok(Self { var, expr })
    }
}

impl ToTokens for Walrus {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Walrus { var, expr } = &self;
        tokens.extend(quote! {
            {
                let #var = #expr;
                #var
            }
        })
    }
}
