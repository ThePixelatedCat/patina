use std::{cmp, convert::Infallible, iter};

use ena::unify::{UnifyKey, UnifyValue};

use crate::{parser::ast::Type as AstType, typecheck::error::TypeErrorS};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Var(TypeId),
    Named(String, Vec<Type>),
}

impl Type {
    pub fn id(&self) -> Option<TypeId> {
        match self {
            Type::Var(id) => Some(*id),
            Type::Named(..) => None,
        }
    }
}

impl From<AstType> for Type {
    fn from(value: AstType) -> Self {
        match value {
            AstType::Named { name, generics } => Type::Named(
                name,
                generics
                    .into_iter()
                    .map(|type_s| type_s.inner.into())
                    .collect(),
            ),
            AstType::Array(ty) => Type::Named("$Array".to_string(), vec![ty.inner.into()]),
            AstType::Tuple(tys) => Type::Named(
                "$Tuple".to_string(),
                tys.into_iter().map(|type_s| type_s.inner.into()).collect(),
            ),
            AstType::Fn { params, result } => {
                let type_args: Vec<Type> = params
                    .into_iter()
                    .map(|type_s| type_s.inner.into())
                    .chain(iter::once(result.inner.into()))
                    .collect();
                Type::Named("$Function".to_string(), type_args)
            }
        }
    }
}

impl UnifyValue for Type {
    type Error = Infallible;

    fn unify_values(a: &Self, b: &Self) -> Result<Self, Self::Error> {
        match (a, b) {
            (Type::Var(id_a), Type::Var(id_b)) => {
                Ok(Type::Var(cmp::min(id_a.index(), id_b.index()).into()))
            }
            (ty @ Type::Named(..), Type::Var(_)) | (Type::Var(_), ty @ Type::Named(..)) => {
                Ok(ty.clone())
            }
            (Type::Named(..), Type::Named(..)) => {
                panic!("shouldn't be unifying two concrete types")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct TypeId(u32);

impl From<u32> for TypeId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl UnifyKey for TypeId {
    type Value = Type;
    fn index(&self) -> u32 {
        self.0
    }
    fn from_index(u: u32) -> TypeId {
        u.into()
    }
    fn tag() -> &'static str {
        "TypeId"
    }
}
