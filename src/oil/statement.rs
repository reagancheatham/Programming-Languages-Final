use crate::oil::{expression::Expression, token::Token};

type Stmt = Box<Statement>;

#[derive(Debug)]
pub enum Statement {
    Var {
        name: Token,
        initializer: Option<Expression>,
    },
    Expression(Expression),
    Print(Expression),
    ReadInput {
        target_var: Token
    },
    If {
        condition: Expression,
        then_branch: Stmt,
        else_branch: Option<Stmt>,
    },
    While {
        condition: Expression,
        body: Stmt,
    },
    Block(Vec<Statement>),
}

impl Statement {
    pub fn var(name: Token, initializer: Option<Expression>) -> Statement {
        Self::Var { name, initializer }
    }

    pub fn if_statement(
        condition: Expression,
        then_branch: Statement,
        else_branch: Option<Statement>,
    ) -> Statement {
        let else_value = match else_branch {
            Some(statement) => Some(Box::new(statement)),
            None => None,
        };

        Self::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_value,
        }
    }

    pub fn while_statement(condition: Expression, body: Statement) -> Statement {
        Self::While {
            condition,
            body: Box::new(body),
        }
    }
}
