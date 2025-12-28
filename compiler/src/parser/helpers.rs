use crate::{
    lexer::{Token, TokenType},
    span::Span,
};

use super::{
    ParseError, ParseResult, Parser,
    ast::{Binding, Type},
};

impl<'input, I: Iterator<Item = Token>> Parser<'input, I> {
    pub fn binding(&mut self) -> ParseResult<Binding> {
        let mutable = self.consume_at(TokenType::Mut);

        let name = self.ident()?;

        let type_annotation = if self.consume_at(TokenType::Colon) {
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

    pub fn type_(&mut self) -> ParseResult<Type> {
        Ok(match self.peek() {
            TokenType::Ident(_) => {
                let Some(Token {
                    ty: TokenType::Ident(name),
                    ..
                }) = self.next()
                else {
                    unreachable!()
                };

                let generics = if self.at(TokenType::LAngle) {
                    self.delimited_list(Self::type_, TokenType::LAngle, TokenType::RAngle)?
                } else {
                    Vec::new()
                };

                Type::Ident { name, generics }
            }
            TokenType::LBracket => {
                self.next();
                let inner_type = self.type_()?;
                self.consume(TokenType::RBracket)?;
                Type::Array(Box::new(inner_type))
            }
            TokenType::LParen => Type::Tuple(self.delimited_list(
                Self::type_,
                TokenType::LParen,
                TokenType::RParen,
            )?),
            TokenType::Fn => {
                self.next();
                let params =
                    self.delimited_list(Self::type_, TokenType::LParen, TokenType::RParen)?;
                self.consume(TokenType::Colon)?;
                let result = Box::new(self.type_()?);
                Type::Fn { params, result }
            }
            token => {
                return Err(ParseError::UnexpectedToken(
                    token.to_string(),
                    Some("start of type name".into()),
                ));
            }
        })
    }

    pub fn ident(&mut self) -> ParseResult<String> {
        match self.next() {
            Some(Token {
                ty: TokenType::Ident(name),
                ..
            }) => Ok(name),
            Some(token) => Err(ParseError::MismatchedToken {
                expected: TokenType::Ident(String::new()).to_string(),
                found: token.to_string(),
            }),
            None => Err(ParseError::MismatchedToken {
                expected: TokenType::Ident(String::new()).to_string(),
                found: TokenType::Eof.to_string(),
            }),
        }
    }

    pub fn delimited_list<T, F>(
        &mut self,
        mut f: F,
        start: TokenType,
        end: TokenType,
    ) -> ParseResult<(Vec<T>, Span)>
    where
        F: FnMut(&mut Self) -> ParseResult<T>,
    {
        let start = self.consume(start)?.span.start;

        let mut items = Vec::new();
        while !self.at(end) {
            items.push(f(self)?);

            if !self.consume_at(TokenType::Comma) {
                break;
            }
        }
        let end = self.consume(end)?.span.end;

        Ok((items, (start..end).into()))
    }
}
