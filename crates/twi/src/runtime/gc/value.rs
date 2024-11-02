use std::{collections::BTreeMap, fmt::Display};

use lex::token::teer;
use parse::ast::stmt::StmtNode;

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Int(i64),
    Float(f64),
    Teer(teer),
    Bool(bool),
    String(String),
    Func {
        params: Vec<String>,
        body: Vec<StmtNode>,
    },
    Model {
        name: String,
        fields: BTreeMap<String, Value>,
    },
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
