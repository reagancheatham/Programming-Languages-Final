use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
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
