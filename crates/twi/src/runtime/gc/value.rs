use std::fmt::Display;

use lex::token::teer;

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Int(i64),
    Float(f64),
    Teer(teer),
    Bool(bool),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
