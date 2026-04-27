use anyhow::{Result, anyhow};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::oil::{object::Object, token::Token};

pub type RefEnvironment = Rc<RefCell<Environment>>;

#[derive(Default)]
pub struct Environment {
    enclosing: Option<RefEnvironment>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            enclosing: enclosing,
            values: HashMap::new(),
        }
    }

    pub fn new_ref(enclosing: Option<Rc<RefCell<Environment>>>) -> RefEnvironment {
        Rc::new(RefCell::new(Self::new(enclosing)))
    }

    pub fn define(&mut self, name: &str, value: Object) {
        self.values.insert(name.to_string(), value);
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<()> {
        match self.values.get_mut(&name.lexeme) {
            Some(slot) => {
                *slot = value;
                Ok(())
            }
            None => self.enclosing.as_mut().map_or_else(
                || Err(anyhow!("Undefined variable '{}'.", name.lexeme)),
                |enclosing| enclosing.borrow_mut().assign(name, value),
            ),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object> {
        let result = self.values.get(&name.lexeme);

        match result {
            Some(value) => Ok(value.clone()),
            None => self.enclosing.as_ref().map_or_else(
                || Err(anyhow!("Undefined variable '{}'.", name.lexeme)),
                |enclosing| enclosing.borrow().get(name),
            ),
        }
    }
}
