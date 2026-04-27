use anyhow::{Error, Result, anyhow};

use crate::oil::{
    expression::Expression,
    logger::Logger,
    object::Object,
    statement::Statement,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            current: 0,
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Vec<Statement>> {
        self.tokens = tokens;
        self.current = 0;
        let mut statements: Vec<Statement> = vec![];

        while !self.is_at_end() {
            let result = self.declaration();

            if result.is_err() {
                self.synchronize();
            } else {
                statements.push(result?);
            }
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Statement> {
        if self.match_next(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Statement> {
        let name = self
            .consume(TokenType::Identifier, "Expected a variable name.")?
            .clone();

        let mut initializer = None;

        if self.match_next(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        )?;

        Ok(Statement::var(name, initializer))
    }

    fn statement(&mut self) -> Result<Statement> {
        if self.match_next(&[TokenType::Print]) {
            self.print_statement()
        } else if self.match_next(&[TokenType::ReadInput]) {
            self.input_statement()
        } else if self.match_next(&[TokenType::If]) {
            self.if_statement()
        } else if self.match_next(&[TokenType::While]) {
            self.while_statement()
        } else if self.match_next(&[TokenType::For]) {
            self.for_statement()
        } else if self.match_next(&[TokenType::LeftBrace]) {
            self.block_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Statement> {
        let value = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected ';' after statement.")?;

        Ok(Statement::Print(value))
    }

    fn input_statement(&mut self) -> Result<Statement> {
        let target_var = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected ';' after statement.")?;

        match target_var {
            Expression::Variable { name } => Ok(Statement::ReadInput { target_var: name }),
            _ => Err(anyhow!("'read_input' expects a var to read into.")),
        }
    }

    fn if_statement(&mut self) -> Result<Statement> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after condition.")?;

        let then_branch = self.statement()?;
        let mut else_branch = None;

        if self.match_next(&[TokenType::Else]) {
            else_branch = Some(self.statement()?);
        }

        Ok(Statement::if_statement(condition, then_branch, else_branch))
    }

    fn while_statement(&mut self) -> Result<Statement> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after condition.")?;
        let body = self.statement()?;

        Ok(Statement::while_statement(condition, body))
    }

    fn for_statement(&mut self) -> Result<Statement> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'for'.")?;

        let mut initializer = None;
        let mut condition = None;
        let mut increment = None;

        if self.match_next(&[TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else if !self.match_next(&[TokenType::Semicolon]) {
            initializer = Some(self.expression_statement()?);
        }

        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, "Expected ';' after loop condition.")?;

        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }

        self.consume(TokenType::RightParen, "Expected ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(incr) = increment {
            body = Statement::Block(vec![body, Statement::Expression(incr)]);
        }

        if let Some(cond) = condition {
            body = Statement::while_statement(cond, body);
        }

        if let Some(init) = initializer {
            body = Statement::Block(vec![init, body])
        }

        Ok(body)
    }

    fn block_statement(&mut self) -> Result<Statement> {
        let mut statements = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block.")?;

        Ok(Statement::Block(statements))
    }

    fn expression_statement(&mut self) -> Result<Statement> {
        let value = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected ';' after statement.")?;

        Ok(Statement::Expression(value))
    }

    fn expression(&mut self) -> Result<Expression> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression> {
        let expression = self.or()?;

        if self.match_next(&[TokenType::Equal]) {
            let value = self.assignment()?;

            return match expression {
                Expression::Variable { name } => Ok(Expression::assign(name, value)),
                _ => Err(Self::error(self.previous(), "Invalid assignment target.")),
            };
        }

        Ok(expression)
    }

    fn or(&mut self) -> Result<Expression> {
        let mut expr = self.and()?;

        while self.match_next(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;

            expr = Expression::logical(expr, operator, right);
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expression> {
        let mut expr = self.equality()?;

        while self.match_next(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;

            expr = Expression::logical(expr, operator, right);
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression> {
        let mut expr = self.comparison()?;

        while self.match_next(&[TokenType::NotEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Expression::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression> {
        let mut expr = self.term()?;

        while self.match_next(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expr = Expression::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression> {
        let mut expr = self.factor()?;

        while self.match_next(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expr = Expression::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression> {
        let mut expr = self.unary()?;

        while self.match_next(&[TokenType::Slash, TokenType::Asterisk, TokenType::Mod]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Expression::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression> {
        if self.match_next(&[TokenType::Not, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            return Ok(Expression::unary(operator, right));
        }

        self.primary()
    }

    // obviously this should be using a match, i just don't wanna think about it
    fn primary(&mut self) -> Result<Expression> {
        if self.match_next(&[TokenType::False]) {
            Ok(Expression::literal(Object::Bool(false)))
        } else if self.match_next(&[TokenType::True]) {
            Ok(Expression::literal(Object::Bool(true)))
        } else if self.match_next(&[TokenType::Null]) {
            Ok(Expression::literal(Object::Null))
        } else if self.match_next(&[TokenType::Number, TokenType::String]) {
            let literal = self.previous().literal.clone();

            Ok(Expression::literal(
                literal.expect("Failed to extract literal!"),
            ))
        } else if self.match_next(&[TokenType::Identifier]) {
            Ok(Expression::variable(self.previous().clone()))
        } else if self.match_next(&[TokenType::LeftParen]) {
            let expr = self.expression()?;

            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;

            Ok(Expression::grouping(expr))
        } else {
            Err(Self::error(self.peek(), "Expected expression."))
        }
    }

    fn match_next(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(*token_type) {
                self.advance();

                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self
            .tokens
            .get(self.current)
            .expect("Failed to peek at next token!")
    }

    fn previous(&self) -> &Token {
        &self
            .tokens
            .get(self.current - 1)
            .expect("Failed to get previous token!")
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(Self::error(self.peek(), message))
        }
    }

    fn error(token: &Token, message: &str) -> Error {
        Logger::token_error(token, message);

        anyhow!("Failed to parse token.")
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            let found_boundary = match self.peek().token_type {
                TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print => true,
                _ => false,
            };

            if found_boundary {
                return;
            }

            self.advance();
        }
    }
}
