use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
}

impl Object {
    pub fn is_equal(&self, right: &Object) -> bool {
        match (self, right) {
            (Object::Null, Object::Null) => true,
            (Object::Bool(b1), Object::Bool(b2)) => *b1 == *b2,
            (Object::Num(n1), Object::Num(n2)) => *n1 == *n2,
            (Object::Str(s1), Object::Str(s2)) => *s1 == *s2,
            _ => false,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Null => false,
            Object::Bool(b) => *b,
            _ => true,
        }
    }

    pub fn parse(input: &str) -> Object {
        let input = input.trim();

        if input == "null" || input == "NULL" {
            Object::Null
        } else if let Ok(num) = input.parse::<f64>() {
            Object::Num(num)
        } else if let Ok(b) = input.parse::<bool>() {
            Object::Bool(b)
        } else {
            Object::Str(input.to_string())
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Str(str) => write!(f, "{str}"),
            Object::Num(num) => write!(f, "{num}"),
            Object::Bool(b) => write!(f, "{b}"),
            Object::Null => write!(f, "null"),
        }
    }
}
