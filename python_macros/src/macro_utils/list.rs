// [1, "huhu", 1 + 2, f(5, 6, "huhu")]
// [x := 1, x += 1, 2] what about this? // This should evaluate x := 1 as Expr and return x with x being Rc<RefCell<_>>
// --> vec![Rc<RefCell<1>>, ...]
// --> Vec{
//    Rc<RefCell<Expr>> (where the returned value is saved)
//}

use super::comp::parse_till_end;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Expr, Token};

pub struct List {
    items: Vec<ListItem>,
}

impl Parse for List {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            items: parse_till_end(input),
        })
    }
}

impl ToTokens for List {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let items = &self.items;
        let length = items.len();
        let collection = quote! { {
            let mut temp: Vec<Rc<RefCell<dyn Any>>> = Vec::with_capacity(#length);
            #(
                temp.push(Rc::new(RefCell::new(#items)));
            )*
            temp
            }
        };
        tokens.extend(collection);
    }
}

struct ListItem(Expr);

impl Parse for ListItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr: Expr = input.parse()?;
        if input.peek(Token![,]) {
            _ = input.parse::<Token![,]>()?;
            //return Err(input.error("huhu"));
        }
        Ok(Self(expr))
    }
}

impl ToTokens for ListItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}
