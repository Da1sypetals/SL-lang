use std::collections::BTreeMap;

use parse::ast::stmt::StmtNode;

use crate::{
    errors::{TwiError, TwiResult},
    runtime::gc::{gc::Heap, objects::ObjectInner},
    scope::scope::ScopeType,
};

use super::runtime::{GlobalVar, Model, Runtime};

impl Runtime {
    //

    pub fn structure(statements: Vec<StmtNode>) -> TwiResult<Self> {
        let mut rt = Self {
            models: BTreeMap::new(),
            global_vars: BTreeMap::new(),
            program: Vec::new(),
            heap: Heap::new(),
            scopes: Vec::new(),
        };

        for stmt in statements {
            match stmt {
                StmtNode::FuncDef { name, params, body } => {
                    // main
                    if name == "main" {
                        rt.program = body;
                        return Ok(rt);
                    } else {
                        let func = rt.heap.alloc(ObjectInner::Func { params, body });
                        rt.global_vars.insert(name, GlobalVar { obj: func });
                    }
                }
                StmtNode::Model { name, fields } => {
                    rt.models.insert(name.clone(), Model { name, fields });
                }
                StmtNode::Let { ident, expr } => {
                    let obj = rt.eval(expr)?;
                    rt.global_vars.insert(ident, GlobalVar { obj });
                }
                s => return Err(TwiError::InvalidGlobalDefinition(format!("{:?}", s))),
            }
        }

        Err(TwiError::MainNotFound)
    }

    pub fn run(&mut self) -> TwiResult<()> {
        //
        for stmt in self.program.clone() {
            let stmt = stmt.clone();
            self.exec_stmt(stmt)?;
        }
        Ok(())
    }

    /// todo: return
    pub fn exec_stmt(&mut self, stmt: StmtNode) -> TwiResult<()> {
        match stmt {
            StmtNode::Let { ident, expr } => {
                self.exec_let(ident, expr)?;
            }
            StmtNode::For { iter, n_iter, body } => {
                //
                self.exec_for(iter, n_iter, body)?;
            }
            StmtNode::Print { expr } => self.exec_print(expr)?,
            StmtNode::While { cond, body } => {
                self.exec_while(cond, body)?;
            }
            StmtNode::Expression { expr } => {
                self.eval(expr)?;
            }
            StmtNode::Return { expr } => todo!(),
            StmtNode::If { cond, body } => {
                //
                let sg = self.enter_scope(ScopeType::Block);
                self.exec_if_else(cond, body, Vec::new())?;
            }
            StmtNode::IfElse {
                cond,
                if_body,
                else_body,
            } => {
                let sg = self.enter_scope(ScopeType::Block);
                self.exec_if_else(cond, if_body, else_body)?;
            }
            StmtNode::Scope { body } => {
                let sg = self.enter_scope(ScopeType::Block);
                for stmt in body {
                    self.exec_stmt(stmt.clone())?;
                }
            }
            StmtNode::FuncDef { name, params, body } => self.exec_funcdef(name, params, body)?,
            StmtNode::Model { name, fields } => {
                return Err(TwiError::UnexpectedStatement("Model".into()))
            }
            StmtNode::Assign { target, expr } => self.exec_assign(target, expr)?,
        }
        Ok(())
    }
}
