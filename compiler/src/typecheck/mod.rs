mod error;
#[cfg(test)]
mod test;
mod types;

use std::{collections::HashMap, iter, slice};

use crate::{
    helpers::{Span, Spanned},
    parser::ast::{Ast, Binding, BindingS, Bop, Expr, ExprS, Unop},
};

use error::{TypeError, TypeResult};
use types::Type;

macro_rules! Unit {
    () => {
        Type::Tuple(vec![])
    };
}

macro_rules! check_type {
    ($self:expr, $e:expr, $t:path) => {
        match $self.type_of($e)? {
            $t => (),
            other => {
                return Err(TypeError::MismatchedTypes {
                    found: other,
                    expected: $t,
                }
                .spanned($e.span))
            }
        }
    };
}

#[derive(Clone)]
pub struct BindingInfo {
    ty: Type,
    mutable: bool,
}

#[derive(Clone, Default)]
pub struct TypeChecker {
    env: HashMap<String, BindingInfo>,
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
            Expr::Ident(ident) => self.type_of_ident(Spanned {
                inner: ident,
                span: expr.span,
            }),
            Expr::Int(v) => Ok(if *v > i64::MAX as u64 {
                Type::UInt
            } else {
                Type::GInt
            }),
            Expr::Float(_) => Ok(Type::Float),
            Expr::Str(_) => Ok(Type::Str),
            Expr::Char(_) => Ok(Type::Char),
            Expr::Bool(_) => Ok(Type::Bool),
            Expr::Array(vals) => self.type_of_array(vals),
            Expr::Tuple(vals) => self.type_of_tuple(vals),
            Expr::FnCall { fun, args } => self.type_of_fn_call(fun, args, expr.span),
            Expr::BinaryOp { op, lhs, rhs } => self.type_of_binary_op(*op, lhs, rhs),
            Expr::UnaryOp { op, expr } => self.type_of_unary_op(*op, expr),
            Expr::Index { arr, index } => self.type_of_index(arr, index),
            Expr::FieldAccess { base, field } => todo!(),
            Expr::If { cond, th, el } => self.type_of_if(cond, th, el.as_deref(), &expr.span),
            Expr::Let { binding, value } => self.type_of_let(binding, value),
            Expr::Assign { ident, value } => self.type_of_assign(ident.as_deref(), value),
            Expr::Lambda {
                params,
                return_type,
                body,
            } => todo!(),
            Expr::Block { exprs, trailing } => self.type_of_block(exprs, *trailing),
        }
    }

    fn type_of_ident(&self, ident: Spanned<&str>) -> TypeResult {
        self.env
            .get(ident.inner)
            .cloned()
            .map(|i| i.ty)
            .ok_or_else(|| TypeError::UnboundIdent(ident.inner.to_owned()).spanned(ident.span))
    }

    fn type_of_array(&mut self, vals: &[ExprS]) -> TypeResult {
        let ty = match vals.first() {
            Some(e) => self.type_of(e)?,
            None => Type::Any,
        };

        vals[1..].iter().try_for_each(|v| {
            let this_ty = self.type_of(v)?;
            if this_ty == ty {
                Ok(())
            } else {
                Err(TypeError::MismatchedTypes {
                    found: this_ty,
                    expected: ty.clone(),
                }
                .spanned(v.span))
            }
        })?;

        Ok(Type::Array(ty.into()))
    }

    fn type_of_tuple(&mut self, vals: &[ExprS]) -> TypeResult {
        Ok(Type::Tuple(
            vals.iter()
                .map(|e| self.type_of(e))
                .collect::<TypeResult<Vec<Type>>>()?,
        ))
    }

    fn type_of_fn_call(&mut self, fun: &ExprS, args: &Vec<ExprS>, span: Span) -> TypeResult {
        let (param_tys, result_ty) = match self.type_of(fun)? {
            Type::Fn { params, result } => (params, *result),
            other => return Err(todo!()),
        };

        if param_tys.len() != args.len() {
            return Err(TypeError::WrongArgCount {
                needed: param_tys.len(),
                provided: args.len(),
            }
            .spanned(span));
        }

        iter::zip(param_tys, args).try_for_each(|(p, a)| {
            let arg_ty = self.type_of(a)?;

            if p == arg_ty {
                Ok(())
            } else {
                Err(TypeError::MismatchedTypes {
                    found: p,
                    expected: arg_ty,
                }
                .spanned(a.span))
            }
        })?;

        Ok(result_ty)
    }

    fn type_of_binary_op(&mut self, op: Bop, lhs: &ExprS, rhs: &ExprS) -> TypeResult {
        match op {
            Bop::Add | Bop::Sub | Bop::Mul | Bop::Div | Bop::Exp => {
                let (lhs_ty, rhs_ty) = (self.type_of(lhs)?, self.type_of(rhs)?);

                if !lhs_ty.is_numeric() {
                    return Err(TypeError::NotNumeric(lhs_ty).spanned(lhs.span));
                }

                if !rhs_ty.is_numeric() {
                    return Err(TypeError::NotNumeric(rhs_ty).spanned(rhs.span));
                }

                if lhs_ty != rhs_ty {
                    return Err(TypeError::MismatchedTypes {
                        found: rhs_ty,
                        expected: lhs_ty,
                    }
                    .spanned(rhs.span));
                }

                Ok(lhs_ty)
            }
            Bop::And | Bop::Or | Bop::Xor => {
                check_type!(self, lhs, Type::Bool);
                check_type!(self, rhs, Type::Bool);
                Ok(Type::Bool)
            }
            Bop::BOr | Bop::BAnd => {
                let (lhs_ty, rhs_ty) = (self.type_of(lhs)?, self.type_of(rhs)?);

                if !lhs_ty.is_integer() {
                    return Err(TypeError::NotInteger(lhs_ty).spanned(lhs.span));
                }

                if !rhs_ty.is_integer() {
                    return Err(TypeError::NotInteger(rhs_ty).spanned(rhs.span));
                }

                if lhs_ty != rhs_ty {
                    return Err(TypeError::MismatchedTypes {
                        found: rhs_ty,
                        expected: lhs_ty,
                    }
                    .spanned(rhs.span));
                }

                Ok(lhs_ty)
            }
            Bop::Eqq | Bop::Neq => {
                let (lhs_ty, rhs_ty) = (self.type_of(lhs)?, self.type_of(rhs)?);

                if lhs_ty == rhs_ty {
                    Ok(Type::Bool)
                } else {
                    Err(TypeError::MismatchedTypes {
                        found: rhs_ty,
                        expected: lhs_ty,
                    }
                    .spanned(rhs.span))
                }
            }
            Bop::Gt | Bop::Lt | Bop::Geq | Bop::Leq => {
                let (lhs_ty, rhs_ty) = (self.type_of(lhs)?, self.type_of(rhs)?);

                if !lhs_ty.is_numeric() {
                    return Err(TypeError::NotNumeric(lhs_ty).spanned(lhs.span));
                }

                if !rhs_ty.is_numeric() {
                    return Err(TypeError::NotNumeric(rhs_ty).spanned(rhs.span));
                }

                Ok(Type::Bool)
            }
        }
    }

    fn type_of_unary_op(&mut self, op: Unop, expr: &ExprS) -> TypeResult {
        match op {
            Unop::Not => {
                check_type!(self, expr, Type::Bool);
                Ok(Type::Bool)
            }
            Unop::Neg => match self.type_of(expr)? {
                Type::GInt => Ok(Type::Int),
                ty @ (Type::Int | Type::Float) => Ok(ty),
                other => Err(todo!()),
            },
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
                Err(TypeError::MismatchedBranches {
                    th: th_type,
                    el: el_type,
                }
                .spanned(span))
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

        let ty = match self.type_of(value)? {
            Type::GInt => {
                if let Some(ty) = type_annotation.as_ref().map(Type::from)
                    && ty.is_integer()
                {
                    ty
                } else {
                    Type::GInt
                }
            }
            ty => ty,
        };

        self.env.insert(
            ident.to_owned(),
            BindingInfo {
                ty,
                mutable: *mutable,
            },
        );

        Ok(Unit!())
    }

    fn type_of_assign(&mut self, ident: Spanned<&str>, value: &ExprS) -> TypeResult {
        let assigned_ty = self.type_of(value)?;

        let info = self
            .env
            .get(ident.inner)
            .ok_or_else(|| TypeError::UnboundIdent(ident.inner.to_owned()).spanned(ident.span))?;

        if !info.mutable {
            return Err(TypeError::Mutation(ident.inner.to_owned()).spanned(ident.span));
        }

        if info.ty != assigned_ty {
            return Err(TypeError::MismatchedTypes {
                found: assigned_ty,
                expected: info.ty.clone(),
            }
            .spanned(value.span));
        }

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
