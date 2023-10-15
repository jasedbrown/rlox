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
    // literal: Object ????
    line: u32,
}

impl Token {
    fn simple_token(token_type: TokenType, line: u32) -> Self {
        Token {
            token_type,
            lexeme: None,
            line,
        }
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    error_reporter: ErrorReporter,
}

impl Scanner {
    fn new(source: String, error_reporter: ErrorReporter) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            error_reporter,
        }
    }

    fn scan_tokens(&mut self) -> Result<()> {
        use TokenType::*;

        let start: usize = 0;
        let current: usize = 0;
        let line: u32 = 0;

        let src = self.source.as_str().chars();
        for c in src {
            match c {
                // one-character lexemes
                '(' => self.tokens.push(Token::simple_token(LeftParen, line)),
                ')' => self.tokens.push(Token::simple_token(RightParen, line)),
                '{' => self.tokens.push(Token::simple_token(LeftBrace, line)),
                '}' => self.tokens.push(Token::simple_token(RightBrace, line)),
                ',' => self.tokens.push(Token::simple_token(Comma, line)),
                '.' => self.tokens.push(Token::simple_token(Dot, line)),
                '-' => self.tokens.push(Token::simple_token(Minus, line)),
                '+' => self.tokens.push(Token::simple_token(Plus, line)),
                ';' => self.tokens.push(Token::simple_token(Semicolon, line)),
                '*' => self.tokens.push(Token::simple_token(Star, line)),

                // one or two character lexemes
                '*' => self.tokens.push(Token::simple_token(Star, line)),

                _ => self
                    .error_reporter
                    .error(line, format!("unexpected character: {:?}", c).as_str()),
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: None,
            line,
        });

        Ok(())
    }
}

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
