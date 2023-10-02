use crate::ast::{
    Attribute, Element, For, If, IfClause, IfClauseTest, Match, MatchClause, Node, Struct, Template,
};
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};
use syn::Ident;

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Struct {
            name,
            attributes,
            generics,
            where_clause,
            fields,
            children,
            size_hint,
        } = self;
        let mut stream = Stream::default();
        let writer = Ident::new("__writer", name.span());
        children.generate(&mut stream, &writer);
        let built = stream.finish(&writer);
        let (impl_generics, ty_generics, _) = generics.split_for_impl();
        let mut struct_fields = TokenStream::new();
        let mut splat_fields = TokenStream::new();
        for field in fields {
            let attrs = &field.attrs;
            let name = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            struct_fields.extend(quote! {
                #(#attrs)*
                pub #name: #ty,
            });
            splat_fields.extend(quote! {
                #name,
            });
        }
        tokens.extend(quote! {
            #(#attributes)*
            pub struct #name #generics #where_clause {
                #struct_fields
            }
            impl #impl_generics #name #ty_generics #where_clause {
                #[inline]
                pub fn to_string(&self) -> String {
                    let mut string = String::with_capacity(#size_hint);
                    // Ignoring the result because writing to a String can't fail.
                    let _ = ::markup::Render::render(self, &mut string);
                    string
                }
            }
            impl #impl_generics ::markup::Render for #name #ty_generics #where_clause {
                fn render(&self, #writer: &mut impl std::fmt::Write) -> std::fmt::Result {
                    let #name { #splat_fields } = self;
                    #built
                    Ok(())
                }
            }
            impl #impl_generics std::fmt::Display for #name #ty_generics #where_clause {
                #[inline]
                fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                    ::markup::Render::render(self, fmt)
                }
            }
        })
    }
}

impl ToTokens for Template {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { children } = self;
        let writer = Ident::new("__writer", Span::call_site());
        let mut stream = Stream::default();
        children.generate(&mut stream, &writer);
        let built = stream.finish(&writer);
        tokens.extend(quote! {{
            ::markup::new(move |mut ___writer| {
                let mut #writer = &mut ___writer;
                #built
                Ok(())
            })
        }})
    }
}

trait Generate {
    fn generate(&self, stream: &mut Stream, writer: &Ident);
}

impl<T: Generate> Generate for Vec<T> {
    fn generate(&self, stream: &mut Stream, writer: &Ident) {
        for x in self {
            x.generate(stream, writer)
        }
    }
}

impl Generate for Node {
    fn generate(&self, stream: &mut Stream, writer: &Ident) {
        match self {
            Node::Element(element) => element.generate(stream, writer),
            Node::If(if_) => if_.generate(stream, writer),
            Node::Match(match_) => match_.generate(stream, writer),
            Node::For(for_) => for_.generate(stream, writer),
            Node::Expr(expr) => stream.expr(expr, writer),
            Node::Stmt(stmt) => stream.extend(stmt.into_token_stream(), writer),
        }
    }
}

impl Generate for Element {
    fn generate(&self, stream: &mut Stream, writer: &Ident) {
        let Element {
            name,
            id,
            classes,
            attributes,
            children,
            close,
        } = self;
        stream.raw("<");
        stream.expr(name, writer);
        if let Some(id) = id {
            stream.raw(" id=\"");
            stream.expr(id, writer);
            stream.raw("\"");
        }
        if !classes.is_empty() {
            stream.raw(" class=\"");
            let mut first = true;
            for class in classes {
                if first {
                    first = false;
                } else {
                    stream.raw(" ");
                }
                stream.expr(class, writer);
            }
            stream.raw("\"");
        }

        fn attr(stream: &mut Stream, name: &syn::Expr, value: &syn::Expr, writer: &Ident) {
            stream.extend(quote!(let __value = #value;), writer);
            stream.extend(
                quote! {
                    if ::markup::RenderAttributeValue::is_none(&__value) ||
                       ::markup::RenderAttributeValue::is_false(&__value)
                },
                writer,
            );
            stream.braced(|_| {}, writer);
            stream.extend(
                quote!(else if ::markup::RenderAttributeValue::is_true(&__value)),
                writer,
            );
            stream.braced(
                |stream| {
                    stream.raw(" ");
                    stream.expr(name, writer);
                },
                writer,
            );
            stream.extend(quote!(else), writer);
            stream.braced(
                |stream| {
                    stream.raw(" ");
                    stream.expr(name, writer);
                    stream.raw("=\"");
                    stream.expr(&syn::parse_quote!(__value), writer);
                    stream.raw("\"");
                },
                writer,
            );
        }

        for attribute in attributes {
            match attribute {
                Attribute::One(name, value) => attr(stream, name, value, writer),
                Attribute::Many(iter) => {
                    stream.extend(quote!(for (__name, __value) in #iter), writer);
                    stream.braced(
                        |stream| {
                            attr(
                                stream,
                                &syn::parse_quote!(__name),
                                &syn::parse_quote!(__value),
                                writer,
                            );
                        },
                        writer,
                    );
                }
            }
        }

        stream.raw(">");

        children.generate(stream, writer);

        if *close {
            stream.raw("</");
            stream.expr(name, writer);
            stream.raw(">");
        }
    }
}

impl Generate for If {
    fn generate(&self, stream: &mut Stream, writer: &Ident) {
        let mut first = true;
        for clause in &self.clauses {
            let IfClause { test, consequent } = clause;
            if first {
                first = false;
            } else {
                stream.extend(quote!(else), writer);
            }
            match test {
                IfClauseTest::Expr(expr) => stream.extend(quote!(if #expr), writer),
                IfClauseTest::Let(pattern, expr) => {
                    stream.extend(quote!(if let #pattern = #expr), writer)
                }
            }
            stream.braced(|stream| consequent.generate(stream, writer), writer);
        }
        if let Some(default) = &self.default {
            stream.extend(quote!(else), writer);
            stream.braced(|stream| default.generate(stream, writer), writer);
        }
    }
}

impl Generate for Match {
    fn generate(&self, stream: &mut Stream, writer: &Ident) {
        let Match { expr, clauses, .. } = self;
        stream.extend(quote!(match #expr), writer);
        stream.braced(
            |stream| {
                for clause in clauses {
                    let MatchClause {
                        pat,
                        guard,
                        consequent,
                    } = clause;
                    stream.extend(quote!(#pat), writer);
                    if let Some(guard) = guard {
                        stream.extend(quote!(if #guard), writer);
                    }
                    stream.extend(quote!(=>), writer);
                    stream.braced(|stream| consequent.generate(stream, writer), writer)
                }
            },
            writer,
        );
    }
}

impl Generate for For {
    fn generate(&self, stream: &mut Stream, writer: &Ident) {
        let For { pat, expr, body } = self;
        stream.extend(quote!(for #pat in #expr), writer);
        stream.braced(|stream| body.generate(stream, writer), writer)
    }
}

#[derive(Default)]
struct Stream {
    stream: TokenStream,
    buffer: String,
}

impl Stream {
    fn raw(&mut self, str: &str) {
        self.buffer.push_str(str);
    }

    fn escaped(&mut self, str: &str) {
        let mut string = String::new();
        crate::escape::escape(str, &mut string).unwrap();
        self.buffer.push_str(&string);
    }

    fn expr(&mut self, expr: &syn::Expr, writer: &Ident) {
        match expr {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) => self.escaped(&lit_str.value()),
            _ => self.extend(
                quote!(::markup::Render::render(&(#expr), #writer)?;),
                writer,
            ),
        }
    }

    fn extend<Iter: IntoIterator<Item = TokenTree>>(&mut self, iter: Iter, writer: &Ident) {
        if !self.buffer.is_empty() {
            let buffer = &self.buffer;
            self.stream.extend(quote! {
                #writer.write_str(#buffer)?;
            });
            self.buffer.clear();
        }
        self.stream.extend(iter);
    }

    fn braced(&mut self, f: impl Fn(&mut Stream), writer: &Ident) {
        let mut stream = Stream::default();
        f(&mut stream);
        let stream = stream.finish(writer);
        self.stream.extend(quote!({#stream}));
    }

    fn finish(mut self, writer: &Ident) -> TokenStream {
        self.extend(None, writer);
        self.stream
    }
}
