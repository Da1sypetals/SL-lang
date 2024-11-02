use crate::{
    errors::{TwiError, TwiResult},
    runtime::{gc::objects::Object, runtime::runtime::Runtime},
    scope,
};
use parse::ast::expr::ExprNode;

impl Runtime {
    pub fn eval(&mut self, expr: ExprNode) -> TwiResult<Object> {
        //
        match expr {
            ExprNode::Literal(literal) => {
                //
                Ok(self.parse_literal(literal))
            }
            ExprNode::Identifer(ident) => {
                // track back stack
                for scope in self.scopes.iter().rev() {
                    if let Some(obj) = scope.vars.get(&ident) {
                        return Ok(*obj);
                    }
                }
                // global
                if let Some(var) = self.global_vars.get(&ident) {
                    Ok(var.obj)
                } else {
                    Err(TwiError::IdentifierNotFound(ident))
                }
            }
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
