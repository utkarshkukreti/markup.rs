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
pub fn dynamic(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let new = syn::parse_macro_input!(tokens as ast::Dynamic);
    quote::quote!( #new ).into()
}

#[proc_macro]
pub fn to_string(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let template = syn::parse_macro_input!(tokens as ast::ToString);
    quote::quote!( #template ).into()
}

#[proc_macro]
pub fn to_writer(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let template = syn::parse_macro_input!(tokens as ast::ToWriter);
    quote::quote!( #template ).into()
}
