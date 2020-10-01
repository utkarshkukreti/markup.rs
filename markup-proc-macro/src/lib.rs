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
    let dynamic = syn::parse_macro_input!(tokens as ast::Dynamic);
    quote::quote!( #dynamic ).into()
}

#[proc_macro]
pub fn to_string(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let to_string = syn::parse_macro_input!(tokens as ast::ToString);
    quote::quote!( #to_string ).into()
}

#[proc_macro]
pub fn to_writer(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let to_writer = syn::parse_macro_input!(tokens as ast::ToWriter);
    quote::quote!( #to_writer ).into()
}
