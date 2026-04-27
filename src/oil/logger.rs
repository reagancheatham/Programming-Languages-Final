use crate::oil::token::{Token, TokenType};

pub struct Logger;

impl Logger {
    pub fn error(line: usize, message: &str) {
        Self::report(line, "", message);
    }

    pub fn token_error(token: &Token, message: &str) {
        if token.token_type == TokenType::EOF {
            Self::report(token.line, "end", message);
        } else {
            Self::report(token.line, &token.lexeme, message);
        }
    }

    fn report(line: usize, place: &str, message: &str) {
        eprintln!("[line {line}] Error at {place}: {message}");
    }
}
