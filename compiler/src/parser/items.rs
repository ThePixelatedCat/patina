use crate::lexer::{Token, TokenType};

use super::{
    ParseError, ParseResult, Parser,
    ast::{Ast, Field, Item, Variant},
};

impl<'input, I: Iterator<Item = Token>> Parser<'input, I> {
    pub fn file(&mut self) -> ParseResult<Ast> {
        let mut items = Vec::new();
        while !self.at(TokenType::Eof) {
            items.push(self.item()?);
        }
        Ok(items)
    }

    pub fn item(&mut self) -> ParseResult<Item> {
        Ok(match self.peek() {
            TokenType::Const => {
                self.next();

                let ident = self.ident()?;

                self.consume(TokenType::Colon)?;
                let ty = self.type_()?;

                self.consume(TokenType::Eq)?;
                let value = self.expression()?;

                Item::Const { ident, ty, value }
            }
            TokenType::Fn => {
                self.next();

                let name = self.ident()?;

                let params =
                    self.delimited_list(Self::binding, TokenType::LParen, TokenType::RParen)?;

                let return_type = if self.consume_at(TokenType::Colon) {
                    Some(self.type_()?)
                } else {
                    None
                };

                self.consume(TokenType::Arrow)?;

                let body = self.expression()?;

                Item::Function {
                    name,
                    params,
                    return_type,
                    body,
                }
            }
            TokenType::Struct => {
                self.next();

                let (name, generic_params) = self.type_name()?;

                Item::Struct {
                    name,
                    generic_params,
                    fields: self.fields()?,
                }
            }
            TokenType::Enum => {
                self.next();

                let (name, generic_params) = self.type_name()?;

                let variants = self.delimited_list(
                    |this| {
                        let variant_name = this.ident()?;

                        Ok(match this.peek() {
                            TokenType::LBrace => Variant::Struct(variant_name, this.fields()?),
                            TokenType::LParen => Variant::Tuple(
                                variant_name,
                                this.delimited_list(
                                    Self::type_,
                                    TokenType::LParen,
                                    TokenType::RParen,
                                )?,
                            ),
                            TokenType::Comma => Variant::Unit(variant_name),
                            token => {
                                return Err(ParseError::MismatchedToken {
                                    expected: "one of `,` `(` `{`".into(),
                                    found: token.to_string(),
                                });
                            }
                        })
                    },
                    TokenType::LBrace,
                    TokenType::RBrace,
                )?;

                Item::Enum {
                    name,
                    generic_params,
                    variants,
                }
            }
            token => {
                return Err(ParseError::UnexpectedToken(
                    token.to_string(),
                    Some("start of item".into()),
                ));
            }
        })
    }

    fn type_name(&mut self) -> ParseResult<(String, Vec<String>)> {
        let name = self.ident()?;

        let generic_params = if self.at(TokenType::LAngle) {
            self.delimited_list(Self::ident, TokenType::LAngle, TokenType::RAngle)?
        } else {
            Vec::new()
        };

        Ok((name, generic_params))
    }

    fn fields(&mut self) -> ParseResult<Vec<Field>> {
        self.delimited_list(
            |this| {
                let name = this.ident()?;

                this.consume(TokenType::Colon)?;
                let ty = this.type_()?;

                Ok(Field { name, ty })
            },
            TokenType::LBrace,
            TokenType::RBrace,
        )
    }
}
