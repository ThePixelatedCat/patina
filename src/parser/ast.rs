pub struct Ast {}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Lit),
    Ident(String),
    FnCall {
        fun: Box<Expr>,
        args: Vec<Expr>,
    },
    BinaryOp {
        op: Bop,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    UnaryOp {
        op: Unop,
        expr: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),
    Bool(bool),
    Array(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bop {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    And,
    Or,
    //Xor,
    Gt,
    Lt,
    Eqq,
    Neq,
    Geq,
    Leq,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unop {
    Not,
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunDef {
    name: String,
    params: Vec<Binding>,
    return_type: Option<String>,
    body: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    mutable: bool,
    name: String,
    type_annotation: Option<String>,
}
