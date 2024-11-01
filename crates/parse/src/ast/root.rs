use super::stmt::StmtNode;

#[derive(Debug)]
pub struct Root {
    pub statements: Vec<StmtNode>,
}
