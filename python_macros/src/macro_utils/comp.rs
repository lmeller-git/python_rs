// [expr for pattern in expr if expr]i
// [x for j in source for k in j for i in k for x in i]
// --> expr : x, for_if: j in source if cond
// --> addl: vec![
//    k in j if cond,
//    i in k if cond,
//    x in i if cond
// ]
/*
source.into_iter().flat_map(|j|{
    (true).then(|| j.into_iter().filter_map(|k|{
        (true).then(|| k.into_iter().filter_map(|i| {
            (true).then(|| i.into_iter().filter_map(|x| {
                (true).then(|| x)
            }))
        }).flatten()
    })).flatten()
})
*/

use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Expr, Pat, Token,
};

pub struct Comprehension {
    expr: Expr,
    for_ifs: Vec<ForIf>,
}

impl Parse for Comprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr: input.parse()?,
            for_ifs: parse_till_end(input),
        })
    }
}

impl ToTokens for Comprehension {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let expr = &self.expr;
        let ForIf {
            pattern,
            gen,
            conds,
        } = self
            .for_ifs
            .last()
            .expect("must contain at least one for_if");
        let mut inner = quote! {
            core::iter::IntoIterator::into_iter(#gen).filter_map(move |#pattern| {
                (true #(&& (#conds))*).then(|| #expr)
            })
        };
        for for_if in self.for_ifs.iter().rev().skip(1) {
            let ForIf {
                pattern,
                gen,
                conds,
            } = for_if;
            inner = quote! {
                core::iter::IntoIterator::into_iter(#gen).filter_map(move |#pattern| {
                    (true #(&& (#conds))*).then(|| #inner)
                }).flatten()
            };
        }
        tokens.extend(inner);
    }
}

struct ForIf {
    pattern: Pat,
    gen: Expr,
    conds: Vec<Condition>,
}

impl Parse for ForIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<Token![for]>()?;
        let pattern = Pat::parse_single(input)?;
        _ = input.parse::<Token![in]>()?;
        let gen = input.parse::<Expr>()?;
        let conds = parse_till_end(input);
        Ok(Self {
            pattern,
            gen,
            conds,
        })
    }
}

pub fn parse_till_end<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut res = Vec::new();
    while let Ok(item) = input.parse() {
        res.push(item);
    }
    res
}

pub struct Condition(Expr);

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<Token![if]>()?;
        input.parse().map(Self)
    }
}

impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}

pub struct Else(Expr);

impl Parse for Else {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<Token![else]>()?;
        input.parse().map(Self)
    }
}

impl ToTokens for Else {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}
