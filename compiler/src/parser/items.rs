use std::ops::Range;

use crate::{
    lexer::{Token, TokenType},
    parser::ast::FieldS,
    span::Span,
};

use super::{
    ParseTokenError, ParseResult, Parser,
    ast::{Ast, Field, Item, ItemS, Variant},
};

impl<I: Iterator<Item = Token>> Parser<'_, I> {
    pub fn file(&mut self) -> ParseResult<Ast> {
        let mut items = Vec::new();
        while !self.at(TokenType::Eof) {
            items.push(self.item()?);
        }
        Ok(items)
    }

    pub fn item(&mut self) -> ParseResult<ItemS> {
        Ok(match self.peek() {
            TokenType::Const => {
                let start = self.next().unwrap().span.start;

                let (name, _) = self.ident()?;

                self.consume(TokenType::Colon)?;
                let ty = self.type_()?;

                self.consume(TokenType::Eq)?;
                let value = self.expression()?;

                let end = value.span.end;

                Item::Const { name, ty, value }.spanned(start..end)
            }
            TokenType::Fn => {
                let start = self.next().unwrap().span.start;

                let (name, _) = self.ident()?;

                let (params, _) =
                    self.delimited_list(Self::binding, TokenType::LParen, TokenType::RParen)?;

                let return_type = if self.consume_at(TokenType::Colon) {
                    Some(self.type_()?)
                } else {
                    None
                };

                self.consume(TokenType::Arrow)?;

                let body = self.expression()?;

                let end = body.span.end;

                Item::Function {
                    name,
                    params,
                    return_type,
                    body,
                }
                .spanned(start..end)
            }
            TokenType::Struct => {
                let start = self.next().unwrap().span.start;

                let (name, generic_params) = self.type_name()?;

                let (fields, span) = self.fields()?;
                let end = span.end;

                Item::Struct {
                    name,
                    generic_params,
                    fields,
                }
                .spanned(start..end)
            }
            TokenType::Enum => {
                let start = self.next().unwrap().span.start;

                let (name, generic_params) = self.type_name()?;

                let (variants, variants_span) = self.delimited_list(
                    |this| {
                        let (variant_name, name_span) = this.ident()?;
                        let start = name_span.start;

                        Ok(match this.peek() {
                            TokenType::LBrace => {
                                let (fields, fields_span) = this.fields()?;
                                Variant::Struct(variant_name, fields)
                                    .spanned(start..fields_span.end)
                            }
                            TokenType::LParen => {
                                let (vals, span) = this.delimited_list(
                                    Self::type_,
                                    TokenType::LParen,
                                    TokenType::RParen,
                                )?;

                                Variant::Tuple(variant_name, vals).spanned(start..span.end)
                            }
                            TokenType::Comma => Variant::Unit(variant_name).spanned(name_span),
                            token => {
                                return Err(ParseTokenError::Unexpected(
                                    token,
                                    Some("after variant name. expected one of `,` `(` `{`".into()),
                                ));
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
                .spanned(start..variants_span.end)
            }
            token => {
                return Err(ParseTokenError::Unexpected(
                    token,
                    Some("start of item".into()),
                ));
            }
        })
    }

    fn type_name(&mut self) -> ParseResult<(String, Vec<String>)> {
        let (name, _) = self.ident()?;

        let generic_params = if self.at(TokenType::LAngle) {
            self.delimited_list(
                |this| this.ident().map(|v| v.0),
                TokenType::LAngle,
                TokenType::RAngle,
            )?
            .0
        } else {
            Vec::new()
        };

        Ok((name, generic_params))
    }

    fn fields(&mut self) -> ParseResult<(Vec<FieldS>, Span)> {
        self.delimited_list(
            |this| {
                let (name, start) = match this.peek() {
                    TokenType::Ident => {
                        let span = this.next().unwrap().span;

                        (this.input[Range::from(span)].to_string(), span.start)
                    }
                    other_type => {
                        return Err(ParseTokenError::Mismatched {
                            expected: TokenType::Ident,
                            found: other_type,
                        });
                    }
                };

                this.consume(TokenType::Colon)?;

                let ty = this.type_()?;
                let end = ty.span.end;

                Ok(Field { name, ty }.spanned(start..end))
            },
            TokenType::LBrace,
            TokenType::RBrace,
        )
    }
}
