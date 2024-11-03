use parse::{ast::expr::ExprNode, types::literal::Literal};
use std::collections::BTreeMap;

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
        self.alloc(lit.into())
    }

    pub fn eval_model(&mut self, typename: String) -> TwiResult<Object> {
        if let Some(model) = self.models.get(&typename) {
            let model = model.clone();
            // initialize all fields to nil
            let instance_inner: BTreeMap<_, _> = model
                .fields
                .iter()
                .map(|field| {
                    let nil = self.alloc(ObjectInner::Nil);
                    (field.clone(), nil)
                })
                .collect();

            Ok(self.alloc(ObjectInner::Model {
                model_name: typename,
                fields: instance_inner,
            }))
        } else {
            Err(TwiError::ModelNotFound(typename))
        }
    }

    pub fn eval_call(&mut self, funcname: String, args: Vec<ExprNode>) -> TwiResult<Object> {
        let callable = self.getvar(funcname.clone())?;
        let func = self.heap.get_value(callable);
        if let Value::Func {
            mut params,
            hid: _,
            body,
        } = func
        {
            if params.len() != args.len() {
                return Err(TwiError::ArgNumMismatch {
                    funcname: funcname.clone(),
                    expected: params.len(),
                    got: args.len(),
                });
            }

            // Do not use iterator/adapter here
            // since we need to do error prop
            let mut args_val = Vec::new();
            // push arguments
            for arg in args {
                args_val.push(self.eval(arg)?);
            }
            // push this function itself
            // first push arg, since push param will modify param list
            // arg
            let thisfunc = self.alloc(ObjectInner::Func {
                params: params.clone(),
                body: body.clone(),
            });
            args_val.push(thisfunc);
            // param
            params.push(funcname);

            // return nil by default
            {
                // start a call
                let sg = self.enter_scope(ScopeType::Call);
                // push all arguments
                for (name, val) in params.into_iter().zip(args_val) {
                    self.cur_scope_mut().add(name, val);
                }
                // execute call body
                for stmt in body {
                    let res = self.exec_stmt(stmt);
                    match res {
                        Ok(_) => {}
                        Err(TwiError::Return(val)) => {
                            // exit scope
                            return Ok(val);
                        }
                        Err(e) => return Err(e),
                    }
                }
                // exit scope
            };

            // MARKER: function return value
            let return_value = self.alloc(ObjectInner::Nil);
            Ok(return_value)
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
        if let Some(obj) = self.global().get(&ident) {
            Ok(obj)
        } else {
            // dbg!(self.cur_scope());
            // dbg!(self.scopes.len());
            Err(TwiError::IdentifierNotFound(ident))
        }
    }

    /// A stupid workaround to fool borrow checker
    pub fn getvar_mut(&mut self, ident: String) -> TwiResult<&mut Object> {
        // you should collect here or the borrow of `scopes` is held by `is_call`
        let (local, global) = self.scopes_mut();
        let is_call = local.iter().map(|s| s.is_call()).collect::<Vec<_>>();
        'find_local: for (scope, is_call) in local.iter_mut().zip(is_call.into_iter()).rev() {
            if let Some(obj) = scope.vars.get_mut(&ident) {
                return Ok(obj);
            }
            if is_call {
                break 'find_local;
            }
        }
        // global
        if let Some(obj) = global.vars.get_mut(&ident) {
            Ok(obj)
        } else {
            Err(TwiError::IdentifierNotFound(ident))
        }
    }
}
