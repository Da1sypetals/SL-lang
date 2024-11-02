use std::collections::BTreeMap;

use parse::{ast::expr::ExprNode, types::literal::Literal};

use crate::{
    errors::{TwiError, TwiResult},
    runtime::{
        gc::{
            objects::{Object, ObjectInner},
            value::Value,
        },
        runtime::runtime::Runtime,
    },
    scope::scope::ScopeType,
};

impl Runtime {
    pub fn eval_literal(&mut self, lit: Literal) -> Object {
        self.heap.alloc(lit.into())
    }

    pub fn parse_model(&mut self, typename: String) -> TwiResult<Object> {
        if let Some(model) = self.models.get(&typename) {
            // initialize all fields to nil
            let instance_inner: BTreeMap<_, _> = model
                .fields
                .iter()
                .map(|field| {
                    let nil = self.heap.alloc(ObjectInner::Nil);
                    (field.clone(), nil)
                })
                .collect();

            Ok(self.heap.alloc(ObjectInner::Model {
                model_name: typename,
                fields: instance_inner,
            }))
        } else {
            Err(TwiError::ModelNotFound(typename))
        }
    }

    pub fn eval_call(&mut self, funcname: String, args: Vec<ExprNode>) -> TwiResult<Object> {
        let callable = self.getvar(funcname)?;
        let func = self.heap.get_value(callable);
        if let Value::Func { params, body } = func {
            if params.len() != args.len() {
                return Err(TwiError::ArgNumMismatch {
                    expected: params.len(),
                    got: args.len(),
                });
            }
            {
                // start a call
                let sg = self.enter_scope(ScopeType::Call);
                // push all arguments
                for (name, value) in params.into_iter().zip(args) {
                    let val = self.eval_at_lvl(value, 1)?;
                    self.cur_scope_mut().add(name, val);
                }
                // execute
                for stmt in body {
                    self.exec_stmt(stmt)?;
                }
                // exit scope
            }

            let nil = self.heap.alloc(ObjectInner::Nil);
            // TODO: return value is here
            Ok(nil)
        } else {
            return Err(TwiError::CannotCall(format!("{:?}", func)));
        }
    }
}

impl Runtime {
    pub fn getvar(&self, ident: String) -> TwiResult<Object> {
        'find_local: for scope in self.scopes.iter().rev() {
            if let Some(obj) = scope.vars.get(&ident) {
                return Ok(*obj);
            }
            // find local terminates at call scope (if not found)
            if scope.is_call() {
                break 'find_local;
            }
        }
        // global
        if let Some(var) = self.global_vars.get(&ident) {
            Ok(var.obj)
        } else {
            dbg!(self.cur_scope());
            dbg!(self.scopes.len());
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
