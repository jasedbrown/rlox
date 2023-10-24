use crate::token::{Literal, Token, TokenType};
use crate::ErrorReporter;

use std::io::Result;

use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    error_reporter: ErrorReporter,

    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String, error_reporter: ErrorReporter) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            error_reporter,
            start: 0_usize,
            current: 0_usize,
            line: 1_u32,
        }
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    /// Scan for all the lexemes in the source.
    pub fn scan_tokens(&mut self) -> Result<()> {
        // this sucks, but i just want an index-addressable array
        let src: Vec<char> = self.source.as_str().chars().collect();

        while !self.at_end(&src) {
            // we're at the start of the next lexeme
            self.start = self.current;
            self.scan_token(&src);
        }

        // we're at the end, add the EOF
        self.tokens
            .push(Token::empty_token(TokenType::Eof, self.line));

        println!("**** tokens start ****");
        for t in &self.tokens {
            println!("{:?}", t);
        }
        println!("**** tokens end ****");

        Ok(())
    }

    /// Scan for the next lexeme in the source.
    fn scan_token(&mut self, src: &[char]) {
        use TokenType::*;
        let c = self.advance(src);
        match c {
            // one-character lexemes
            '(' => self.add_token(LeftParen, src),
            ')' => self.add_token(RightParen, src),
            '{' => self.add_token(LeftBrace, src),
            '}' => self.add_token(RightBrace, src),
            ',' => self.add_token(Comma, src),
            '.' => self.add_token(Dot, src),
            '-' => self.add_token(Minus, src),
            '+' => self.add_token(Plus, src),
            ';' => self.add_token(Semicolon, src),

            // one or two character lexemes
            '!' => {
                let tkn = if self.match_next(src, '=') {
                    BangEqual
                } else {
                    Bang
                };
                self.add_token(tkn, src);
            }
            '=' => {
                let tkn = if self.match_next(src, '=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.add_token(tkn, src);
            }
            '<' => {
                let tkn = if self.match_next(src, '=') {
                    LessEqual
                } else {
                    Less
                };
                self.add_token(tkn, src);
            }
            '>' => {
                let tkn = if self.match_next(src, '=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(tkn, src);
            }

            // comment ot division
            '/' => {
                if self.match_next(src, '/') {
                    while self.peek(src) != '\n' && !self.at_end(src) {
                        self.advance(src);
                    }
                } else {
                    self.add_token(Slash, src);
                }
            }

            // skip whitespace
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,

            // now we're onto handling literals
            '"' => self.string_literal(src),
            _ if c.is_ascii_digit() => self.number_literal(src),
            _ if Self::is_identifier_char(c) => self.identifier(src),

            _ => self
                .error_reporter
                .error(self.line, format!("unexpected character: {:?}", c).as_str()),
        }
    }

    /// Helper function to push the current index pointer into source along.
    fn string_literal(&mut self, src: &[char]) {
        while self.peek(src) != '"' && !self.at_end(src) {
            if self.peek(src) == '\n' {
                self.line += 1;
            }
            self.advance(src);
        }

        if self.at_end(src) {
            self.error_reporter.error(self.line, "unterminated string");
        }

        // account for the closing '"'
        self.advance(src);
        let s: String = src[self.start + 1..self.current - 1].iter().collect();
        self.add_literal_token(TokenType::String, Literal::StringLiteral(s), src);
    }

    fn number_literal(&mut self, src: &[char]) {
        let is_digit = |c: char| -> bool { c.is_ascii_digit() };

        while is_digit(self.peek(src)) {
            self.advance(src);
        }

        // look for a decimal
        if self.peek(src) == '.' && is_digit(self.peek_next(src)) {
            // consume the decimal ('.')
            self.advance(src);

            while is_digit(self.peek(src)) {
                self.advance(src);
            }
        }

        let s: String = src[self.start..self.current].iter().collect();
        let n = s.parse::<f64>().unwrap();
        self.add_literal_token(TokenType::Number, Literal::NumberLiteral(n), src);
    }

    fn identifier(&mut self, src: &[char]) {
        loop {
            let c = self.peek(src);
            if c.is_ascii_digit() || Self::is_identifier_char(c) {
                self.advance(src);
            } else {
                break;
            }
        }

        let lexeme: String = src[self.start..self.current].iter().collect();
        let token_type = match KEYWORDS.get(lexeme.as_str()) {
            Some(v) => v.clone(),
            None => TokenType::Identifier,
        };
        self.add_token(token_type, src);
    }

    fn add_token(&mut self, token_type: TokenType, src: &[char]) {
        let lexeme = src[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::simple_token(token_type, lexeme, self.line));
    }

    fn add_literal_token(&mut self, token_type: TokenType, literal: Literal, src: &[char]) {
        let lexeme = src[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::literal_token(token_type, lexeme, literal, self.line));
    }

    fn is_identifier_char(c: char) -> bool {
        c.is_alphabetic() || '_' == c
    }

    /// Helper function to push the current index pointer into source along.
    fn advance(&mut self, src: &[char]) -> char {
        let c = src[self.current];
        self.current += 1;
        c
    }

    /// Helper function to peek at the next char in the stream.
    fn peek(&mut self, src: &[char]) -> char {
        if self.at_end(src) {
            return '\0';
        }
        src[self.current]
    }

    /// Helper function to peek at the next-next char in the stream,
    /// that is, two chars ahead.
    fn peek_next(&mut self, src: &[char]) -> char {
        if self.current + 1 >= src.len() {
            return '\0';
        }
        src[self.current + 1]
    }

    fn match_next(&mut self, src: &[char], expected: char) -> bool {
        if self.at_end(src) {
            return false;
        }
        if src[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    /// Helper function to know if we're at the end of the source input.
    fn at_end(&self, src: &[char]) -> bool {
        self.current >= src.len()
    }
}
