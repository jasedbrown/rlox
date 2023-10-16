#![allow(dead_code)]
#[derive(Eq, PartialEq, Debug)]
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

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: Option<String>,
    literal: Option<String>, // not sure what the real type is just yet ....
    line: u32,
}

impl Token {
    pub(crate) fn simple_token(token_type: TokenType, line: u32) -> Self {
        Token {
            token_type,
            lexeme: None,
            literal: None,
            line,
        }
    }
}
