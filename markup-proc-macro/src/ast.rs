#[derive(Debug)]
pub struct Struct {
    pub name: syn::Ident,
    pub generics: syn::Generics,
    pub where_clause: Option<syn::WhereClause>,
    pub fields: Vec<(syn::Ident, syn::Type)>,
    pub nodes: Vec<Node>,
    pub size_hint: usize,
}

#[derive(Debug)]
pub enum Node {
    Element(Element),
    If(If),
    For(For),
    String(String),
    Expr(syn::Expr),
    Stmt(syn::Stmt),
    Match(Match),
}

#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub id: Option<syn::Expr>,
    pub classes: Vec<syn::Expr>,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
    pub close: bool,
}

#[derive(Debug)]
pub struct If {
    pub clauses: Vec<IfClause>,
    pub default: Option<Vec<Node>>,
}

#[derive(Debug)]
pub struct IfClause {
    pub test: IfClauseTest,
    pub consequent: Vec<Node>,
}

#[derive(Debug)]
pub enum IfClauseTest {
    Expr(syn::Expr),
    Let(syn::Pat, syn::Expr),
}

#[derive(Debug)]
pub struct Match {
    pub expr: syn::Expr,
    pub clauses: Vec<MatchClause>,
}

#[derive(Debug)]
pub struct MatchClause {
    pub pat: syn::Pat,
    pub guard: Option<syn::Expr>,
    pub consequent: Vec<Node>,
}

#[derive(Debug)]
pub struct For {
    pub pat: syn::Pat,
    pub expr: syn::Expr,
    pub body: Vec<Node>,
}

#[derive(Debug)]
pub struct Attribute {
    pub name: syn::Expr,
    pub value: syn::Expr,
    pub bool: bool,
}
