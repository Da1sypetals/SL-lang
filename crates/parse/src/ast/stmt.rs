use super::expr::ExprNode;

#[derive(Clone, Debug)]
pub enum Lvalue {
    Identifier(String),
    Member { base: String, members: Vec<String> },
}

#[derive(Clone, Debug)]
pub enum StmtNode {
    Expression {
        expr: ExprNode,
    },

    // others
    Let {
        target: Lvalue,
        expr: ExprNode,
    },
    Return {
        expr: ExprNode,
    },
    Print {
        expr: ExprNode,
    },
    For {
        iter: String,
        n_iter: ExprNode,
        body: Vec<StmtNode>,
    },
    While {
        cond: ExprNode,
        body: Vec<StmtNode>,
    },
    If {
        cond: ExprNode,
        body: Vec<StmtNode>,
    },
    IfElse {
        cond: ExprNode,
        if_body: Vec<StmtNode>,
        else_body: Vec<StmtNode>,
    },
    Scope {
        body: Vec<StmtNode>,
    },
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Vec<StmtNode>,
    },
}

impl StmtNode {
    pub fn unwrap_if(&self) -> (ExprNode, Vec<StmtNode>) {
        match self {
            Self::If { cond, body } => (cond.clone(), body.clone()),
            _ => panic!(),
        }
    }
}
