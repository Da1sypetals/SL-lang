use std::collections::VecDeque;

use crate::{
    errors::{TwiError, TwiResult},
    runtime::{
        gc::{
            objects::{Object, ObjectInner},
            value::Value,
        },
        runtime::runtime::Runtime,
    },
    scope::{self, scope::ScopeType},
};
use parse::ast::{expr::ExprNode, stmt::StmtNode};

impl Runtime {
    /// Evaluate expression at specific level of call stack.
    pub fn eval_at_lvl(&mut self, expr: ExprNode, lvl: usize) -> TwiResult<Object> {
        match expr {
            ExprNode::Literal(literal) => {
                //
                Ok(self.eval_literal(literal))
            }
            ExprNode::Identifer(ident) => {
                // track back stack
                self.getvar_lvl(ident, lvl)
            }
            ExprNode::New(typename) => {
                //
                self.parse_model(typename)
            }
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
            ExprNode::Call { name, args } => self.eval_call(name, args),
        }
    }

    pub fn eval(&mut self, expr: ExprNode) -> TwiResult<Object> {
        self.eval_at_lvl(expr, 0)
    }
}
