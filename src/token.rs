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
    string_literal: Option<String>,
    number_literal: Option<f64>,
    line: u32,
}

impl Token {
    pub(crate) fn simple_token(token_type: TokenType, line: u32) -> Self {
        Self::new(token_type, None, None, line)
    }

    pub(crate) fn literal_token(token_type: TokenType, literal: String, line: u32) -> Self {
        Self::new(token_type, Some(literal), None, line)
    }

    pub(crate) fn number_token(token_type: TokenType, number: f64, line: u32) -> Self {
        Self::new(token_type, None, Some(number), line)
    }

    fn new(
        token_type: TokenType,
        string_literal: Option<String>,
        number_literal: Option<f64>,
        line: u32,
    ) -> Self {
        Token {
            token_type,
            lexeme: None,
            string_literal,
            number_literal,
            line,
        }
    }
}
