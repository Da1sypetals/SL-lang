use parse::{
    ast::{
        expr::ExprNode,
        stmt::{Lvalue, StmtNode},
    },
    types::literal::Literal,
};

use crate::{
    errors::{TwiError, TwiResult},
    runtime::gc::{objects::ObjectInner, value::Value},
    scope::scope::ScopeType,
};

use super::runtime::Runtime;

impl Runtime {
    pub fn exec_let(&mut self, ident: String, expr: ExprNode) -> TwiResult<()> {
        // for sc in &self.scopes {
        //     dbg!(&sc.vars);
        // }
        if self.cur_scope().vars.contains_key(&ident) {
            return Err(TwiError::DuplicateLocalBind(ident));
        }
        let val = self.eval(expr)?;
        self.cur_scope_mut().add(ident, val);
        // dbg!(&self.cur_scope());

        Ok(())
    }

    pub fn exec_funcdef(
        &mut self,
        name: String,
        params: Vec<String>,
        body: Vec<StmtNode>,
    ) -> TwiResult<()> {
        // for sc in &self.scopes {
        //     dbg!(&sc.vars);
        // }
        if self.cur_scope_mut().vars.contains_key(&name) {
            return Err(TwiError::DuplicateLocalBind(name));
        }
        self.bind(name, ObjectInner::Func { params, body });

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
                let sg = self.enter_scope(ScopeType::Block);
                // define iter counter
                self.exec_let(iter.clone(), ExprNode::Literal(Literal::Int(count)))?;
                for stmt in &body {
                    self.exec_stmt(stmt.clone())?;
                }
            }
            // dbg!(&self.global_scope.vars);
            Ok(())
        } else {
            Err(TwiError::UnexpectedType {
                expected: "Int".into(),
                got: val.to_string(),
            })
        }
    }

    pub fn exec_while(&mut self, cond: ExprNode, body: Vec<StmtNode>) -> TwiResult<()> {
        loop {
            let cond = self.eval(cond.clone())?;
            let val = self.heap.get_value(cond);

            // check if condition is of correct type
            if let Value::Bool(cnd) = val {
                // check if condition is true
                if cnd {
                    let sg = self.enter_scope(ScopeType::Block);
                    for stmt in body.clone() {
                        self.exec_stmt(stmt.clone())?;
                    }
                } else {
                    return Ok(());
                }
            } else {
                // incorrect type
                return Err(TwiError::UnexpectedType {
                    expected: "Bool".into(),
                    got: val.to_string(),
                });
            }
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

    pub fn exec_assign(&mut self, target: Lvalue, expr: ExprNode) -> TwiResult<()> {
        match target {
            Lvalue::Identifier(ident) => {
                // order is critical to fool borrow checker...
                let val = self.eval(expr)?;
                let objref = self.getvar_mut(ident)?;
                *objref = val;
            }
            Lvalue::Member { base, members } => {
                let val = self.eval(expr)?;
                let mut obj = self.getvar(base)?;
                obj.refs(&mut self.heap, members, val)?;
            }
        }

        Ok(())
    }
}
