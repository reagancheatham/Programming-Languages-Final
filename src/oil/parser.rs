use anyhow::{Error, Result, anyhow};

use crate::oil::{
    expression::Expression,
    logger::Logger,
    object::Object,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expression> {
        self.expression().ok()
    }

    fn expression(&mut self) -> Result<Expression> {
        self.equality()
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

        while self.match_next(&[TokenType::Slash, TokenType::Asterisk]) {
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
        } else if self.match_next(&[TokenType::LeftParen]) {
            let expr = self.expression()?;

            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;

            Ok(Expression::grouping(expr))
        } else {
            Err(Parser::error(self.peek(), "Expected expression."))
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
            Err(Parser::error(self.peek(), message))
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
                TokenType::Class
                | TokenType::Func
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => true,
                _ => false,
            };

            if found_boundary {
                return;
            }

            self.advance();
        }
    }
}
