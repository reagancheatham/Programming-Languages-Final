use std::io;

use anyhow::{Result, anyhow};

use crate::oil::{
    environment::{Environment, RefEnvironment},
    expression::Expression,
    logger::Logger,
    object::Object,
    statement::Statement,
    token::{Token, TokenType},
};

pub struct Interpreter {
    environment: RefEnvironment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new_ref(None),
        }
    }

    pub fn interpret(&mut self, statements: &[Statement]) {
        if statements.len() == 1
            && let Statement::Expression(expr) = &statements[0]
        {
            let result = self.evaluate_expression(expr);

            if result.is_err() {
                println!("Error: {}", result.err().unwrap());
                return;
            } else {
                println!("{}", result.unwrap());
            }
        }

        for statement in statements {
            let result = self.execute(statement);

            if result.is_err() {
                break;
            }
        }
    }

    fn execute(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Var { name, initializer } => {
                let mut value = Object::Null;

                if let Some(expr) = initializer {
                    value = self.evaluate_expression(expr)?;
                }

                self.environment.borrow_mut().define(&name.lexeme, value);
            }
            Statement::Expression(expr) => {
                self.evaluate_expression(expr)?;
            }
            Statement::Print(expr) => {
                let value = self.evaluate_expression(expr)?;

                println!("{value}");
            }
            Statement::ReadInput { target_var } => {
                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read user input!");

                self.environment
                    .borrow_mut()
                    .assign(target_var, Object::parse(&input))?;
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evaluate_expression(condition)?.is_truthy() {
                    self.execute(then_branch)?;
                } else if let Some(else_value) = else_branch {
                    self.execute(else_value)?;
                }
            }
            Statement::While { condition, body } => {
                while self.evaluate_expression(condition)?.is_truthy() {
                    self.execute(body)?;
                }
            }
            Statement::Block(statements) => {
                self.execute_block(
                    statements,
                    Environment::new_ref(Some(self.environment.clone())),
                )?;
            }
        };

        Ok(())
    }

    fn execute_block(
        &mut self,
        statements: &[Statement],
        environment: RefEnvironment,
    ) -> Result<()> {
        let previous = self.environment.clone();
        self.environment = environment;

        let result = (|| {
            for statement in statements {
                self.execute(statement)?;
            }

            Ok(())
        })();

        self.environment = previous;

        result
    }

    fn evaluate_expression(&mut self, expr: &Expression) -> Result<Object> {
        match expr {
            Expression::Assign { name, value } => self.assign(name, value),
            Expression::Binary {
                left,
                operator,
                right,
            } => self.binary(left, operator, right),
            Expression::Grouping { expression } => self.evaluate_expression(&expression),
            Expression::Unary { operator, right } => self.unary(operator, right),
            Expression::Literal { value } => Ok(value.clone()),
            Expression::Logical {
                left,
                operator,
                right,
            } => self.logical(left, operator, right),
            Expression::Variable { name } => self.environment.borrow().get(name),
        }
    }

    fn assign(&mut self, name: &Token, value: &Expression) -> Result<Object> {
        let value = self.evaluate_expression(value)?;

        self.environment.borrow_mut().assign(name, value.clone())?;

        Ok(value)
    }

    fn binary(
        &mut self,
        left: &Expression,
        operator: &Token,
        right: &Expression,
    ) -> Result<Object> {
        let left = self.evaluate_expression(left)?;
        let right = self.evaluate_expression(right)?;

        match (operator.token_type, left, right) {
            (TokenType::EqualEqual, l, r) => Ok(Object::Bool(l.is_equal(&r))),
            (TokenType::NotEqual, l, r) => Ok(Object::Bool(l.is_equal(&r))),

            (TokenType::Greater, Object::Num(l), Object::Num(r)) => Ok(Object::Bool(l > r)),
            (TokenType::GreaterEqual, Object::Num(l), Object::Num(r)) => Ok(Object::Bool(l >= r)),
            (TokenType::Less, Object::Num(l), Object::Num(r)) => Ok(Object::Bool(l < r)),
            (TokenType::LessEqual, Object::Num(l), Object::Num(r)) => Ok(Object::Bool(l <= r)),
            (TokenType::Minus, Object::Num(l), Object::Num(r)) => Ok(Object::Num(l - r)),
            (TokenType::Plus, Object::Num(l), Object::Num(r)) => Ok(Object::Num(l + r)),
            (TokenType::Slash, Object::Num(l), Object::Num(r)) => Ok(Object::Num(l / r)),
            (TokenType::Asterisk, Object::Num(l), Object::Num(r)) => Ok(Object::Num(l * r)),
            (TokenType::Mod, Object::Num(l), Object::Num(r)) => Ok(Object::Num(l % r)),

            (TokenType::Plus, Object::Str(l), Object::Str(r)) => Ok(Object::Str(format!("{l}{r}"))),
            (TokenType::Minus, Object::Str(l), Object::Str(r)) => Ok(Object::Str(format!(
                "{l}{}",
                r.chars().rev().collect::<String>()
            ))),

            (op, l, r) => {
                Logger::error(
                    operator.line,
                    &format!("Invalid binary operation '{op:?}' between '{l}' and '{r}'"),
                );
                Err(anyhow!(
                    "Invalid binary operation '{op:?}' between '{l}' and '{r}'"
                ))
            }
        }
    }

    fn unary(&mut self, operator: &Token, right: &Expression) -> Result<Object> {
        let right_object = self.evaluate_expression(right)?;

        match operator.token_type {
            TokenType::Not => Ok(Object::Bool(right_object.is_truthy())),
            TokenType::Minus => {
                if let Object::Num(num) = right_object {
                    Ok(Object::Num(-num))
                } else if let Object::Str(str) = right_object {
                    Ok(Object::Str(str.chars().rev().collect()))
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

    fn logical(
        &mut self,
        left: &Expression,
        operator: &Token,
        right: &Expression,
    ) -> Result<Object> {
        let left = self.evaluate_expression(left)?;

        match operator.token_type {
            TokenType::Or => {
                if left.is_truthy() {
                    Ok(left)
                } else {
                    Ok(self.evaluate_expression(right)?)
                }
            }
            TokenType::And => {
                if !left.is_truthy() {
                    Ok(left)
                } else {
                    Ok(self.evaluate_expression(right)?)
                }
            }

            op => {
                Logger::error(
                    operator.line,
                    &format!("Invalid logical operation '{op:?}'"),
                );
                Err(anyhow!("Invalid logical operation '{op:?}'"))
            }
        }
    }
}
