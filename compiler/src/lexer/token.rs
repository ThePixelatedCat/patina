use crate::span::{Spannable, Spanned};
use std::fmt::Display;

pub type Token = Spanned<TokenType>;

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} from {} to {}",
            self.inner, self.span.start, self.span.end
        )
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenType {
    // Literals
    IntLit,
    FloatLit,
    StringLit,
    CharLit,
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    // Symbols
    Eq,
    Ampersand,
    Pipe,
    Bang,
    LAngle,
    RAngle,
    Plus,
    Minus,
    Times,
    FSlash,
    BSlash,
    Dot,
    Comma,
    Colon,
    Semicolon,
    Underscore,
    Arrow,
    // Operators
    Exponent,
    And,
    Or,
    Xor,
    Eqq,
    Neq,
    Leq,
    Geq,
    // Keywords
    Let,
    Mut,
    Const,
    Fn,
    Struct,
    Enum,
    If,
    Else,
    Match,
    True,
    False,
    // Misc
    Ident,
    Error,
    Eof,
}

impl Spannable for TokenType {}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::IntLit => "int literal",
                TokenType::FloatLit => "float literal",
                TokenType::StringLit => "string literal",
                TokenType::CharLit => "char literal",
                TokenType::LParen => "(",
                TokenType::RParen => ")",
                TokenType::LBrace => "{",
                TokenType::RBrace => "}",
                TokenType::LBracket => "[",
                TokenType::RBracket => "]",
                TokenType::Eq => "=",
                TokenType::Ampersand => "&",
                TokenType::Pipe => "|",
                TokenType::Bang => "!",
                TokenType::LAngle => "<",
                TokenType::RAngle => ">",
                TokenType::Plus => "+",
                TokenType::Minus => "-",
                TokenType::Times => "*",
                TokenType::FSlash => "/",
                TokenType::BSlash => "\\",
                TokenType::Dot => ".",
                TokenType::Comma => ",",
                TokenType::Colon => ":",
                TokenType::Semicolon => ";",
                TokenType::Underscore => "_",
                TokenType::Arrow => "->",
                TokenType::Exponent => "**",
                TokenType::And => "&&",
                TokenType::Or => "||",
                TokenType::Xor => "^",
                TokenType::Eqq => "==",
                TokenType::Neq => "!=",
                TokenType::Leq => "<=",
                TokenType::Geq => ">=",
                TokenType::Let => "let",
                TokenType::Mut => "mut",
                TokenType::Const => "const",
                TokenType::Fn => "fn",
                TokenType::Struct => "struct",
                TokenType::Enum => "enum",
                TokenType::If => "if",
                TokenType::Else => "else",
                TokenType::Match => "match",
                TokenType::True => "true",
                TokenType::False => "false",
                TokenType::Ident => "identifier",
                TokenType::Error => "ERROR",
                TokenType::Eof => "eof",
            }
        )
    }
}
