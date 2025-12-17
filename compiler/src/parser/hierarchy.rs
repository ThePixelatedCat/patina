use crate::parser::ast::{Binding, Field, Item, Stmt, Variant};

use super::{ParseError, ParseResult, Parser, Token};

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn item(&mut self) -> ParseResult<Item> {
        Ok(match self.peek() {
            Token::Fn => {
                self.next();

                let name = self.consume_ident()?;

                self.consume(&Token::LParen)?;

                let mut params = Vec::new();
                while !self.at(&Token::RParen) {
                    params.push(self.binding()?);

                    if !self.consume_at(&Token::Comma) {
                        break;
                    }
                }
                self.next();

                let return_type = if self.consume_at(&Token::Colon) {
                    Some(self.consume_ident()?)
                } else {
                    None
                };

                self.consume(&Token::Arrow)?;

                let body = self.expression()?;

                Item::Function {
                    name,
                    params,
                    return_type,
                    body,
                }
            }
            Token::Struct => {
                self.next();

                Item::Struct {
                    name: self.consume_ident()?,
                    fields: self.fields()?,
                }
            }
            Token::Enum => {
                self.next();

                let name = self.consume_ident()?;

                self.consume(&Token::LBrace)?;

                let mut variants = Vec::new();
                while !self.at(&Token::RBrace) {
                    let name = self.consume_ident()?;

                    let variant = match self.peek() {
                        Token::LBrace => Variant::Struct(name, self.fields()?),
                        Token::LParen => {
                            self.next();

                            let mut types = Vec::new();
                            while !self.at(&Token::RParen) {
                                let ty = self.consume_ident()?;

                                types.push(ty);
                                if self.at(&Token::Comma) {
                                    self.next();
                                } else {
                                    break;
                                }
                            }
                            self.next();

                            Variant::Tuple(name, types)
                        }
                        Token::Comma => Variant::Unit(name),
                        token => {
                            return Err(ParseError::MismatchedToken {
                                expected: "one of `,` `(` `{`".into(),
                                found: token.to_string(),
                            });
                        }
                    };
                    variants.push(variant);

                    if !self.consume_at(&Token::Comma) {
                        break;
                    }
                }
                self.next();

                Item::Enum { name, variants }
            }
            token => {
                return Err(ParseError::UnexpectedToken(
                    token.to_string(),
                    Some("start of item".into()),
                ));
            }
        })
    }

    pub fn statement(&mut self) -> ParseResult<Stmt> {
        Ok(match self.peek() {
            Token::Let => {
                self.next();

                let binding = self.binding()?;

                self.consume(&Token::Eq)?;
                let value = self.expression()?;
                self.consume(&Token::Semicolon)?;

                Stmt::Let { binding, value }
            }
            Token::Ident(_) => {
                let Token::Ident(ident) = self.next().unwrap() else {
                    unreachable!()
                };
                self.consume(&Token::Eq)?;
                let value = self.expression()?;
                self.consume(&Token::Semicolon)?;
                Stmt::Assign { ident, value }
            }
            _ => {
                let expr = self.expression()?;
                self.consume(&Token::Semicolon).map(|_| Stmt::Expr(expr))?
            }
        })
    }

    fn fields(&mut self) -> ParseResult<Vec<Field>> {
        self.consume(&Token::LBrace)?;

        let mut fields = Vec::new();
        while !self.at(&Token::RBrace) {
            let name = self.consume_ident()?;

            self.consume(&Token::Colon)?;

            let ty = self.consume_ident()?;

            fields.push(Field { name, ty });
            if !self.consume_at(&Token::Comma) {
                break;
            }
        }
        self.next();

        Ok(fields)
    }

    fn binding(&mut self) -> ParseResult<Binding> {
        let mutable = self.consume_at(&Token::Mut);

        let name = self.consume_ident()?;

        let type_annotation = if self.at(&Token::Colon) {
            self.next();
            Some(self.consume_ident()?)
        } else {
            None
        };

        Ok(Binding {
            mutable,
            name,
            type_annotation,
        })
    }
}
