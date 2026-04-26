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
        Expression::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn unary(operator: Token, right: Expression) -> Expression {
        Expression::Unary {
            operator,
            right: Box::new(right),
        }
    }

    pub fn grouping(expression: Expression) -> Expression {
        Expression::Grouping {
            expression: Box::new(expression),
        }
    }

    pub fn literal(value: Object) -> Expression {
        Expression::Literal { value }
    }

    pub fn to_tree(&self) -> String {
        match self {
            Expression::Binary {
                left,
                operator,
                right,
            } => Expression::parenthesize(&operator.lexeme, &[left, right]),
            Expression::Grouping { expression } => Expression::parenthesize("", &[expression]),
            Expression::Literal { value } => value.to_string(),
            Expression::Unary { operator, right } => {
                Expression::parenthesize(&operator.lexeme, &[right])
            }
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
        }
        else {
            format!("({name} {inner})")
        }
    }
}
