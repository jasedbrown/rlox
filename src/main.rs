#![allow(dead_code)]
use std::cell::Cell;
use std::env;
use std::fs;
use std::io::{stdin, Result};
use std::process;

#[derive(Eq, PartialEq, Debug)]
enum TokenType {
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
struct Token {
    token_type: TokenType,
    lexeme: Option<String>,
    literal: Option<String>, // not sure what the real type is just yet ....
    line: u32,
}

impl Token {
    fn simple_token(token_type: TokenType, line: u32) -> Self {
        Token {
            token_type,
            lexeme: None,
            literal: None,
            line,
        }
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    error_reporter: ErrorReporter,

    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    fn new(source: String, error_reporter: ErrorReporter) -> Self {
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
    fn scan_tokens(&mut self) -> Result<()> {
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
            //                '*' => self.tokens.push(Token::simple_token(Star, line)),
            _ => self
                .error_reporter
                .error(self.line, format!("unexpected character: {:?}", c).as_str()),
        }
    }

    fn simple_token(&mut self, src: &[char], token_type: TokenType) {
        // hmm, this is not great, but a starting point ....
        let lexeme: String = src[self.start..self.current].iter().collect();

        self.tokens.push(Token {
            token_type,
            lexeme: Some(lexeme),
            literal: None,
            line: self.line,
        });
    }

    /// Helper function to push the current index pointer into source along.
    fn advance(&mut self, src: &[char]) -> char {
        let c = src[self.current];
        self.current += 1;
        c
    }

    /// Helper function to know if we're at the end of the source input.
    fn at_end(&self, src: &Vec<char>) -> bool {
        self.current >= src.len()
    }
}

/// A centralized error reporting struct. Should be passed around to all
/// the workers in this project.
#[derive(Default, Clone, Debug)]
struct ErrorReporter {
    // TODO(jeb): Not sure if Cell is the best here, but it's
    // at least some form of interior mutability (yay!)
    had_error: Cell<bool>,
}

impl ErrorReporter {
    fn error(&self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&self, line: u32, place: &str, message: &str) {
        println!("[line {:?}] Error {:?}: {:?}", line, place, message);
        self.had_error.replace(true);
    }

    fn reset(&self) {
        self.had_error.replace(false);
    }

    fn had_error(&self) -> bool {
        self.had_error.get()
    }
}

/// The main struct for doing all the things for this project.
struct RLox {
    error_reporter: ErrorReporter,
}

impl RLox {
    fn new(error_reporter: ErrorReporter) -> Self {
        RLox { error_reporter }
    }

    fn run_prompt(&self) -> Result<()> {
        println!("JEB::run_prompt HEAD");
        loop {
            println!("\nenter some program: ");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("failed to read line");
            let input = input.as_str().trim();

            if input.is_empty() {
                return Ok(());
            }

            self.run(input)?;

            // reset the has_error on each run...
            self.error_reporter.reset();
        }
    }

    fn run(&self, input: &str) -> Result<()> {
        println!("... run, run ... :: {:?}", input);
        let mut scanner = Scanner::new(input.to_string(), self.error_reporter.clone());
        scanner.scan_tokens()?;
        Ok(())
    }

    fn run_file(&self, filename: &str) -> Result<()> {
        match fs::read_to_string(filename) {
            Ok(s) => self.run(s.as_str()),
            Err(e) => panic!("failed to read file: {:?}", e),
        }
    }
}

fn main() {
    let mut env_args: Vec<String> = env::args().collect();
    // ignore the first arg (it's the standard unix nmame of process)
    env_args.remove(0);

    let error_reporter = ErrorReporter::default();
    let rlox = RLox::new(error_reporter.clone());

    let _ = match env_args.len() {
        0 => rlox.run_prompt(),
        1 => rlox.run_file(&env_args[0]),
        _ => {
            println!("Usage: rlox [script]");
            process::exit(64);
        }
    };

    // Not sure if this is cool with the clone, but :shrug: for now
    if error_reporter.had_error() {
        process::exit(65);
    }
}
