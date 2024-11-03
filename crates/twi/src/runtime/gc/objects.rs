use lex::token::teer;
use parse::ast::stmt::StmtNode;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Object {
    /// heap index
    pub(crate) hid: usize,
}

impl Eq for Object {}
impl Ord for Object {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hid.cmp(&other.hid)
    }
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
