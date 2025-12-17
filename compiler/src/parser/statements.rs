use crate::{next_checked, parser::IDENT_DISCRIM};

use super::{ParseError, ParseResult, Parser, Token, ast::Stmt};

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn statement(&mut self) -> ParseResult<Stmt> {
        Ok(match self.peek() {
            Token::Let => {
                self.next();
                let mutable = self.at(Token::Mut.ty());
                if mutable {
                    self.next();
                }

                let ident = next_checked!(self, Token::Ident, *IDENT_DISCRIM);

                let type_annotation = if self.at(Token::Colon.ty()) {
                    self.next();
                    Some(next_checked!(self, Token::Ident, *IDENT_DISCRIM))
                } else {
                    None
                };

                self.consume(Token::Eq.ty())?;
                let value = self.expression()?;
                self.consume(Token::Semicolon.ty())?;

                Stmt::Let {
                    mutable,
                    ident,
                    type_annotation,
                    value,
                }
            }
            Token::Ident(_) => {
                let Token::Ident(ident) = self.next().unwrap() else {
                    unreachable!()
                };
                self.consume(Token::Eq.ty())?;
                let value = self.expression()?;
                self.consume(Token::Semicolon.ty())?;
                Stmt::Assign { ident, value }
            }
            _ => {
                let expr = self.expression()?;
                self.consume(Token::Semicolon.ty())
                    .map(|_| Stmt::Expr(expr))?
            }
        })
    }
}
