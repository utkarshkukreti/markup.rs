#![recursion_limit = "128"]
extern crate proc_macro;

mod ast;
mod escape;
mod generate;
mod parse;

#[proc_macro]
pub fn define(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let structs = syn::parse_macro_input!(tokens as parse::Many<ast::Struct>).0;
    quote::quote!( #(#structs)* ).into()
}

#[proc_macro]
pub fn render(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let template = syn::parse_macro_input!(tokens as ast::Template);
    quote::quote!( #template ).into()
}
