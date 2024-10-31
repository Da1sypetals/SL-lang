use super::stmt::StmtNode;

#[derive(Debug)]
pub struct Root {
    pub(crate) statements: Vec<StmtNode>,
}
