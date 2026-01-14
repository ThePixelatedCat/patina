use std::fmt::Display;

use ena::unify::UnifyKey;

use crate::{
    helpers::concat,
    parser::ast::{Type as AstType, TypeS as AstTypeS},
};

#[derive(Debug, Clone)]
pub enum Type {
    Var(usize),
    Named(String, Vec<Type>)
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct TypeId(u32);

impl UnifyKey for TypeId {
    type Value = ();
    fn index(&self) -> u32 {
        self.0
    }
    fn from_index(u: u32) -> TypeId {
        TypeId(u)
    }
    fn tag() -> &'static str {
        "TypeId"
    }
}
