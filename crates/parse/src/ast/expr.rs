use lex::token::Token;

use crate::{errors::ParserError, types::literal::Literal};

use super::root::Root;

#[derive(Debug, Clone)]
pub enum ExprNode {
    // atomic
    Literal(Literal),
    Identifer(String),

    // non atomic
    Eq {
        left: Box<ExprNode>,
        right: Box<ExprNode>,
    },
    Neq {
        left: Box<ExprNode>,
        right: Box<ExprNode>,
    },

    Gt {
        left: Box<ExprNode>,
        right: Box<ExprNode>,
    },
    Lt {
        left: Box<ExprNode>,
        right: Box<ExprNode>,
    },
    Geq {
        left: Box<ExprNode>,
        right: Box<ExprNode>,
    },
    Leq {
        left: Box<ExprNode>,
        right: Box<ExprNode>,
    },
    Add {
        left: Box<ExprNode>,
        right: Box<ExprNode>,
    },
    Minus {
        left: Box<ExprNode>,
        right: Box<ExprNode>,
    },
    Div {
        left: Box<ExprNode>,
        right: Box<ExprNode>,
    },
    Mul {
        left: Box<ExprNode>,
        right: Box<ExprNode>,
    },
    Not(Box<ExprNode>),
    Neg(Box<ExprNode>),

    // pathed with Lpar and Rpar
    Packed(Box<ExprNode>),

    Call {
        name: String,
        args: Vec<ExprNode>,
    },
}

impl TryFrom<Vec<Token>> for ExprNode {
    type Error = ParserError;

    fn try_from(value: Vec<Token>) -> Result<Self, Self::Error> {
        // only 1 token
        if value.len() == 1 {
            let token = value[0].clone();
            // literal
            if let Ok(lit) = Literal::try_from(token.clone()) {
                return Ok(Self::Literal(lit));
            }
            // ident
            if let Token::Identifier(ident) = token.clone() {
                return Ok(Self::Identifer(ident));
            }
        }

        todo!()
    }
}

// expression constructors
impl ExprNode {
    // Non-atomic variants
    pub fn eq(left: ExprNode, right: ExprNode) -> Self {
        ExprNode::Eq {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn neq(left: ExprNode, right: ExprNode) -> Self {
        ExprNode::Neq {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn gt(left: ExprNode, right: ExprNode) -> Self {
        ExprNode::Gt {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn lt(left: ExprNode, right: ExprNode) -> Self {
        ExprNode::Lt {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn geq(left: ExprNode, right: ExprNode) -> Self {
        ExprNode::Geq {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn leq(left: ExprNode, right: ExprNode) -> Self {
        ExprNode::Leq {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn add(left: ExprNode, right: ExprNode) -> Self {
        ExprNode::Add {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn minus(left: ExprNode, right: ExprNode) -> Self {
        ExprNode::Minus {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn div(left: ExprNode, right: ExprNode) -> Self {
        ExprNode::Div {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn mul(left: ExprNode, right: ExprNode) -> Self {
        ExprNode::Mul {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn not(expr: ExprNode) -> Self {
        ExprNode::Not(Box::new(expr))
    }

    pub fn neg(expr: ExprNode) -> Self {
        ExprNode::Neg(Box::new(expr))
    }

    // Pathed with Lpar and Rpar
    pub fn packed(expr: ExprNode) -> Self {
        ExprNode::Packed(Box::new(expr))
    }

    // Call variant
    pub fn call(name: String, args: Vec<ExprNode>) -> Self {
        ExprNode::Call { name, args }
    }
}
