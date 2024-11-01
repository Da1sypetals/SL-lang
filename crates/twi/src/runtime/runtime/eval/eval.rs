use crate::runtime::{gc::objects::Object, runtime::runtime::Runtime};
use parse::ast::expr::ExprNode;

impl Runtime {
    pub fn eval(&mut self, expr: ExprNode) -> Object {
        //
        match expr {
            ExprNode::Literal(literal) => {
                //
                self.parse_literal(literal)
            }
            ExprNode::Identifer(_) => todo!(),
            ExprNode::New(_) => todo!(),
            ExprNode::Member { base, members } => todo!(),
            ExprNode::Eq { left, right } => todo!(),
            ExprNode::Neq { left, right } => todo!(),
            ExprNode::Gt { left, right } => todo!(),
            ExprNode::Lt { left, right } => todo!(),
            ExprNode::Geq { left, right } => todo!(),
            ExprNode::Leq { left, right } => todo!(),
            ExprNode::Add { left, right } => todo!(),
            ExprNode::Minus { left, right } => todo!(),
            ExprNode::Div { left, right } => todo!(),
            ExprNode::Mul { left, right } => todo!(),
            ExprNode::Not(expr_node) => todo!(),
            ExprNode::Neg(expr_node) => todo!(),
            ExprNode::Packed(expr_node) => todo!(),
            ExprNode::Call { name, args } => todo!(),
        }
    }
}
