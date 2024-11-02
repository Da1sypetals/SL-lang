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
        hid: usize,
        body: Vec<StmtNode>,
    },
    Model {
        name: String,
        hid: usize,
        members: Vec<(String, Value)>,
    },
    ModelRef {
        name: String,
        hid: usize,
    },
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
