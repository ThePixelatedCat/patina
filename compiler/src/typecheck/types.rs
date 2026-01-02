use std::fmt::Display;

use crate::{
    helpers::concat,
    parser::ast::{Type as AstType, TypeS as AstTypeS},
};

#[derive(Debug, Clone)]
pub enum Type {
    GInt,
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
    Any,
}

impl Type {
    pub const fn is_integer(&self) -> bool {
        matches!(self, Self::GInt | Self::Int | Self::UInt | Self::Byte)
    }

    pub fn is_numeric(&self) -> bool {
        self.is_integer() || *self == Self::Float
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::GInt, int) if int.is_integer() => true,
            (int, Self::GInt) if int.is_integer() => true,
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            (Self::Tuple(l0), Self::Tuple(r0)) => l0 == r0,
            (
                Self::Fn {
                    params: l_params,
                    result: l_result,
                },
                Self::Fn {
                    params: r_params,
                    result: r_result,
                },
            ) => l_params == r_params && l_result == r_result,
            (
                Self::Named {
                    name: l_name,
                    generics: l_generics,
                },
                Self::Named {
                    name: r_name,
                    generics: r_generics,
                },
            ) => l_name == r_name && l_generics == r_generics,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
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
                    generics: generics.iter().map(Self::from).collect(),
                }
            }
            AstType::Array(inner) => Self::Array(Box::new((&inner.inner).into())),
            AstType::Tuple(inners) => Self::Tuple(inners.iter().map(Self::from).collect()),
            AstType::Fn { params, result } => Self::Fn {
                params: params.iter().map(Self::from).collect(),
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
            Self::GInt => "{integer}".fmt(f),
            Self::Int => "Int".fmt(f),
            Self::UInt => "UInt".fmt(f),
            Self::Byte => "Byte".fmt(f),
            Self::Float => "Float".fmt(f),
            Self::Bool => "Bool".fmt(f),
            Self::Str => "Str".fmt(f),
            Self::Char => "Char".fmt(f),
            Self::Array(inner) => write!(f, "[{inner}]"),
            Self::Tuple(items) => write!(f, "({})", concat(items)),
            Self::Fn { params, result } => write!(f, "fn({}): {result}", concat(params)),
            Self::Named { name, generics } => {
                if generics.is_empty() {
                    write!(f, "{name}")
                } else {
                    write!(f, "{name}<{}>", concat(generics))
                }
            }
            Self::Never => "!".fmt(f),
            Self::Any => "{any}".fmt(f),
        }
    }
}
