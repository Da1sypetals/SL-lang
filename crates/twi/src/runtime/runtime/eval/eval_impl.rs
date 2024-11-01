use parse::types::literal::Literal;

use crate::runtime::{gc::objects::Object, runtime::runtime::Runtime};

impl Runtime {
    pub fn parse_literal(&mut self, lit: Literal) -> Object {
        self.heap.alloc(lit.into())
    }
}
