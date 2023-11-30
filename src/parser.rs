use anyhow::{anyhow, Result};

use crate::expr::{Expr, LiteralValue};
use crate::stmt::Stmt;
use crate::token::{Literal, Token, TokenType};
use crate::ErrorReporter;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    _error_reporter: ErrorReporter,
    current: usize,
}

#[allow(dead_code)]
enum FunctionKind {
    Function,
    #[allow(dead_code)]
    Method,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>, error_reporter: ErrorReporter) -> Self {
        Parser {
            tokens,
            _error_reporter: error_reporter,
            current: 0,
        }
    }

    /// Implements a recursive descent parser over the token stream.
    /// The hierarchy is as follows (from lower precedence to higher,
    /// from the top of the grammer to the lower):
    ///
    /// Declaration
    /// (Statement)
    /// (Expression)
    /// assignment
    /// Equality
    /// Comparison
    /// Term (Addition)
    /// Factor (Multiplication)
    /// Unary
    /// (Primary)
    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while !self.at_end() {
            stmts.push(self.declaration()?);
        }
        Ok(stmts)
    }

    fn declaration(&mut self) -> Result<Stmt> {
        if self.matching(vec![TokenType::Fun]) {
            return self.function(FunctionKind::Function);
        }
        if self.matching(vec![TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn function(&mut self, _kind: FunctionKind) -> Result<Stmt> {
        let name = self.consume(TokenType::Identifier).clone();

        self.consume(TokenType::LeftParen);
        let mut params = Vec::new();

        // check for zero params
        if !self.check(TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    return Err(anyhow!(
                        "cannot have more than 255 parameters on a function"
                    ));
                }

                params.push(self.consume(TokenType::Identifier).clone());

                if !self.matching(vec![TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen);

        // now, on to the body of the function
        self.consume(TokenType::LeftBrace);
        // there's a better a way to do this, i am sure ...
        match self.block()? {
            Stmt::Block(v) => Ok(Stmt::Function {
                name,
                params,
                body: v,
            }),
            _ => unreachable!("wtf?!?!"),
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume(TokenType::Identifier).clone();
        let has_initializer = &self.matching(vec![TokenType::Equal]);

        let initializer = if *has_initializer {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon);
        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.matching(vec![TokenType::For]) {
            return self.for_statement();
        } else if self.matching(vec![TokenType::If]) {
            return self.if_statement();
        } else if self.matching(vec![TokenType::Print]) {
            return self.print_statement();
        } else if self.matching(vec![TokenType::While]) {
            return self.while_statement();
        } else if self.matching(vec![TokenType::LeftBrace]) && !self.at_end() {
            return self.block();
        }

        self.expression_statement()
    }

    fn for_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen);

        let initializer = if self.matching(vec![TokenType::Semicolon]) {
            None
        } else if self.matching(vec![TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        // default the condition to true if None was provided
        let condition = if self.check(TokenType::Semicolon) {
            Expr::Literal(LiteralValue::Boolean(true))
        } else {
            self.expression()?
        };
        self.consume(TokenType::Semicolon);

        let increment = if self.check(TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::RightParen);

        let mut body = self.statement()?;

        //desugar the syntax
        // put the increment after the body
        if let Some(incr) = increment {
            body = Stmt::Block(vec![body, Stmt::Expression(incr)]);
        };

        // put the conditional up front, and make it a while loop
        body = Stmt::While {
            condition,
            body: Box::new(body),
        };

        if let Some(init) = initializer {
            body = Stmt::Block(vec![init, body]);
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen);
        let condition = self.expression()?;
        self.consume(TokenType::RightParen);

        let then_branch = Box::new(self.statement()?);
        let else_branch = match self.matching(vec![TokenType::Else]) {
            true => Some(Box::new(self.statement()?)),
            false => None,
        };
        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon);
        Ok(Stmt::Print(value))
    }

    fn while_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen);
        let condition = self.expression()?;
        self.consume(TokenType::RightParen);
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { condition, body })
    }

    fn block(&mut self) -> Result<Stmt> {
        let mut stmts = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.at_end() {
            stmts.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace);
        Ok(Stmt::Block(stmts))
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon);
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.or()?;

        if self.matching(vec![TokenType::Equal]) {
            let _equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Variable(t) => {
                    return Ok(Expr::Assign(t, Box::new(value)));
                }
                _ => {
                    return Err(anyhow!("Invalid assignment target"));
                }
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr> {
        let mut expr = self.and()?;

        while self.matching(vec![TokenType::Or]) {
            let operator = self.previous().clone();
            let right = Box::new(self.and()?);
            expr = Expr::Logical(Box::new(expr), operator, right)
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while self.matching(vec![TokenType::And]) {
            let operator = self.previous().clone();
            let right = Box::new(self.equality()?);
            expr = Expr::Logical(Box::new(expr), operator, right);
        }

        Ok(expr)
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

        self.call()
    }

    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.matching(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr> {
        let mut args = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.matching(vec![TokenType::Comma]) {
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RightParen).clone();

        Ok(Expr::Call(Box::new(callee), paren, args))
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

            TokenType::Var => Expr::Variable(self.previous().clone()),
            TokenType::Identifier => Expr::Variable(self.previous().clone()),

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
