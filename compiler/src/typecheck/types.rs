use std::fmt::Display;

use crate::{
    helpers::concat,
    parser::ast::{Type as AstType, TypeS as AstTypeS},
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    UInt,
    Byte,
    Float,
    Bool,
    Str,
    Char,
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Fn {
        params: Vec<Type>,
        result: Box<Type>,
    },
    Named {
        name: String,
        generics: Vec<Type>,
    },
    Never,
}

impl From<&AstType> for Type {
    fn from(value: &AstType) -> Self {
        match value {
            AstType::Named { name, generics } => {
                if generics.is_empty() {
                    match name.as_str() {
                        "Int" => return Self::Int,
                        "UInt" => return Self::UInt,
                        "Byte" => return Self::Byte,
                        "Float" => return Self::Float,
                        "Bool" => return Self::Bool,
                        "Str" => return Self::Str,
                        "Char" => return Self::Char,
                        _ => (),
                    }
                }

                Self::Named {
                    name: name.to_owned(),
                    generics: generics.iter().map(|t| t.into()).collect(),
                }
            }
            AstType::Array(inner) => Self::Array(Box::new((&inner.inner).into())),
            AstType::Tuple(inners) => {
                Self::Tuple(inners.iter().map(|t| t.into()).collect())
            }
            AstType::Fn { params, result } => Self::Fn {
                params: params.iter().map(|t| t.into()).collect(),
                result: Box::new(result.as_ref().into()),
            },
        }
    }
}

impl From<&AstTypeS> for Type {
    fn from(value: &AstTypeS) -> Self {
        (&value.inner).into()
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => "Int".fmt(f),
            Type::UInt => "UInt".fmt(f),
            Type::Byte => "Byte".fmt(f),
            Type::Float => "Float".fmt(f),
            Type::Bool => "Bool".fmt(f),
            Type::Str => "Str".fmt(f),
            Type::Char => "Char".fmt(f),
            Type::Array(inner) => write!(f, "[{inner}]"),
            Type::Tuple(items) => write!(f, "({})", concat(items)),
            Type::Fn { params, result } => write!(f, "fn({}): {result}", concat(params)),
            Type::Named { name, generics } => {
                if generics.is_empty() {
                    write!(f, "{name}")
                } else {
                    write!(f, "{name}<{}>", concat(generics))
                }
            }
            Type::Never => "!".fmt(f),
        }
    }
}