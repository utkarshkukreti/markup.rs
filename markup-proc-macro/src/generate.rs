use crate::ast::{
    Attribute, Element, For, If, IfClause, IfClauseTest, Match, MatchClause, Node, Struct, Template,
};
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};

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
        children.generate(&mut stream);
        let built = stream.finish();
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
                fn render(&self, __writer: &mut impl std::fmt::Write) -> std::fmt::Result {
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
        let mut stream = Stream::default();
        children.generate(&mut stream);
        let built = stream.finish();
        tokens.extend(quote! {{
            ::markup::new(move |mut __writer| {
                let mut __writer = &mut __writer;
                #built
                Ok(())
            })
        }})
    }
}

trait Generate {
    fn generate(&self, stream: &mut Stream);
}

impl<T: Generate> Generate for Vec<T> {
    fn generate(&self, stream: &mut Stream) {
        for x in self {
            x.generate(stream)
        }
    }
}

impl Generate for Node {
    fn generate(&self, stream: &mut Stream) {
        match self {
            Node::Element(element) => element.generate(stream),
            Node::If(if_) => if_.generate(stream),
            Node::Match(match_) => match_.generate(stream),
            Node::For(for_) => for_.generate(stream),
            Node::Expr(expr) => stream.expr(expr),
            Node::Stmt(stmt) => stream.extend(stmt.into_token_stream()),
        }
    }
}

impl Generate for Element {
    fn generate(&self, stream: &mut Stream) {
        let Element {
            name,
            id,
            classes,
            attributes,
            children,
            close,
        } = self;
        stream.raw("<");
        stream.escaped(name);
        if let Some(id) = id {
            stream.raw(" id=\"");
            stream.expr(id);
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
                stream.expr(class);
            }
            stream.raw("\"");
        }
        for Attribute { name, value } in attributes {
            stream.extend(quote!(let __value = #value;));
            stream.extend(quote!(if ::markup::Render::is_none(&__value) || ::markup::Render::is_false(&__value)));
            stream.braced(|_| {});
            stream.extend(quote!(else if ::markup::Render::is_true(&__value)));
            stream.braced(|stream| {
                stream.raw(" ");
                stream.expr(name);
            });
            stream.extend(quote!(else));
            stream.braced(|stream| {
                stream.raw(" ");
                stream.expr(name);
                stream.raw("=\"");
                stream.expr(&syn::parse_quote!(__value));
                stream.raw("\"");
            });
        }
        stream.raw(">");
        children.generate(stream);
        if *close {
            stream.raw("</");
            stream.escaped(name);
            stream.raw(">");
        }
    }
}

impl Generate for If {
    fn generate(&self, stream: &mut Stream) {
        let mut first = true;
        for clause in &self.clauses {
            let IfClause { test, consequent } = clause;
            if first {
                first = false;
            } else {
                stream.extend(quote!(else));
            }
            match test {
                IfClauseTest::Expr(expr) => stream.extend(quote!(if #expr)),
                IfClauseTest::Let(pattern, expr) => stream.extend(quote!(if let #pattern = #expr)),
            }
            stream.braced(|stream| {
                consequent.generate(stream);
            });
        }
        if let Some(default) = &self.default {
            stream.extend(quote!(else));
            stream.braced(|stream| default.generate(stream))
        }
    }
}

impl Generate for Match {
    fn generate(&self, stream: &mut Stream) {
        let Match { expr, clauses } = &*self;
        stream.extend(quote!(match #expr));
        stream.braced(|stream| {
            for clause in clauses {
                let MatchClause {
                    pat,
                    guard,
                    consequent,
                } = clause;
                stream.extend(quote!(#pat));
                if let Some(guard) = guard {
                    stream.extend(quote!(if #guard));
                }
                stream.extend(quote!(=>));
                stream.braced(|stream| {
                    consequent.generate(stream);
                })
            }
        });
    }
}

impl Generate for For {
    fn generate(&self, stream: &mut Stream) {
        let For { pat, expr, body } = self;
        stream.extend(quote!(for #pat in #expr));
        stream.braced(|stream| body.generate(stream))
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

    fn expr(&mut self, expr: &syn::Expr) {
        match expr {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) => self.escaped(&lit_str.value()),
            _ => self.extend(quote!(::markup::Render::render(&(#expr), __writer)?;)),
        }
    }

    fn extend<Iter: IntoIterator<Item = TokenTree>>(&mut self, iter: Iter) {
        if !self.buffer.is_empty() {
            let buffer = &self.buffer;
            self.stream.extend(quote! {
                __writer.write_str(#buffer)?;
            });
            self.buffer.clear();
        }
        self.stream.extend(iter);
    }

    fn braced(&mut self, f: impl Fn(&mut Stream)) {
        let mut stream = Stream::default();
        f(&mut stream);
        let stream = stream.finish();
        self.stream.extend(quote!({#stream}));
    }

    fn finish(mut self) -> TokenStream {
        self.extend(None);
        self.stream
    }
}
