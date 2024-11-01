use parse::ast::{root::Root, stmt::StmtNode};

pub struct Runtime {
    statements: Vec<StmtNode>,
}

impl Runtime {
    pub fn new(root: Root) -> Self {
        Self {
            statements: root.statements,
        }
    }
}
