mod ast;
mod expressions;
mod items;
mod helpers;
#[cfg(test)]
mod test;

use crate::lexer::{Lexer, Token};
use std::{error::Error, fmt::Display, iter::Peekable, mem};

#[derive(Debug)]
pub enum ParseError {
    MissingToken,
    MismatchedToken { expected: String, found: String },
    UnexpectedToken(String, Option<String>),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::MissingToken => write!(f, "expected another token"),
            ParseError::MismatchedToken { expected, found } => {
                write!(f, "expected token {expected}, found token {found}")
            }
            ParseError::UnexpectedToken(token, Some(desc)) => {
                write!(f, "unexpected token `{token}` at {desc}")
            }
            ParseError::UnexpectedToken(token, None) => write!(f, "unexpected token `{token:?}`"),
        }
    }
}

impl Error for ParseError {}

type ParseResult<T> = Result<T, ParseError>;

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
    pub(crate) fn at(&mut self, token: &Token) -> bool {
        mem::discriminant(self.peek()) == mem::discriminant(token)
    }

    pub(crate) fn consume_at(&mut self, token: &Token) -> bool {
        let at = self.at(token);
        if at {
            self.next();
        }
        at
    }

    /// Get the next token.
    pub(crate) fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    /// Move forward one token in the input and check
    /// that we pass the kind of token we expect.
    pub(crate) fn consume(&mut self, expected: &Token) -> ParseResult<()> {
        let token = self.next().ok_or(ParseError::MissingToken)?;
        if mem::discriminant(&token) == mem::discriminant(expected) {
            Ok(())
        } else {
            Err(ParseError::MismatchedToken {
                expected: expected.to_string(),
                found: token.to_string(),
            })
        }
    }
}
