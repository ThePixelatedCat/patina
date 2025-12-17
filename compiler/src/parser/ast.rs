pub struct Ast {}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        mutable: bool,
        ident: String,
        type_annotation: Option<String>,
        value: Expr,
    },
    Assign {
        ident: String,
        value: Expr,
    },
    Expr(Expr),
}

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
    If {
        cond: Box<Expr>,
        th: Box<Expr>,
        el: Option<Box<Expr>>,
    },
    Block {
        body: Vec<Stmt>,
        trailing: Option<Box<Expr>>,
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

impl From<Lit> for Expr {
    fn from(value: Lit) -> Self {
        Expr::Literal(value)
    }
}

impl From<Lit> for Box<Expr> {
    fn from(value: Lit) -> Self {
        Box::new(Expr::Literal(value))
    }
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
    Xor,
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
pub enum Item {
    Function {
        name: String,
        params: Vec<Arg>,
        return_type: Option<String>,
        body: Expr,
    },
    Struct {
        name: String,
        fields: Vec<Field>,
    },
    Enum {
        name: String,
        variants: Vec<Variant>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arg {
    pub mutable: bool,
    pub name: String,
    pub type_annotation: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Unit,
    Tuple(Vec<String>),
    Struct(Vec<Field>),
}
