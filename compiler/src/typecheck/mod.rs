mod error;

use std::{collections::HashMap, fmt::Display, iter, slice};

use crate::{
    helpers::{Span, Spanned, concat},
    parser::ast::{
        Ast, Binding, BindingS, Expr, ExprS, Item, Type as AstType, TypeS as AstTypeS, Unop,
    },
};

use error::{TypeError, TypeResult};

macro_rules! Unit {
    () => {
        Type::Tuple(vec![])
    };
}

macro_rules! check_type {
    ($self:expr, $e:expr, $t:path) => {
        match $self.type_of($e)? {
            $t => (),
            other => return Err(TypeError::MismatchedTypes(other, $t).spanned($e.span)),
        }
    };
}

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
            AstType::Tuple(inners) => Self::Tuple(inners.iter().map(|t| t.into()).collect()),
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

#[derive(Clone)]
pub struct TypeChecker {
    env: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new(ast: &Ast) -> Self {
        let mut new = Self {
            env: HashMap::with_capacity(ast.len()),
        };

        for item in ast {
            // match &item.inner {
            //     Item::Const { name, ty, value } => todo!(),
            //     Item::Function {
            //         name,
            //         params,
            //         return_type,
            //         body,
            //     } => todo!(),
            //     Item::Struct {
            //         name,
            //         generic_params,
            //         fields,
            //     } => todo!(),
            //     Item::Enum {
            //         name,
            //         generic_params,
            //         variants,
            //     } => todo!(),
            // }
        }

        new
    }

    pub fn check(&self, exprs: &[ExprS]) -> TypeResult<Vec<Type>> {
        let mut env = self.clone();

        let mut types = Vec::with_capacity(exprs.len());

        for expr in exprs {
            types.push(env.type_of(expr)?);
        }

        Ok(types)
    }

    pub fn type_of(&mut self, expr: &ExprS) -> TypeResult {
        match &expr.inner {
            Expr::Ident(ident) => self.type_of_ident(ident, &expr.span),
            Expr::Int(_) => todo!(),
            Expr::Float(_) => Ok(Type::Float),
            Expr::Str(_) => Ok(Type::Str),
            Expr::Char(_) => Ok(Type::Char),
            Expr::Bool(_) => Ok(Type::Bool),
            Expr::Array(vals) => todo!(),
            Expr::Tuple(vals) => self.type_of_tuple(vals),
            Expr::FnCall { fun, args } => self.type_of_fn_call(fun, args, &expr.span),
            Expr::BinaryOp { op, lhs, rhs } => todo!(),
            Expr::UnaryOp { op, expr } => self.type_of_unary_op(*op, expr),
            Expr::Index { arr, index } => self.type_of_index(arr, index),
            Expr::FieldAccess { base, field } => todo!(),
            Expr::If { cond, th, el } => self.type_of_if(cond, th, el.as_deref(), &expr.span),
            Expr::Let { binding, value } => self.type_of_let(binding, value),
            Expr::Assign { .. } => Ok(Unit!()),
            Expr::Lambda {
                params,
                return_type,
                body,
            } => todo!(),
            Expr::Block { exprs, trailing } => self.type_of_block(exprs, *trailing),
        }
    }

    fn type_of_ident(&self, ident: &str, span: &Span) -> TypeResult {
        self.env
            .get(ident)
            .cloned()
            .ok_or_else(|| TypeError::UnboundIdent(ident.to_owned()).spanned(span))
    }

    fn type_of_tuple(&mut self, vals: &[ExprS]) -> TypeResult {
        Ok(Type::Tuple(
            vals.iter()
                .map(|e| self.type_of(e))
                .collect::<TypeResult<Vec<Type>>>()?,
        ))
    }

    fn type_of_fn_call(&mut self, fun: &ExprS, args: &Vec<ExprS>, span: &Span) -> TypeResult {
        let (param_tys, result_ty) = match self.type_of(fun)? {
            Type::Fn { params, result } => (params, *result),
            other => return Err(todo!()),
        };

        if param_tys.len() != args.len() {
            return Err(TypeError::WrongArgCount(param_tys.len(), args.len()).spanned(span));
        }

        iter::zip(param_tys, args).try_for_each(|(p, a)| {
            let arg_ty = self.type_of(a)?;

            if p != arg_ty {
                Err(TypeError::MismatchedTypes(p, arg_ty).spanned(a.span))
            } else {
                Ok(())
            }
        })?;

        Ok(result_ty)
    }

    fn type_of_unary_op(&mut self, op: Unop, expr: &ExprS) -> TypeResult {
        match op {
            Unop::Not => {
                check_type!(self, expr, Type::Bool);
                Ok(Type::Bool)
            }
            Unop::Neg => {
                check_type!(self, expr, Type::Int);
                Ok(Type::Int)
            }
        }
    }

    fn type_of_index(&mut self, arr: &ExprS, index: &ExprS) -> TypeResult {
        check_type!(self, index, Type::UInt);

        let Type::Array(ref inner) = self.type_of(arr)? else {
            return Err(todo!());
        };

        Ok(*inner.clone())
    }

    fn type_of_if(
        &mut self,
        cond: &ExprS,
        th: &ExprS,
        el: Option<&ExprS>,
        span: &Span,
    ) -> TypeResult {
        check_type!(self, cond, Type::Bool);

        let th_types = self.check(slice::from_ref(th))?;

        if let Some(el) = el {
            let el_type = self
                .check(slice::from_ref(el))?
                .last()
                .cloned()
                .unwrap_or(Unit!());
            let th_type = th_types.last().cloned().unwrap_or(Unit!());

            if el_type == th_type {
                Ok(th_type)
            } else {
                Err(TypeError::MismatchedBranches(th_type, el_type).spanned(span))
            }
        } else {
            Ok(Unit!())
        }
    }

    fn type_of_let(&mut self, binding: &BindingS, value: &ExprS) -> TypeResult {
        let Binding::Var {
            mutable,
            ident,
            type_annotation,
        } = &binding.inner;

        let ty = match self.type_of(value) {
            Ok(ty) => ty,
            Err(Spanned {
                inner: TypeError::CantInfer(options),
                span,
            }) => {
                let Some(ty) = type_annotation else {
                    return Err(TypeError::CantInfer(options).spanned(span));
                };
                let ty: Type = ty.into();
                if options.is_empty() {
                    ty
                } else {
                    if options.contains(&ty) { ty } else { todo!() }
                }
            }
            other => return other,
        };

        self.env.insert(ident.to_owned(), ty);

        Ok(Unit!())
    }

    fn type_of_block(&self, exprs: &[ExprS], trailing: bool) -> TypeResult {
        let types = self.check(exprs)?;

        Ok(if trailing && let Some(last) = types.last().cloned() {
            last
        } else {
            Unit!()
        })
    }
}
