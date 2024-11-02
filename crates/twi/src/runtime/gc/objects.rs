use lex::token::teer;
use parse::ast::stmt::StmtNode;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug)]
pub struct Object {
    /// heap index
    pub(crate) hid: usize,
}

#[derive(Debug)]
pub struct ObjectHandle {
    pub(crate) alive: bool,
    pub(crate) ptr: *mut ObjectInner,
}

#[derive(Debug)]
pub enum ObjectInner {
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
        model_name: String,
        fields: BTreeMap<String, Object>,
    },
}
