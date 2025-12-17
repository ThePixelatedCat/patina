use crate::parser::{IDENT_DISCRIM, ast::Field};

use super::{ParseError, ParseResult, Parser, Token, ast::Item};
use crate::next_checked;

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn item(&mut self) -> ParseResult<Item> {
        Ok(match self.peek() {
            Token::Fn => todo!(),
            Token::Struct => {
                self.next();

                let name = next_checked!(self, Token::Ident, *IDENT_DISCRIM);

                let mut fields = Vec::new();

                self.consume(Token::LBrace.ty())?;
                while !self.at(Token::RBrace.ty()) {
                    let name = next_checked!(self, Token::Ident, *IDENT_DISCRIM);

                    self.consume(Token::Colon.ty())?;

                    let ty = next_checked!(self, Token::Ident, *IDENT_DISCRIM);

                    fields.push(Field { name, ty });
                    if self.at(Token::Comma.ty()) {
                        self.next();
                    }
                }
                self.consume(Token::RBrace.ty())?;
                Item::Struct { name, fields }
            }
            token => {
                return Err(ParseError::UnexpectedToken(
                    token.ty(),
                    Some("start of item".into()),
                ));
            }
        })
    }
}
