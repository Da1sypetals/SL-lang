use parse::{
    ast::{expr::ExprNode, stmt::StmtNode},
    types::literal::Literal,
};

use crate::{
    errors::{TwiError, TwiResult},
    runtime::gc::value::Value,
    scope::scope::ScopeType,
};

use super::runtime::{Eval, Runtime};

impl Runtime {
    pub fn exec_let(&mut self, ident: String, expr: ExprNode) -> TwiResult<()> {
        // for sc in &self.scopes {
        //     dbg!(&sc.vars);
        // }
        if self.cur_scope().vars.contains_key(&ident) {
            return Err(TwiError::DuplicateLocalBind(ident));
        }
        let val = self.eval(expr)?;
        self.cur_scope().add(ident, val);

        Ok(())
    }

    pub fn exec_print(&mut self, expr: ExprNode) -> TwiResult<()> {
        let obj = self.eval(expr)?;
        println!("[SL print] {}", self.heap.get_value(obj));

        Ok(())
    }

    /// iter is placeholder
    pub fn exec_for(
        &mut self,
        iter: String,
        n_iter: ExprNode,
        body: Vec<StmtNode>,
    ) -> TwiResult<()> {
        let n_iter = self.eval(n_iter)?;
        let val = self.heap.get_value(n_iter);
        if let Value::Int(n) = val {
            for count in 0..n {
                // each iteration is a scope
                self.enter_scope(ScopeType::Block);
                // define iter counter
                self.exec_let(iter.clone(), ExprNode::Literal(Literal::Int(count)))?;
                for stmt in &body {
                    self.exec_stmt(stmt.clone())?;
                }
                self.exit_scope();
            }
            Ok(())
        } else {
            Err(TwiError::UnexpectedType {
                expected: "Int".into(),
                got: val.to_string(),
            })
        }
    }

    pub fn exec_if_else(
        &mut self,
        cond: ExprNode,
        if_body: Vec<StmtNode>,
        else_body: Vec<StmtNode>,
    ) -> TwiResult<()> {
        let cond = self.eval(cond)?;
        let val = self.heap.get_value(cond);
        if let Value::Bool(cnd) = val {
            if cnd {
                for stmt in if_body {
                    self.exec_stmt(stmt.clone())?;
                }
            } else {
                for stmt in else_body {
                    self.exec_stmt(stmt.clone())?;
                }
            }
            Ok(())
        } else {
            Err(TwiError::UnexpectedType {
                expected: "Bool".into(),
                got: val.to_string(),
            })
        }
    }
}
