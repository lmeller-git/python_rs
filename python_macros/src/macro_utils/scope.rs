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
// or : pref this:
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
//  parse each Expr recursively and build Graph || just build a list
//
//

use proc_macro2::{TokenStream, TokenTree};
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
        //println!("tokens: {}");
        while !input.is_empty() {
            let expr = input.parse()?;
            visitor.visit_expr(&expr);
            for s in visitor.setters.drain(..) {
                setters.push(syn::parse2::<Setter>(s)?.with_context(ParseContext::Decl));
            }
            exprs.push(expr);
            if input.peek(Token![;]) {
                _ = input.parse::<Token![;]>()?;
            }
        }

        Ok(Self { setters, exprs })
    }
}

impl ToTokens for Scoper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Scoper { setters, exprs } = &self;
        let s = if let Some((last, first)) = exprs.split_last() {
            quote! {
                {
                    #(
                        #setters
                    )*
                    #(
                        #first;
                    )*
                    #last
                }
            }
        } else {
            quote! {
                #(
                    #setters
                )*
                #(
                    #exprs;
                )*
            }
        };

        tokens.extend(s);
    }
}

fn parse_set_macro(tokens: TokenStream) -> Vec<TokenStream> {
    let mut tokens_iter = tokens.into_iter();
    let mut setters = Vec::new();

    while let Some(token) = tokens_iter.next() {
        if let TokenTree::Ident(ident) = &token {
            if ident == "set" {
                if let Some(TokenTree::Punct(punct)) = tokens_iter.next() {
                    if punct.as_char() == '!' {
                        if let Some(TokenTree::Group(group)) = tokens_iter.next() {
                            let group_tokens = group.stream();
                            println!("g t: {:?}", group_tokens);
                            setters.append(&mut parse_set_macro(group_tokens));
                            setters.push(group.stream());
                        }
                    }
                }
            }
        }
    }
    setters
}
#[derive(Default)]
struct SetterVisitor {
    setters: Vec<TokenStream>,
}

impl SetterVisitor {
    fn process_set_macro(&mut self, tokens: TokenStream) {
        let mut tokens_iter = tokens.into_iter();

        while let Some(token) = tokens_iter.next() {
            if let TokenTree::Ident(ident) = &token {
                if ident == "set" {
                    if let Some(TokenTree::Punct(punct)) = tokens_iter.next() {
                        if punct.as_char() == '!' {
                            if let Some(TokenTree::Group(group)) = tokens_iter.next() {
                                // Recurse into the group for nested `set` macros
                                let group_tokens = group.stream();
                                self.process_set_macro(group_tokens.clone());
                                self.setters.push(group_tokens);
                            }
                        }
                    }
                }
            } else if let TokenTree::Group(group) = token {
                // Handle other nested groups by recursively visiting them
                self.process_set_macro(group.stream());
            }
        }
    }
}

impl<'ast> Visit<'ast> for SetterVisitor {
    /*
    fn visit_expr_macro(&mut self, i: &'ast syn::ExprMacro) {
        println!("{:?}", i.mac.path.get_ident());
        println!("{:#?}", i.mac.tokens);
        if i.mac.path.is_ident("set") {
            println!("f: {:?}", i.mac.path.get_ident());
            self.setters.push(i.mac.tokens.clone());
        }
        let nested_tokens = i.mac.tokens.clone();
        println!("nested: {:#?}", nested_tokens);
        if let Ok(parsed) = syn::parse2::<syn::Expr>(nested_tokens) {
            println!("parsed");
            self.visit_expr(&parsed);
        }
        visit::visit_expr_macro(self, i);
    }*/
    /*
        fn visit_expr_macro(&mut self, i: &'ast syn::ExprMacro) {
        // current best i guess
            if i.mac.path.is_ident("set") {
                println!("f: {:?}", i.mac.path.get_ident());
                println!("tokens: {:#?}", i.mac.tokens);
                let nested_tokens = i.mac.tokens.clone();
                if let Ok(parsed) = syn::parse2::<syn::Expr>(nested_tokens) {
                    println!("v1");
                    self.visit_expr(&parsed);
                }
                //let mut set_tokens = parse_set_macro(i.mac.tokens.clone());
                //println!("Parsed set macro tokens: {:?}", set_tokens);
                //self.setters.append(&mut set_tokens);
                self.setters.push(i.mac.tokens.clone());
            } else {
                let nested_tokens = i.mac.tokens.clone();
                println!("v2 in : {:#?}", nested_tokens);
                let mut set_tokens = parse_set_macro(i.mac.tokens.clone());
                println!(
                    "Parsed set macro tokens: {:?}, {}",
                    set_tokens,
                    set_tokens.len()
                );
                self.setters.append(&mut set_tokens);
                if let Ok(parsed) = syn::parse2::<syn::Expr>(nested_tokens) {
                    println!("v2");
                    self.visit_expr(&parsed);
                }
            }
            visit::visit_expr_macro(self, i);
        }
    */
    fn visit_expr_macro(&mut self, i: &'ast syn::ExprMacro) {
        if i.mac.path.is_ident("set") {
            // Process the `set` macro tokens
            let nested_tokens = i.mac.tokens.clone();
            self.setters.push(i.mac.tokens.clone());
            self.process_set_macro(nested_tokens);
        } else {
            // Recursively visit other macros or expressions
            let nested_tokens = i.mac.tokens.clone();
            self.process_set_macro(nested_tokens.clone());
            if let Ok(parsed) = syn::parse2::<syn::Expr>(nested_tokens) {
                self.visit_expr(&parsed);
            }
        }
        // Continue visiting macro expressions
        visit::visit_expr_macro(self, i);
    }
    /*
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if mac.path.is_ident("set") {
            println!("Found set macro: {:?}", mac.tokens);
            self.setters.push(mac.tokens.clone());
        }

        // Recursively visit nested macros
        let nested_tokens = mac.tokens.clone();
        if let Ok(parsed) = syn::parse2::<syn::Expr>(nested_tokens) {
            self.visit_expr(&parsed);
        }
    }
    */
    /*
    fn visit_expr_group(&mut self, i: &'ast syn::ExprGroup) {
        let nested_tokens = i.expr.to_token_stream().clone();
        if let Ok(parsed) = syn::parse2::<syn::Expr>(nested_tokens) {
            self.visit_expr(&parsed);
        }
    }*/
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
                    let SetInfo { var, expr: _ } = &i;
                    tokens.extend(quote! {
                        let #var ;//tmp= #expr;
                    });
                }
                Self::SetMut(_, i) => {
                    let SetInfo { var, expr: _ } = &i;
                    tokens.extend(quote! {
                        let mut #var;// = #expr;
                    });
                }
                Self::SetPython(_, i) => {
                    let SetInfo { var, expr: _ } = &i;
                    tokens.extend(quote! {
                        let mut #var; //= Rc::new(RefCell::new(#expr));
                    });
                }
            },
            ParseContext::Expr => {
                //TODO return the bound value

                match self {
                    Self::Set(_, i) | Self::SetMut(_, i) => {
                        let SetInfo { var, expr } = &i;
                        tokens.extend(quote! {
                            {
                                #var = #expr;
                                #var.clone()
                            }
                        });
                    }

                    Self::SetPython(_, i) => {
                        let SetInfo { var, expr } = &i;
                        tokens.extend(quote! {
                            {
                                #var = Rc::new(RefCell::new(#expr));
                                #var.clone()
                            }
                        });
                    }
                }
            }
        }
    }
}
