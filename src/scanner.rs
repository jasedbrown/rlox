use crate::token::{Token, TokenType};
use crate::ErrorReporter;

use std::io::Result;

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
            .push(Token::simple_token(TokenType::Eof, self.line));

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
            '(' => self.tokens.push(Token::simple_token(LeftParen, self.line)),
            ')' => self.tokens.push(Token::simple_token(RightParen, self.line)),
            '{' => self.tokens.push(Token::simple_token(LeftBrace, self.line)),
            '}' => self.tokens.push(Token::simple_token(RightBrace, self.line)),
            ',' => self.tokens.push(Token::simple_token(Comma, self.line)),
            '.' => self.tokens.push(Token::simple_token(Dot, self.line)),
            '-' => self.tokens.push(Token::simple_token(Minus, self.line)),
            '+' => self.tokens.push(Token::simple_token(Plus, self.line)),
            ';' => self.tokens.push(Token::simple_token(Semicolon, self.line)),

            // one or two character lexemes
            '!' => {
                let tkn = if self.match_next(self, src, '=') {
                    BangEqual
                } else {
                    Bang
                };
                self.tokens.push(Token::simple_token(tkn, self.line));
            }
            _ => self
                .error_reporter
                .error(self.line, format!("unexpected character: {:?}", c).as_str()),
        }
    }

    /// Helper function to push the current index pointer into source along.
    fn advance(&mut self, src: &[char]) -> char {
        let c = src[self.current];
        self.current += 1;
        c
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
    fn at_end(&self, src: &Vec<char>) -> bool {
        self.current >= src.len()
    }
}
