macro_rules! span {
    ($t:ident as $s:ident) => {
        pub type $s = Spanned<$t>;
        impl Spannable for $t {}
    };
}

use crate::span::{Spannable, Spanned};

pub type Ast = Vec<ItemS>;

span! {Item as ItemS}
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Const {
        name: String,
        ty: TypeS,
        value: ExprS,
    },
    Function {
        name: String,
        params: Vec<BindingS>,
        return_type: Option<TypeS>,
        body: ExprS,
    },
    Struct {
        name: String,
        generic_params: Vec<String>,
        fields: Vec<FieldS>,
    },
    Enum {
        name: String,
        generic_params: Vec<String>,
        variants: Vec<VariantS>,
    },
}

span!(Variant as VariantS);
#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Unit(String),
    Tuple(String, Vec<TypeS>),
    Struct(String, Vec<FieldS>),
}

// pub type FieldS = Spanned<Field>;
// impl Spannable for Field {}
span! {Field as FieldS}
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub ty: TypeS,
}

span!{Binding as BindingS}
#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    pub mutable: bool,
    pub name: String,
    pub type_annotation: Option<TypeS>,
}

span! {Type as TypeS}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Ident {
        name: String,
        generics: Vec<TypeS>,
    },
    Array(Box<TypeS>),
    Tuple(Vec<TypeS>),
    Fn {
        params: Vec<TypeS>,
        result: Box<TypeS>,
    },
}

span! {Expr as ExprS}
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(String),
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),
    Bool(bool),
    Array(Vec<ExprS>),
    Tuple(Vec<ExprS>),
    FnCall {
        fun: Box<ExprS>,
        args: Vec<ExprS>,
    },
    BinaryOp {
        op: Bop,
        lhs: Box<ExprS>,
        rhs: Box<ExprS>,
    },
    UnaryOp {
        op: Unop,
        expr: Box<ExprS>,
    },
    Index {
        arr: Box<ExprS>,
        index: Box<ExprS>,
    },
    FieldAccess {
        base: Box<ExprS>,
        field: String,
    },
    If {
        cond: Box<ExprS>,
        th: Box<ExprS>,
        el: Option<Box<ExprS>>,
    },
    Let {
        binding: BindingS,
        value: Box<ExprS>,
    },
    Assign {
        name: String,
        value: Box<ExprS>,
    },
    Lambda {
        params: Vec<BindingS>,
        return_type: Option<TypeS>,
        body: Box<ExprS>,
    },
    Block {
        exprs: Vec<ExprS>,
        trailing: bool,
    },
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
    BOr,
    BAnd,
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
