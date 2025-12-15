pub(crate) struct Ast {}

pub(crate) struct FunDef {
    name: String,
    params: Vec<Binding>,
    return_type: Option<String>,
    body: Expr,
}

pub(crate) struct Binding {
    mutable: bool,
    name: String,
    type_annotation: Option<String>,
}

pub(crate) enum Expr {
    Literal(Lit),
    Ident(String),
    Binop(Bop, Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Let(Binding, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Block(Vec<Expr>),
}

pub(crate) enum Lit {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
}

pub(crate) enum Bop {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    Gt,
    Lt,
    Eqq,
    Geq,
    Leq,
}
