// helper structs for  macro scope!{} and walrus!(),
// where scope!{} will expand any code within, such that walrus!() allows for variable declration within data structures like vec![]
// let l = scoped!{
//    vec![
//    walrus!(x = 5),
//    x + 5,
//   ...
//]
//};
// -->
// let l = {
//    let x = 5;
//    vec![x, x + 5]
//};

use quote::{quote, ToTokens};
use syn::{custom_keyword, parse::Parse, Expr, Ident, Token};

use super::comp::parse_till_end;

custom_keyword!(set);
custom_keyword!(set_mut);
custom_keyword!(set_py);

pub struct Scoper {
    setters: Vec<Setter>,
    exprs: Vec<Expr>,
}

impl Parse for Scoper {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse all setters first.
        let mut setters = Vec::new();
        let fork = input.fork();
        while !fork.is_empty() {
            if let Ok(s) = Setter::parse(&fork) {
                setters.push(s.with_context(ParseContext::Decl));
            }
            if fork.peek(Token![,]) {
                _ = fork.parse::<Token![,]>()?;
            }
        }

        let exprs = parse_till_end(input);

        Ok(Self { setters, exprs })
    }
}

impl ToTokens for Scoper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Scoper { setters, exprs } = &self;
        let s = quote! {
            {
                #(
                    #setters;
                )*
                #(
                    #exprs;
                )*
            }
        };
        tokens.extend(s);
    }
}

#[derive(Default)]
enum ParseContext {
    #[default]
    Expr,
    Decl,
}

struct SetInfo {
    ident: Ident,
    val: Expr,
}

enum Setter {
    Set(ParseContext, SetInfo),
    SetMut(ParseContext, SetInfo),
    SetPython(ParseContext, SetInfo),
}

impl Parse for Setter {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

impl Setter {
    fn with_context(mut self, ctx: ParseContext) -> Self {
        match self {
            Self::Set(_ctx, i) => Self::Set(ctx, i),
            Self::SetMut(_ctx, i) => Self::SetMut(ctx, i),
            Self::SetPython(_ctx, i) => Self::SetPython(ctx, i),
        }
    }
}

impl ToTokens for Setter {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let (Self::Set(ctx, _) | Self::SetMut(ctx, _) | Self::SetPython(ctx, _)) = self;
        match ctx {
            ParseContext::Expr => match self {
                Self::Set(_, i) => {
                    let SetInfo { ident, val } = &i;
                    tokens.extend(quote! {
                        set!(#ident = #val);
                    });
                }
                Self::SetMut(_, i) => {
                    let SetInfo { ident, val } = &i;
                    tokens.extend(quote! {
                        set_mut!(#ident = #val);
                    });
                }
                Self::SetPython(_, i) => {
                    let SetInfo { ident, val } = &i;
                    tokens.extend(quote! {
                        set_py!(#ident = #val);
                    });
                }
            },
            ParseContext::Decl => {
                let (Self::Set(_, i) | Self::SetMut(_, i) | Self::SetPython(_, i)) = self;
                let SetInfo { ident, val: _ } = &i;
                tokens.extend(quote! {#ident});
            }
        }
    }
}
