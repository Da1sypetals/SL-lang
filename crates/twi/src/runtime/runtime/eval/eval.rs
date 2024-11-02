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
    pub fn eval(&mut self, expr: ExprNode) -> TwiResult<Object> {
        //
        match expr {
            ExprNode::Literal(literal) => {
                //
                Ok(self.parse_literal(literal))
            }
            ExprNode::Identifer(ident) => {
                // track back stack
                self.getvar(ident)
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
            ExprNode::Call { name, args } => {
                let callable = self.getvar(name)?;
                let func = self.heap.get_value(callable);
                if let Value::Func { params, body } = func {
                    if params.len() != args.len() {
                        return Err(TwiError::ArgNumMismatch {
                            expected: params.len(),
                            got: args.len(),
                        });
                    }
                    // start a call
                    self.enter_scope(ScopeType::Call);
                    // push all arguments
                    for (name, value) in params.into_iter().zip(args) {
                        self.exec_let(name, value)?;
                    }
                    // execute
                    for stmt in body {
                        self.exec_stmt(stmt)?;
                    }

                    self.exit_scope();
                    let nil = self.heap.alloc(ObjectInner::Nil);
                    // TODO: return value is here
                    Ok(nil)
                } else {
                    return Err(TwiError::CannotCall(format!("{:?}", func)));
                }
            }
        }
    }

    pub fn getvar(&self, ident: String) -> TwiResult<Object> {
        'find_local: for scope in self.scopes.iter().rev() {
            if let Some(obj) = scope.vars.get(&ident) {
                return Ok(*obj);
            }
            // find local sterminates at call scope (if not found)
            if scope.is_call() {
                break 'find_local;
            }
        }
        // global
        if let Some(var) = self.global_vars.get(&ident) {
            Ok(var.obj)
        } else {
            Err(TwiError::IdentifierNotFound(ident))
        }
    }

    /// A stupid workaround to fool borrow checker
    pub fn getvar_mut(&mut self, ident: String) -> TwiResult<&mut Object> {
        // you should collect here or the borrow of `scopes` is held by `is_call`
        let is_call = self.scopes.iter().map(|s| s.is_call()).collect::<Vec<_>>();
        'find_local: for (scope, is_call) in self.scopes.iter_mut().zip(is_call.into_iter()).rev() {
            if let Some(obj) = scope.vars.get_mut(&ident) {
                return Ok(obj);
            }
            if is_call {
                break 'find_local;
            }
        }
        // global
        if let Some(var) = self.global_vars.get_mut(&ident) {
            Ok(&mut var.obj)
        } else {
            Err(TwiError::IdentifierNotFound(ident))
        }
    }
}
