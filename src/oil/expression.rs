#![allow(dead_code, unused)]

use crate::oil::{object::Object, token::Token};

type Expr = Box<Expression>;

#[derive(Clone, Debug)]
pub enum Expression {
    Assign {
        name: Token,
        value: Expr,
    },
    Binary {
        left: Expr,
        operator: Token,
        right: Expr,
    },
    Grouping {
        expression: Expr,
    },
    Unary {
        operator: Token,
        right: Expr,
    },
    Literal {
        value: Object,
    },
    Logical {
        left: Expr,
        operator: Token,
        right: Expr,
    },
    Variable {
        name: Token,
    },
}

impl Expression {
    pub fn assign(name: Token, value: Expression) -> Expression {
        Self::Assign {
            name,
            value: Box::new(value),
        }
    }

    pub fn binary(left: Expression, operator: Token, right: Expression) -> Expression {
        Self::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn grouping(expression: Expression) -> Expression {
        Self::Grouping {
            expression: Box::new(expression),
        }
    }

    pub fn unary(operator: Token, right: Expression) -> Expression {
        Self::Unary {
            operator,
            right: Box::new(right),
        }
    }

    pub fn literal(value: Object) -> Expression {
        Self::Literal { value }
    }

    pub fn logical(left: Expression, operator: Token, right: Expression) -> Expression {
        Self::Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn variable(name: Token) -> Expression {
        Self::Variable { name }
    }

    pub fn to_tree(&self) -> String {
        match self {
            Self::Assign { name, value } => Self::parenthesize(&name.lexeme, &[value]),
            Self::Binary {
                left,
                operator,
                right,
            } => Self::parenthesize(&operator.lexeme, &[left, right]),
            Self::Grouping { expression } => Self::parenthesize("", &[expression]),
            Self::Unary { operator, right } => Self::parenthesize(&operator.lexeme, &[right]),
            Self::Literal { value } => value.to_string(),
            Self::Logical {
                left,
                operator,
                right,
            } => Self::parenthesize(&operator.lexeme, &[left, right]),
            Self::Variable { name } => name.to_string(),
        }
    }

    fn parenthesize(name: &str, expressions: &[&Expr]) -> String {
        let inner = expressions
            .iter()
            .map(|e| e.to_tree())
            .collect::<Vec<_>>()
            .join(" ");

        if name == "" {
            format!("{inner}")
        } else {
            format!("({name} {inner})")
        }
    }
}
