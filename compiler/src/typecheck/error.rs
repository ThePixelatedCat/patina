use crate::span;

use super::Type;
use std::{error::Error, fmt::Display};

pub type TypeResult<T = Type> = Result<T, TypeErrorS>;

span! { TypeError as TypeErrorS }
#[derive(Debug)]
pub enum TypeError {
    UnboundIdent(String),
    MismatchedTypes { found: Type, expected: Type },
    MismatchedBranches { th: Type, el: Type },
    WrongArgCount { needed: usize, provided: usize },
    CantInfer,
    Mutation(String),
    NotInteger(Type),
    NotNumeric(Type),
}

impl Display for TypeErrorS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            TypeError::UnboundIdent(ident) => {
                write!(f, "identifider `{ident}` at {} is unbound", self.span)
            }
            TypeError::MismatchedTypes { found, expected } => write!(
                f,
                "mismatched types at {}, found `{found}`, expected `{expected}`",
                self.span
            ),
            TypeError::MismatchedBranches { th, el } => write!(
                f,
                "branches of if at {} have mismatched types, then: `{th}`, else: `{el}`",
                self.span
            ),
            TypeError::WrongArgCount { needed, provided } => write!(
                f,
                "function call at {} has the wrong number of arguments, needs {needed}, provides {provided}",
                self.span
            ),
            TypeError::CantInfer => write!(f, "can't infer type of expression at {}", self.span),
            TypeError::Mutation(name) => write!(
                f,
                "attempted mutation of immutable variable {name} at {}",
                self.span
            ),
            TypeError::NotInteger(ty) => {
                write!(f, "expected an integer at {}, found {ty}", self.span)
            }
            TypeError::NotNumeric(ty) => {
                write!(f, "expected a number at {}, found {ty}", self.span)
            }
        }
    }
}

impl Error for TypeErrorS {}
