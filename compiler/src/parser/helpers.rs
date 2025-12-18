use crate::parser::ParseError;

use super::{
    ParseResult, Parser, Token,
    ast::{Binding, Type},
};

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn binding(&mut self) -> ParseResult<Binding> {
        let mutable = self.consume_at(&Token::Mut);

        let name = self.ident()?;

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

    pub fn type_(&mut self) -> ParseResult<Type> {
        let name = self.ident()?;

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

    pub fn ident(&mut self) -> ParseResult<String> {
        match self.next() {
            Some(Token::Ident(ident)) => Ok(ident),
            Some(token) => Err(ParseError::MismatchedToken {
                expected: Token::Ident(String::new()).to_string(),
                found: token.to_string(),
            }),
            None => Err(ParseError::MissingToken),
        }
    }
}
