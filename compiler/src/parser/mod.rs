mod ast;
mod expressions;
mod items;
mod statements;
#[cfg(test)]
mod test;

use std::error::Error;
use std::fmt::Display;
use std::{iter::Peekable, mem::Discriminant};

use lazy_static::lazy_static;

use crate::lexer::{Lexer, Token};

type TokenType = Discriminant<Token>;
type ParseResult<T> = Result<T, ParseError>;

lazy_static! {
    static ref IDENT_DISCRIM: TokenType = Token::Ident("".into()).ty();
}

#[derive(Debug)]
pub enum ParseError {
    MissingToken,
    MismatchedToken {
        expected: TokenType,
        found: TokenType,
    },
    UnexpectedToken(TokenType, Option<String>),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::MissingToken => write!(f, "expected another token"),
            ParseError::MismatchedToken { expected, found } => {
                write!(f, "expected token {expected:?}, found token {found:?}")
            }
            ParseError::UnexpectedToken(token, Some(desc)) => {
                write!(f, "unexpected token `{token:?}` at {desc}")
            }
            ParseError::UnexpectedToken(token, None) => write!(f, "unexpected token `{token:?}`"),
        }
    }
}

impl Error for ParseError {}

pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekable<I>,
}

impl<'input> Parser<Lexer<'input>> {
    pub fn new(input: &'input str) -> Parser<Lexer<'input>> {
        Parser {
            tokens: Lexer::new(input).peekable(),
        }
    }
}

impl<I: Iterator<Item = Token>> Parser<I> {
    /// Look-ahead one token and see what kind of token it is.
    pub(crate) fn peek(&mut self) -> &Token {
        self.tokens.peek().unwrap_or(&Token::Eof)
    }

    /// Check if the next token is the same variant as another token.
    pub(crate) fn at(&mut self, other: TokenType) -> bool {
        self.peek().ty() == other
    }

    /// Get the next token.
    pub(crate) fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    /// Move forward one token in the input and check
    /// that we pass the kind of token we expect.
    pub(crate) fn consume(&mut self, expected: TokenType) -> ParseResult<()> {
        let token = self.next().ok_or(ParseError::MissingToken)?;
        if token.ty() != expected {
            Err(ParseError::MismatchedToken {
                expected,
                found: token.ty(),
            })
        } else {
            Ok(())
        }
    }
}

#[macro_export]
macro_rules! next_checked {
    ($self:ident, $type:ident :: $expect:ident, $discrim:expr) => {
        match $self.next() {
            Some($type::$expect(inner)) => inner,
            Some(token) => {
                return Err(ParseError::MismatchedToken {
                    expected: $discrim,
                    found: token.ty(),
                });
            }
            None => return Err(ParseError::MissingToken),
        }
    };
}
