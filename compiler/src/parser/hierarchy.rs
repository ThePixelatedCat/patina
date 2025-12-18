use crate::parser::ast::{Ast, Binding, Field, Item, Stmt, Type, Variant};

use super::{ParseError, ParseResult, Parser, Token};

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn file(&mut self) -> ParseResult<Ast> {
        let mut items = Vec::new();
        while !self.at(&Token::Eof) {
            items.push(self.item()?);
        }
        Ok(items)
    }

    pub fn item(&mut self) -> ParseResult<Item> {
        Ok(match self.peek() {
            Token::Const => {
                self.next();

                let ident = self.consume_ident()?;

                self.consume(&Token::Colon)?;
                let ty = self.type_()?;

                self.consume(&Token::Eq)?;
                let value = self.expression()?;

                Item::Const { ident, ty, value }
            }
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
                    Some(self.type_()?)
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
                    name: self.type_()?,
                    fields: self.fields()?,
                }
            }
            Token::Enum => {
                self.next();

                let name = self.type_()?;

                self.consume(&Token::LBrace)?;

                let mut variants = Vec::new();
                while !self.at(&Token::RBrace) {
                    let variant_name = self.consume_ident()?;

                    let variant = match self.peek() {
                        Token::LBrace => Variant::Struct(variant_name, self.fields()?),
                        Token::LParen => {
                            self.next();

                            let mut types = Vec::new();
                            while !self.at(&Token::RParen) {
                                types.push(self.type_()?);

                                if !self.consume_at(&Token::Comma) {
                                    break;
                                }
                            }
                            self.next();

                            Variant::Tuple(variant_name, types)
                        }
                        Token::Comma => Variant::Unit(variant_name),
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
            let ty = self.type_()?;

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

        let type_annotation = if self.consume_at(&Token::Colon) {
            Some(self.type_()?)
        } else {
            None
        };

        Ok(Binding {
            mutable,
            name,
            type_annotation,
        })
    }

    fn type_(&mut self) -> ParseResult<Type> {
        let name = self.consume_ident()?;

        let mut generics = Vec::new();
        if self.consume_at(&Token::LAngle) {
            while !self.at(&Token::RAngle) {
                generics.push(self.type_()?);

                if !self.consume_at(&Token::Comma) {
                    break;
                }
            }
            self.next();
        }

        Ok(Type { name, generics })
    }
}
