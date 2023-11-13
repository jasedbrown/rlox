use anyhow::{anyhow, Result};

use crate::expr::{Expr, LiteralValue};
use crate::stmt::Stmt;
use crate::token::{Literal, Token, TokenType};
use crate::ErrorReporter;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    error_reporter: ErrorReporter,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>, error_reporter: ErrorReporter) -> Self {
        Parser {
            tokens,
            error_reporter,
            current: 0,
        }
    }

    /// Implements a recursive descent parser over the token stream.
    /// The hierarchy is as follows (from lower precedence to higher,
    /// from the top of the grammer to the lower):
    ///
    /// (Statement)
    /// (Expression)
    /// Equality
    /// Comparison
    /// Term (Addition)
    /// Factor (Multiplication)
    /// Unary
    /// (Primary)
    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while !self.at_end() {
            stmts.push(self.statement()?);
        }
        Ok(stmts)
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.matching(vec![TokenType::Print]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon);
        Ok(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon);
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        if self.matching(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        if self.matching(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        if self.matching(vec![TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        if self.matching(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.matching(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        let next = self.advance();
        let expr = match next.token_type {
            TokenType::False => Expr::Literal(LiteralValue::Boolean(false)),
            TokenType::True => Expr::Literal(LiteralValue::Boolean(true)),
            TokenType::Nil => Expr::Literal(LiteralValue::Nil()),

            TokenType::Number => {
                if let Some(Literal::NumberLiteral(n)) = next.literal {
                    Expr::Literal(LiteralValue::Number(n))
                } else {
                    return Err(anyhow!(
                        "unsupported literal type with Number token type: {:?}",
                        next
                    ));
                }
            }

            TokenType::String => {
                if let Some(Literal::StringLiteral(ref s)) = next.literal {
                    Expr::Literal(LiteralValue::String(s.clone()))
                } else {
                    return Err(anyhow!(
                        "unsupported literal type with Number token type: {:?}",
                        next
                    ));
                }
            }

            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen);
                Expr::Grouping(Box::new(expr))
            }

            _ => return Err(anyhow!("unsupported token type in primary(): {:?}", next)),
        };

        Ok(expr)
    }

    ///////////////////
    // helper functions
    ///////////////////

    fn matching(&mut self, token_types: Vec<TokenType>) -> bool {
        for t in token_types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, _token_type: TokenType) -> &Token {
        //if self.check(token_type) {
        return self.advance();
        //}
    }
}
