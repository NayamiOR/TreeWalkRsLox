use crate::expr::Expr;
use crate::stmt::LoxFunctionNode;
use crate::stmt::Stmt;
use crate::token::{Literal, Token};
use crate::token_type::TokenType;
use crate::token_type::TokenType::*;
use crate::Lox;

pub(crate) struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub(crate) fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_token(&[VAR]) {
            match self.var_declaration() {
                Ok(stmt) => {
                    return Some(stmt);
                }
                Err(_) => self.synchronize(),
            }
        }
        if self.match_token(&[FUN]) {
            return Some(self.function("function".to_string()).unwrap());
        }

        match self.statement() {
            Ok(stmt) => Some(stmt),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;
        if self.match_token(&[EQUAL]) {
            let equals = self.previous();
            let value = self.assignment()?;
            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }
            return Err(Self::error(
                equals,
                "Invalid assignment target.".to_string(),
            ));
        }
        Ok(expr)
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[IF]) {
            return self.if_statement();
        }
        if self.match_token(&[PRINT]) {
            return self.print_statement();
        }
        if self.match_token(&[RETURN]) {
            return self.return_statement();
        }
        if self.match_token(&[LEFT_BRACE]) {
            return Ok(Stmt::Block {
                statements: self.block()?,
            });
        }
        if self.match_token(&[WHILE]) {
            return self.while_statement();
        }
        if self.match_token(&[FOR]) {
            return self.for_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(SEMICOLON, "Expect ';' after value.".to_string())?;
        Ok(Stmt::Print {
            expression: Box::new(value),
        })
    }

    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword: Token = self.previous();
        let mut value = Expr::Literal {
            value: Literal::Nil,
        };

        if !self.check(&SEMICOLON) {
            value = self.expression()?;
        }

        self.consume(SEMICOLON, "Expect ';' after return value.".to_string())?;

        Ok(Stmt::Return {
            keyword,
            value: Box::new(value),
        })
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name: Token = self.consume(IDENTIFIER, "Expect variable name.".to_string())?;
        let mut initializer = None;
        if self.match_token(&[EQUAL]) {
            initializer = Some(Box::new(self.expression()?));
        }
        self.consume(
            SEMICOLON,
            "Expect ';' after variable declaration.".to_string(),
        )?;
        Ok(Stmt::Var { name, initializer })
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(LEFT_PAREN, "Expect '(' after 'while'.".to_string())?;
        let condition = Box::new(self.expression()?);
        self.consume(RIGHT_PAREN, "Expect ')' after condition.".to_string())?;
        let body = Box::new(self.statement()?);

        Ok(Stmt::While { condition, body })
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        // start reading the for loop header

        self.consume(LEFT_PAREN, "Expect '(' after 'for'.".to_string())?;

        let initializer = if self.match_token(&[SEMICOLON]) {
            None
        } else if self.match_token(&[VAR]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(&SEMICOLON) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(SEMICOLON, "Expect ';' after loop condition.".to_string())?;
        let increment = if !self.check(&RIGHT_PAREN) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(RIGHT_PAREN, "Expect ')' after for clauses.".to_string())?;

        // end reading the for loop header

        // reading the for loop body

        let mut body = Box::new(self.statement()?);

        // desugar

        if let Some(increment) = increment {
            body = Box::new(Stmt::Block {
                statements: vec![
                    *body,
                    Stmt::Expression {
                        expression: Box::new(increment),
                    },
                ],
            });
        }

        /* Original:
            if (condition == null) condition = new Expr.Literal(true);
            body = new Stmt.While(condition, body);
        */

        body = Box::new(Stmt::While {
            condition: Box::new(condition.unwrap_or_else(|| Expr::Literal {
                value: Literal::Bool(true),
            })),
            body,
        });

        // if there is an initializer, wrap the body in a block with the initializer

        if let Some(initializer) = initializer {
            body = Box::new(Stmt::Block {
                statements: vec![initializer, *body],
            });
        }
        Ok(*body)
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(SEMICOLON, "Expect ';' after expression.".to_string())?;
        Ok(Stmt::Expression {
            expression: Box::new(expr),
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.check(&RIGHT_BRACE) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
        self.consume(RIGHT_BRACE, "Expect '}' after block.".to_string())?;
        Ok(statements)
    }

    fn function(&mut self, kind: String) -> Result<Stmt, ParseError> {
        let name: Token = self.consume(IDENTIFIER, format!("Expect {} name.", kind))?;
        self.consume(LEFT_PAREN, format!("Expect '(' after {} name.", kind))?;
        let mut params = Vec::new();
        if !self.check(&RIGHT_PAREN) {
            loop {
                if params.len() >= 255 {
                    Self::error(
                        self.peek(),
                        "Cannot have more than 255 parameters.".to_string(),
                    );
                }
                params.push(self.consume(IDENTIFIER, "Expect parameter name.".to_string())?);

                if !self.match_token(&[COMMA]) {
                    break;
                }
            }
        }
        self.consume(RIGHT_PAREN, "Expect ')' after parameters.".to_string())?;

        self.consume(LEFT_BRACE, format!("Expect '{{' before {} body.", kind))?;
        let body = self.block()?;
        Ok(Stmt::Function {
            function: Box::new(LoxFunctionNode { name, body, params }),
        })
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and();

        while self.match_token(&[OR]) {
            let operator = self.previous();
            let right = self.and();
            expr = Ok(Expr::Logical {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }
        expr
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality();

        while self.match_token(&[AND]) {
            let operator = self.previous();
            let right = self.equality();
            expr = Ok(Expr::Logical {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }
        expr
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison();
        while self.match_token(&[BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }
        expr
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term();
        while self.match_token(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.term();
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }
        expr
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor();
        while self.match_token(&[MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }
        expr
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary();
        while self.match_token(&[SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }
        expr
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut primary = self.primary();
        loop {
            if self.match_token(&[LEFT_PAREN]) {
                primary = self.finish_call(primary?);
            } else {
                break;
            }
        }

        primary
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments = Vec::new();
        if !self.check(&RIGHT_PAREN) {
            loop {
                if arguments.len() >= 255 {
                    Self::error(
                        self.peek(),
                        "Cannot have more than 255 arguments.".to_string(),
                    );
                }
                arguments.push(Box::new(self.expression()?));

                if !self.match_token(&[COMMA]) {
                    break;
                }
            }
        }
        let paren = self.consume(RIGHT_PAREN, "Expect ')' after arguments.".to_string())?;
        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[FALSE]) {
            return Ok(Expr::Literal {
                value: Literal::Bool(false),
            });
        }
        if self.match_token(&[TRUE]) {
            return Ok(Expr::Literal {
                value: Literal::Bool(true),
            });
        }
        if self.match_token(&[NIL]) {
            return Ok(Expr::Literal {
                value: Literal::Nil,
            });
        }
        if self.match_token(&[NUMBER, STRING]) {
            return Ok(Expr::Literal {
                value: self.previous().literal.clone().unwrap(),
            });
        }
        if self.match_token(&[IDENTIFIER]) {
            return Ok(Expr::Variable {
                name: self.previous(),
            });
        }
        if self.match_token(&[LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(RIGHT_PAREN, "Expect ')' after expression.".to_string())?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }
        Err(Self::error(self.peek(), "Expect expression.".to_string()))
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, ParseError> {
        if !self.check(&token_type) {
            return Err(Self::error(self.peek(), message));
        }
        Ok(self.advance())
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn error(token: Token, message: String) -> ParseError {
        Lox::error_at_token(token, message);
        ParseError
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == SEMICOLON {
                return;
            }
            match self.peek().token_type {
                CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => return,
                _ => (),
            }
            self.advance();
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(LEFT_PAREN, "Expect '(' after 'if'.".to_string())?;
        let condition = Box::new(self.expression()?);
        self.consume(RIGHT_PAREN, "Expect ')' after if condition.".to_string())?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token(&[ELSE]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }
}

#[derive(Debug)]
pub(crate) struct ParseError;
