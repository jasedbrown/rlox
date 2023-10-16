#![allow(dead_code)]
#[derive(Eq, PartialEq, Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Literal {
    StringLiteral(String),
    NumberLiteral(f64),
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: u32,
}

impl Token {
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
