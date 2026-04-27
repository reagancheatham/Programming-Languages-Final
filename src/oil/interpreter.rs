use anyhow::{Result, anyhow};

use crate::oil::{
    expression::Expression,
    logger::Logger,
    object::Object,
    token::{Token, TokenType},
};

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(expr: &Expression) {
        let value = Self::evaluate(expr);

        match value {
            Ok(v) => println!("{}", v.to_string()),
            Err(_) => {}
        };
    }

    fn evaluate(expr: &Expression) -> Result<Object> {
        match expr {
            Expression::Literal { value } => Ok(value.clone()),
            Expression::Grouping { expression } => Self::evaluate(&expression),
            Expression::Unary { operator, right } => Self::unary(operator, right),
            Expression::Binary {
                left,
                operator,
                right,
            } => Self::binary(left, operator, right),
        }
    }

    fn unary(operator: &Token, right: &Expression) -> Result<Object> {
        let right_object = Self::evaluate(right)?;

        match operator.token_type {
            TokenType::Not => Ok(Object::Bool(Self::is_truthy(&right_object))),
            TokenType::Minus => {
                if let Object::Num(num) = right_object {
                    Ok(Object::Num(-num))
                } else {
                    Logger::error(operator.line, "Failed to parse number from unary operator.");
                    Err(anyhow!("Failed to parse number from unary operator."))
                }
            }
            _ => {
                Logger::error(operator.line, "Failed to reach value for unary operator.");
                Err(anyhow!("Failed to reach value for unary operator"))
            }
        }
    }

    fn binary(left: &Expression, operator: &Token, right: &Expression) -> Result<Object> {
        let left_object = Self::evaluate(left)?;
        let right_object = Self::evaluate(right)?;

        // This works for any token type
        match operator.token_type {
            TokenType::NotEqual => {
                return Ok(Object::Bool(!Self::is_equal(&left_object, &right_object)));
            }
            TokenType::EqualEqual => {
                return Ok(Object::Bool(Self::is_equal(&left_object, &right_object)));
            }
            _ => {}
        };

        if let Object::Num(left_num) = left_object
            && let Object::Num(right_num) = right_object
        {
            match operator.token_type {
                TokenType::Greater => Ok(Object::Bool(left_num > right_num)),
                TokenType::GreaterEqual => Ok(Object::Bool(left_num >= right_num)),
                TokenType::Less => Ok(Object::Bool(left_num < right_num)),
                TokenType::LessEqual => Ok(Object::Bool(left_num <= right_num)),
                TokenType::Minus => Ok(Object::Num(left_num - right_num)),
                TokenType::Plus => Ok(Object::Num(left_num + right_num)),
                TokenType::Slash => Ok(Object::Num(left_num / right_num)),
                TokenType::Asterisk => Ok(Object::Num(left_num * right_num)),
                _ => {
                    Logger::error(operator.line, "Could not parse binary operator for number.");
                    Err(anyhow!("Could not parse binary operator for number."))
                }
            }
        } else if let Object::Str(left_str) = left_object
            && let Object::Str(right_str) = right_object
        {
            match operator.token_type {
                TokenType::Plus => Ok(Object::Str(format!("{left_str}{right_str}"))),
                _ => {
                    Logger::error(operator.line, "Could not parse binary operator for string.");
                    Err(anyhow!("Could not parse binary operator for string."))
                }
            }
        } else {
            Logger::error(
                operator.line,
                "Tried to perform binary operation on invalid expressions!",
            );
            Err(anyhow!(
                "Tried to perform binary operation on invalid expressions!"
            ))
        }
    }

    fn is_truthy(object: &Object) -> bool {
        match object {
            Object::Null => false,
            Object::Bool(b) => *b,
            _ => true,
        }
    }

    fn is_equal(left: &Object, right: &Object) -> bool {
        match (left, right) {
            (Object::Null, Object::Null) => true,
            (Object::Bool(b1), Object::Bool(b2)) => *b1 == *b2,
            (Object::Num(n1), Object::Num(n2)) => *n1 == *n2,
            (Object::Str(s1), Object::Str(s2)) => *s1 == *s2,
            _ => false,
        }
    }
}
