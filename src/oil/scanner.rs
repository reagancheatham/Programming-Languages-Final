use anyhow::{Result, anyhow};
use phf::phf_map;

use crate::oil::{
    logger::Logger,
    object::Object,
    token::{Token, TokenType},
};

const END_OF_LINE: char = '\n';
const END_OF_FILE: char = '\0';
const KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "func" => TokenType::Func,
    "if" => TokenType::If,
    "null" => TokenType::Null,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "read_input" => TokenType::ReadInput,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While
};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    had_error: bool,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            source: "".to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
            had_error: false,
        }
    }

    pub fn scan_tokens(&mut self, source: &str) -> Result<Vec<Token>> {
        self.source = source.to_string();
        self.tokens.clear();
        self.start = 0;
        self.current = 0;
        self.line = 0;
        self.had_error = false;

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "", Object::Null, self.line));

        if self.had_error {
            Err(anyhow!("Scanner failed to scan tokens!"))
        } else {
            Ok(std::mem::take(&mut self.tokens))
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        let token = match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Period),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '*' => Some(TokenType::Asterisk),
            '/' => {
                if self.match_next('/') {
                    while self.peek() != END_OF_LINE && !self.is_at_end() {
                        self.advance();
                    }

                    None
                } else {
                    Some(TokenType::Slash)
                }
            }
            '%' => Some(TokenType::Mod),
            '!' => Some(if self.match_next('=') {
                TokenType::NotEqual
            } else {
                TokenType::Not
            }),
            '=' => Some(if self.match_next('=') {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            }),
            '<' => Some(if self.match_next('=') {
                TokenType::LessEqual
            } else {
                TokenType::Less
            }),
            '>' => Some(if self.match_next('=') {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            }),
            ' ' => None,
            '\r' => None,
            '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }
            '"' => {
                self.handle_string();
                None
            }
            default_char => {
                if default_char.is_ascii_digit() {
                    self.handle_number();
                    None
                } else if default_char.is_alphabetic() || default_char == '_' {
                    self.handle_identifier();
                    None
                } else {
                    self.error(&format!("Unexpected character: {default_char}"));
                    None
                }
            }
        };

        if let Some(token) = token {
            self.add_token(token);
        }
    }

    fn advance(&mut self) -> char {
        let current = self.current;
        self.current += 1;

        self.source
            .chars()
            .nth(current)
            .expect("Failed to advance in source file!")
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Object::Null);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Object) {
        let text = self.source[self.start..self.current].to_string();

        self.tokens
            .push(Token::new(token_type, &text, literal, self.line));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let current_char = self
            .source
            .chars()
            .nth(self.current)
            .expect("Failed to match next character!");

        if current_char != expected {
            false
        } else {
            self.current += 1;

            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            END_OF_FILE
        } else {
            self.source
                .chars()
                .nth(self.current)
                .expect("Failed to peek at next character!")
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            END_OF_FILE
        } else {
            self.source
                .chars()
                .nth(self.current + 1)
                .expect("Failed to peek next!")
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == END_OF_LINE {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated string.");
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_literal(TokenType::String, Object::Str(value));
    }

    fn handle_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current]
            .parse::<f64>()
            .expect("Failed to parse number value!");
        self.add_token_with_literal(TokenType::Number, Object::Num(value));
    }

    fn handle_identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let value = &self.source[self.start..self.current];
        let token_type = match KEYWORDS.get(value) {
            None => TokenType::Identifier,
            Some(v) => *v,
        };

        self.add_token(token_type);
    }

    fn error(&mut self, message: &str) {
        self.had_error = true;
        Logger::error(self.line, message);
    }
}
