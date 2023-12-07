#![allow(dead_code)]
use std::{
    fmt::{Display, Formatter, Result},
    hash::{Hash, Hasher},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TokenType {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Clone, Debug)]
pub enum Literal {
    StringLiteral(String),
    NumberLiteral(f64),
}

impl Hash for Literal {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self {
            Literal::StringLiteral(s) => s.hash(state),
            // this is complete bullshit, but as f64 doesn't support hash,
            // stoopidly casting to i64 gets us through for this application.
            Literal::NumberLiteral(n) => ((n * 1000000.0) as i64).hash(state),
        }
    }
}

impl Eq for Literal {}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::StringLiteral(s), Literal::StringLiteral(o)) => s == o,
            // this is slightly half-assed, directly comparing two f64's,
            // but good enough for this application.
            (Literal::NumberLiteral(s), Literal::NumberLiteral(o)) => s == o,
            _ => false,
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Literal::StringLiteral(s) => write!(f, "{}", s),
            Literal::NumberLiteral(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<Literal>,
    line: u32,
}

impl Token {
    pub(crate) fn empty_token(token_type: TokenType, line: u32) -> Self {
        Token {
            token_type,
            lexeme: "".to_string(),
            literal: None,
            line,
        }
    }

    pub(crate) fn simple_token(token_type: TokenType, lexeme: String, line: u32) -> Self {
        Token {
            token_type,
            lexeme,
            literal: None,
            line,
        }
    }

    pub(crate) fn literal_token(
        token_type: TokenType,
        lexeme: String,
        literal: Literal,
        line: u32,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal: Some(literal),
            line,
        }
    }
}
