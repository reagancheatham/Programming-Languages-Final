use std::fmt::Display;

use crate::oil::object::Object;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Period,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Asterisk,
    Mod,

    Not,
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,
    Func,
    For,
    If,
    Null,
    Or,
    Print,
    ReadInput,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Object, line: usize) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal: Some(literal),
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.literal {
            None => write!(f, "{}", self.lexeme),
            Some(obj) => match obj {
                Object::Null => write!(f, "{}", self.lexeme),
                default => write!(f, "{default}"),
            },
        }
    }
}
