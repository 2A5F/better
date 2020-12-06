mod expr;
mod fns;
mod tem_str;
use expr::*;
use fns::*;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote,format_ident};
use std::ops::Deref;
use syn::*;
use tem_str::*;

#[proc_macro_attribute]
pub fn better(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    map_item(&input).into()
}

fn map_item(item: &Item) -> TokenStream2 {
    match item {
        Item::Fn(f) => map_fn(f),
        _ => return syn::Error::new(Span::call_site(), "Not support").to_compile_error(),
    }
}
