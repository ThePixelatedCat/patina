mod ast;
mod expressions;
mod helpers;
mod items;
#[cfg(test)]
mod test;

use crate::lexer::{Lexer, Token, TokenType};
use std::{error::Error, fmt::Display, iter::Peekable};

#[derive(Debug)]
pub enum ParseTokenError {
    Mismatched {
        expected: TokenType,
        found: TokenType,
    },
    Unexpected(TokenType, Option<String>),
    Missing,
}

impl Display for ParseTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mismatched { expected, found } => {
                write!(f, "expected token {expected}, found token {found}")
            }
            Self::Unexpected(token, Some(desc)) => {
                write!(f, "unexpected token `{token}` at {desc}")
            }
            Self::Unexpected(token, None) => write!(f, "unexpected token `{token:?}`"),
            Self::Missing => "expected another token".fmt(f),
        }
    }
}

impl Error for ParseTokenError {}

type ParseResult<T> = Result<T, ParseTokenError>;

pub struct Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    input: &'input str,
    tokens: Peekable<I>,
}

impl<'input> Parser<'input, Lexer<'input>> {
    pub fn new(input: &'input str) -> Self {
        Parser {
            input,
            tokens: Lexer::new(input).peekable(),
        }
    }
}

impl<I: Iterator<Item = Token>> Parser<'_, I> {
    /// Look-ahead one token and see what kind of token it is.
    pub(crate) fn peek(&mut self) -> TokenType {
        self.tokens
            .peek()
            .map_or(TokenType::Eof, |token| token.inner)
    }

    /// Check if the next token is the same variant as another token.
    pub(crate) fn at(&mut self, token: TokenType) -> bool {
        self.peek() == token
    }

    pub(crate) fn consume_at(&mut self, token: TokenType) -> bool {
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
    pub(crate) fn consume(&mut self, expected: TokenType) -> ParseResult<Token> {
        let next = self.next().ok_or(ParseTokenError::Missing)?;
        if next.inner == expected {
            Ok(next)
        } else {
            Err(ParseTokenError::Mismatched {
                expected,
                found: next.inner,
            })
        }
    }
}
