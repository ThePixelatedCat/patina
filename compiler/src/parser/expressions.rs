use std::{ops::Range, str::FromStr};

use crate::{
    lexer::{Token, TokenType},
    parser::ast::ExprS,
    span::{Span, Spannable},
};

use super::{
    ParseError, ParseResult, Parser,
    ast::{Bop, Expr, Unop},
};

trait PrefixOperator {
    fn binding_power(&self) -> u8;
}

trait InfixOperator {
    fn binding_power(&self) -> (u8, u8);
}

impl PrefixOperator for Unop {
    fn binding_power(&self) -> u8 {
        match self {
            Unop::Neg | Unop::Not => 51,
        }
    }
}

impl InfixOperator for Bop {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Bop::Assign => (1, 2),
            Bop::Or => (3, 4),
            Bop::And => (5, 6),
            Bop::Eqq | Bop::Neq => (7, 8),
            Bop::Gt | Bop::Lt | Bop::Leq | Bop::Geq => (9, 10),
            Bop::BOr => (11, 12),
            Bop::Xor => (13, 14),
            Bop::BAnd => (15, 16),
            Bop::Add | Bop::Sub => (17, 18),
            Bop::Mul | Bop::Div => (19, 20),
            Bop::Exp => (22, 21),
        }
    }
}

impl<'input, I: Iterator<Item = Token>> Parser<'input, I> {
    pub fn expression(&mut self) -> ParseResult<ExprS> {
        self.parse_expression(0)
    }

    fn parse_expression(&mut self, binding_power: u8) -> ParseResult<ExprS> {
        let mut lhs = match self.peek() {
            TokenType::LParen => {
                let start = self.next().unwrap().span.start;
                let expr = self.expression()?;
                
                let expr = if self.consume_at(TokenType::Comma) {
                    let mut exprs = vec![expr];
                    while !self.at(TokenType::RParen) {
                        exprs.push(self.expression()?);

                        if !self.consume_at(TokenType::Comma) {
                            break;
                        }
                    }

                    Expr::Tuple(exprs)
                } else {
                    expr.inner
                };

                let end = self.consume(TokenType::RParen)?.span.end;

                expr.spanned(start..end)
            }
            TokenType::IntLit => {
                let token = self.next().unwrap();
                let val = i64::from_str(self.input[token.span.into()]).unwrap();
                Expr::Int(val).spanned(token.span)
            }
            TokenType::FloatLit => {
                let token = self.next().unwrap();
                let val = f64::from_str(self.input[token.span.into()]).unwrap();
                Expr::Float(val).spanned(token.span)
            }
            TokenType::StringLit => {
                let token = self.next().unwrap();
                let span = token.span;

                let val = self.input[span.start + 1..span.end - 1]
                    .replace("\\n", "\n")
                    .replace("\\\"", "\"")
                    .replace("\\\\", "\\");

                Expr::Str(val).spanned(token.span)
            }
            TokenType::CharLit => {
                let token = self.next().unwrap();
                let span = token.span;

                let val = self.input[span.start + 1..span.end - 1]
                    .replace("\\n", "\n")
                    .replace("\\\'", "'")
                    .replace("\\\\", "\\")
                    .chars()
                    .next()
                    .unwrap();

                Expr::Char(val).spanned(token.span)
            }
            TokenType::True => Expr::Bool(true).spanned(self.next().unwrap().span),
            TokenType::False => Expr::Bool(false).spanned(self.next().unwrap().span),
            TokenType::LBracket => {
                let (arr, span) = self.delimited_list(
                    Self::expression,
                    TokenType::LBracket,
                    TokenType::RBracket,
                )?;
                Expr::Array(arr).spanned(span)
            }
            TokenType::Ident => {
                let token = self.next().unwrap();
                let range: Range<_> = token.span.into();

                let ident = self.input[range].to_string();

                Expr::Ident(ident).spanned(token.span)
            }
            TokenType::If => {
                let start = self.next().unwrap().span.start;

                self.consume(TokenType::LParen)?;
                let cond = self.expression()?;
                self.consume(TokenType::RParen)?;

                let th = self.expression()?;

                let el = if self.consume_at(TokenType::Else) {
                    Some(Box::new(self.expression()?))
                } else {
                    None
                };

                let end = el.map(|e| e.span.end).unwrap_or(th.span.end);

                Expr::If {
                    cond: Box::new(cond),
                    th: Box::new(th),
                    el,
                }
                .spanned(start..end)
            }
            op @ (TokenType::Minus | TokenType::Bang) => {
                let op = match op {
                    TokenType::Minus => Unop::Neg,
                    TokenType::Bang => Unop::Not,
                    _ => unreachable!(),
                };

                let start = self.next().unwrap().span.start;

                let right_binding_power = op.binding_power();
                let expr = self.parse_expression(right_binding_power)?;

                Expr::UnaryOp {
                    op,
                    expr: Box::new(expr),
                }.spanned(start..expr.span.end)
            }
            TokenType::Let => {
                let start = self.next().unwrap().span.start;

                let binding = self.binding()?;

                self.consume(TokenType::Eq)?;
                let value = self.expression()?;

                Expr::Let {
                    binding,
                    value: Box::new(value),
                }.spanned(start..value.span.end)
            }
            TokenType::Pipe => {
                let (params, Span { start, .. }) =
                    self.delimited_list(Self::binding, TokenType::Pipe, TokenType::Pipe)?;

                let return_type = if self.consume_at(TokenType::Colon) {
                    Some(self.type_()?)
                } else {
                    None
                };

                self.consume(TokenType::Arrow)?;

                let body = Box::new(self.expression()?);

                Expr::Lambda {
                    params,
                    return_type,
                    body,
                }.spanned(start..body.span.end)
            }
            TokenType::LBrace => {
                let start = self.next().unwrap().span.start;

                let mut trailing = true;
                let mut exprs = Vec::new();
                while !self.at(TokenType::RBrace) {
                    exprs.push(self.expression()?);

                    if self.consume_at(TokenType::Semicolon) && self.at(TokenType::RBrace) {
                        trailing = false;
                        break;
                    }
                }
                let end = self.consume(TokenType::RBrace)?.span.end;

                Expr::Block { exprs, trailing }.spanned(start..end)
            }
            token => {
                return Err(ParseError::UnexpectedToken(
                    token,
                    Some("start of expression".into()),
                ));
            }
        };
        loop {
            let op = match self.peek() {
                TokenType::Eq => Bop::Assign,
                TokenType::Plus => Bop::Add,
                TokenType::Minus => Bop::Sub,
                TokenType::Times => Bop::Mul,
                TokenType::FSlash => Bop::Div,
                TokenType::Xor => Bop::Xor,
                TokenType::Ampersand => Bop::BAnd,
                TokenType::Pipe => Bop::BOr,
                TokenType::Exponent => Bop::Exp,
                TokenType::Eqq => Bop::Eqq,
                TokenType::Neq => Bop::Neq,
                TokenType::And => Bop::And,
                TokenType::Or => Bop::Or,
                TokenType::LAngle => Bop::Lt,
                TokenType::Leq => Bop::Leq,
                TokenType::RAngle => Bop::Gt,
                TokenType::Geq => Bop::Geq,
                TokenType::LBracket => {
                    self.next();

                    let start = lhs.span.start;

                    let index = Box::new(self.expression()?);
                    let end = self.consume(TokenType::RBracket)?.span.end;

                    lhs = Expr::Index {
                        arr: Box::new(lhs),
                        index
                    }.spanned(start..end);
                    continue;
                }
                TokenType::Dot => {
                    self.next();

                    let start = lhs.span.start;

                    let field = self.ident()?;
                    let end = lhs.span.end + 1 + field.len();

                    lhs = Expr::FieldAccess {
                        base: Box::new(lhs),
                        field
                    }.spanned(start..end);
                    continue;
                }
                TokenType::LParen => {
                    let start = lhs.span.start;

                    let (args, Span { end, .. }) =
                        self.delimited_list(Self::expression, TokenType::LParen, TokenType::RParen)?;

                    lhs = Expr::FnCall {
                        fun: Box::new(lhs),
                        args,
                    }.spanned(start..end);
                    continue;
                }
                TokenType::Eof => break,
                TokenType::Else
                | TokenType::RParen // Delimiters
                | TokenType::RBrace
                | TokenType::RBracket
                | TokenType::Comma
                | TokenType::Semicolon
                | TokenType::Fn
                | TokenType::Const
                | TokenType::Struct
                | TokenType::Enum => break,
                token => return Err(ParseError::UnexpectedToken(token, Some("end of expression".into()))),
            };

            let (left_binding_power, right_binding_power) = op.binding_power();

            if left_binding_power < binding_power {
                break;
            }

            self.next();

            let rhs = self.parse_expression(right_binding_power)?;

            let start = lhs.span.start;
            let end = rhs.span.end;

            lhs = Expr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }.spanned(start..end);
        }

        Ok(lhs)
    }
}
