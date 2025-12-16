mod ast;
mod expressions;
mod hierarchy;
#[cfg(test)]
mod test;

use std::mem::discriminant;
use std::{iter::Peekable, mem::Discriminant};

use crate::lexer::{Lexer, Token};

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
    pub(crate) fn at(&mut self, other: Discriminant<Token>) -> bool {
        discriminant(self.peek()) == other
    }

    /// Get the next token.
    pub(crate) fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    /// Move forward one token in the input and check
    /// that we pass the kind of token we expect.
    pub(crate) fn consume(&mut self, expected: Discriminant<Token>) {
        let token = self.next().unwrap_or_else(|| {
            panic!("Expected to consume `{expected:?}`, but there was no next token")
        });
        assert_eq!(
            discriminant(&token),
            expected,
            "Expected to consume `{:?}`, but found `{:?}`",
            expected,
            token
        );
    }
}
