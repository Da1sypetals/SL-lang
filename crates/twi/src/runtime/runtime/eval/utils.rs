use parse::ast::expr::ExprNode;

use crate::{
    errors::{TwiError, TwiResult},
    runtime::{gc::objects::Object, runtime::runtime::Runtime},
};

impl Runtime {
    /// ### Get variable, being able to downtrack certain level of function call;
    /// - current scope is `lvl = 0`.
    pub(crate) fn getvar_lvl(&self, ident: String, lvl: usize) -> TwiResult<Object> {
        let mut cur_lvl = 0;
        'find_local: for scope in self.scopes.iter().rev() {
            if cur_lvl < lvl {
                if scope.is_call() {
                    cur_lvl += 1;
                }
                continue;
            } else {
                if let Some(obj) = scope.vars.get(&ident) {
                    return Ok(*obj);
                }
                // find local terminates at call scope (if not found)
                if scope.is_call() {
                    break 'find_local;
                }
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
    pub(crate) fn getvar_mut_lvl(&mut self, ident: String, lvl: usize) -> TwiResult<&mut Object> {
        // you should collect here or the borrow of `scopes` is held by `is_call`
        let is_call = self.scopes.iter().map(|s| s.is_call()).collect::<Vec<_>>();
        let mut cur_lvl = 0;
        'find_local: for (scope, is_call) in self.scopes.iter_mut().zip(is_call.into_iter()).rev() {
            if cur_lvl < lvl {
                if scope.is_call() {
                    cur_lvl += 1;
                }
                continue;
            } else {
                if let Some(obj) = scope.vars.get_mut(&ident) {
                    return Ok(obj);
                }
                if is_call {
                    break 'find_local;
                }
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
