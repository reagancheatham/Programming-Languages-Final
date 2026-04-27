use crate::oil::{object::Object, token::Token};

type Expr = Box<Expression>;

#[derive(Clone)]
pub enum Expression {
    Binary {
        left: Expr,
        operator: Token,
        right: Expr,
    },
    Grouping {
        expression: Expr,
    },
    Literal {
        value: Object,
    },
    Unary {
        operator: Token,
        right: Expr,
    },
}

impl Expression {
    pub fn binary(left: Expression, operator: Token, right: Expression) -> Expression {
        Self::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn unary(operator: Token, right: Expression) -> Expression {
        Self::Unary {
            operator,
            right: Box::new(right),
        }
    }

    pub fn grouping(expression: Expression) -> Expression {
        Self::Grouping {
            expression: Box::new(expression),
        }
    }

    pub fn literal(value: Object) -> Expression {
        Self::Literal { value }
    }

    pub fn to_tree(&self) -> String {
        match self {
            Self::Binary {
                left,
                operator,
                right,
            } => Self::parenthesize(&operator.lexeme, &[left, right]),
            Self::Grouping { expression } => Self::parenthesize("", &[expression]),
            Self::Literal { value } => value.to_string(),
            Self::Unary { operator, right } => Self::parenthesize(&operator.lexeme, &[right]),
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
