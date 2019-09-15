#![recursion_limit = "128"]
extern crate proc_macro;

mod ast;
mod caching;
mod generate;
mod parse;

use proc_macro::TokenStream;

#[proc_macro]
pub fn define(tokens: TokenStream) -> TokenStream {
    caching::cached(tokens, |tokens| {
        let structs = syn::parse_macro_input!(tokens as parse::Many<ast::Struct>).0;
        quote::quote!( #(#structs)* ).into()
    })
}
