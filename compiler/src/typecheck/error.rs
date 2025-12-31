use crate::{
    helpers::{self, concat},
    span,
};

use super::Type;
use std::{error::Error, fmt::Display};

pub type TypeResult<T = Type> = Result<T, TypeErrorS>;

span! { TypeError as TypeErrorS }
#[derive(Debug)]
pub enum TypeError {
    UnboundIdent(String),
    MismatchedTypes(Type, Type),
    MismatchedBranches(Type, Type),
    WrongArgCount(usize, usize),
    CantInfer(Vec<Type>),
    Mutation(String)
}

impl Display for TypeErrorS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            TypeError::UnboundIdent(ident) => {
                write!(f, "identifider `{ident}` at {} is unbound", self.span)
            }
            TypeError::MismatchedTypes(t1, t2) => write!(
                f,
                "mismatched types at {}, found `{t1}`, expected `{t2}`",
                self.span
            ),
            TypeError::MismatchedBranches(th, el) => write!(
                f,
                "branches of if at {} have mismatched types, then: `{th}`, else: `{el}`",
                self.span
            ),
            TypeError::WrongArgCount(needed, provided) => write!(
                f,
                "function call at {} has the wrong number of arguments, needs {needed}, provides {provided}",
                self.span
            ),
            TypeError::CantInfer(options) if options.is_empty() => {
                write!(f, "can't infer type of expression at {}", self.span)
            }
            TypeError::CantInfer(options) => write!(
                f,
                "can't infer type of expression at {}, options are {}",
                self.span,
                concat(options)
            ),
            TypeError::Mutation(name) => write!(
                f,
                "attempted mutation of immutable variable {name} at {}",
                self.span
            )
        }
    }
}

impl Error for TypeErrorS {}
