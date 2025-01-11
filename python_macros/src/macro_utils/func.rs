//lambda args: Body
//Body: Expr ifElse_clauses
// lambda x,y: x + y if x == 0

use crate::macro_utils::comp::{parse_till_end, Condition};
use quote::{quote, ToTokens};
use syn::{custom_keyword, parse::Parse, Expr, Token};

use super::comp::Else;

custom_keyword!(lambda);

pub struct LambdaFunc {
    args: Vec<Arg>,
    body: Body,
}

impl Parse for LambdaFunc {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        _ = input.parse::<lambda>()?;
        let args = parse_till_end(input);
        _ = input.parse::<Token![:]>()?;
        let body = Body::parse(input)?;
        Ok(Self { args, body })
    }
}

impl ToTokens for LambdaFunc {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let args = &self.args;
        let Body { expr, conds } = &self.body;
        let arg_tokens = quote! { #(#args),* };
        let mut if_else_tokens = quote! {#expr};
        for ternary_op in conds {
            let IfElse { cond, else_clause } = &ternary_op;
            if_else_tokens = quote! {
                if #cond {
                    #if_else_tokens
                } else {
                    #else_clause
                }
            }
        }
        tokens.extend(quote! {
            |#arg_tokens| #if_else_tokens
        });
    }
}

struct Arg(Expr);

impl Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let a = input.parse()?;
        if input.peek(Token![,]) {
            _ = input.parse::<Token![,]>()?;
        } else if !input.peek(Token![:]) && !input.is_empty() {
            return Err(input.error("reached end of args"));
        }
        Ok(Self(a))
    }
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}

struct Body {
    expr: Expr,
    conds: Vec<IfElse>,
}

impl Parse for Body {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr = input.parse()?;
        let conds = parse_till_end(input);
        Ok(Self { expr, conds })
    }
}

pub struct IfElse {
    cond: Condition,
    else_clause: Else,
}

impl Parse for IfElse {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let cond = Condition::parse(input)?;
        let else_clause = Else::parse(input)?;
        Ok(Self { cond, else_clause })
    }
}
