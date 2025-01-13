// helper structs for  macro scope!{} and set!(),
// where scope!{} will expand any code within, such that set!() allows for variable declration within data structures like vec![]
// let l = scoped!{
//    vec![
//    set!(x = 5),
//    x + 5,
//   ...
//]
//};
// -->
// let l = {
//    let x = 5;
//    vec![x, x + 5]
//};
// or :
// let l = {
//  let x;
//  vec![
//    {
//     x = 5;
//     x
//    },
//    x + 5
//  ]
//}

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    custom_keyword,
    parse::Parse,
    visit::{self, Visit},
    Expr, Ident, Token,
};

custom_keyword!(py);

pub struct Scoper {
    setters: Vec<Setter>,
    exprs: Vec<Expr>,
}

impl Parse for Scoper {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        //TODO
        // parse all setters first.
        let mut setters = Vec::new();
        let mut exprs = Vec::new();
        let mut visitor = SetterVisitor::default();
        while !input.is_empty() {
            let expr = input.parse()?;
            visitor.visit_expr(&expr);
            for s in visitor.setters.drain(..) {
                setters.push(syn::parse2::<Setter>(s)?.with_context(ParseContext::Decl));
            }
            exprs.push(expr);
            if input.peek(Token![,]) {
                _ = input.parse::<Token![,]>()?;
            }
        }

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
                #(#exprs)*
            }
        };
        tokens.extend(s);
    }
}

#[derive(Default)]
struct SetterVisitor {
    setters: Vec<TokenStream>,
}

impl<'ast> Visit<'ast> for SetterVisitor {
    fn visit_expr_macro(&mut self, i: &'ast syn::ExprMacro) {
        if i.mac.path.is_ident("set") {
            self.setters.push(i.mac.tokens.clone());
        }
        visit::visit_expr_macro(self, i);
    }
}

#[derive(Default)]
enum ParseContext {
    #[default]
    Expr,
    Decl,
}

struct SetInfo {
    var: Ident,
    expr: Expr,
}

impl ToTokens for SetInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let SetInfo { var, expr } = &self;
        tokens.extend(quote! {#expr})
    }
}

pub enum Setter {
    Set(ParseContext, SetInfo),
    SetMut(ParseContext, SetInfo),
    SetPython(ParseContext, SetInfo),
}

impl Parse for Setter {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        //TODO simplify this parsing to either allow let, or simply var = expr (this would require checking, if the var already exists)
        if !(input.peek(Token![mut]) || input.peek(py) || input.peek(Ident)) {
            Err(input.error("not a set"))
        } else if input.peek(Ident) {
            let var = input.parse()?;
            _ = input.parse::<Token![=]>()?;
            let expr = input.parse()?;
            Ok(Self::Set(ParseContext::default(), SetInfo { var, expr }))
        } else if input.peek(Token![mut]) {
            _ = input.parse::<Token![mut]>()?;
            let var = input.parse()?;
            _ = input.parse::<Token![=]>()?;
            let expr = input.parse()?;
            Ok(Self::SetMut(ParseContext::default(), SetInfo { var, expr }))
        } else if input.peek(py) {
            _ = input.parse::<py>()?;
            let var = input.parse()?;
            _ = input.parse::<Token![=]>()?;
            let expr = input.parse()?;
            Ok(Self::SetPython(
                ParseContext::default(),
                SetInfo { var, expr },
            ))
        } else {
            Err(input.error("wtf"))
        }
    }
}

impl Setter {
    fn with_context(self, ctx: ParseContext) -> Self {
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
            ParseContext::Decl => match self {
                Self::Set(_, i) => {
                    let SetInfo { var, expr } = &i;
                    tokens.extend(quote! {
                        let #var = #expr;
                    });
                }
                Self::SetMut(_, i) => {
                    let SetInfo { var, expr } = &i;
                    tokens.extend(quote! {
                        let mut #var = #expr;
                    });
                }
                Self::SetPython(_, i) => {
                    let SetInfo { var, expr } = &i;
                    tokens.extend(quote! {
                        let mut #var = Rc::new(RefCell::new(#expr));
                    });
                }
            },
            ParseContext::Expr => {
                //TODO return the bound value
                let (Self::Set(_, i) | Self::SetMut(_, i) | Self::SetPython(_, i)) = self;
                let SetInfo { var, expr: _ } = &i;
                tokens.extend(quote! {
                    #var
                });
            }
        }
    }
}
