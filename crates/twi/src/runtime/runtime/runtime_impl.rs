use std::collections::BTreeMap;

use log::info;
use parse::ast::stmt::StmtNode;

use crate::{
    errors::{TwiError, TwiResult},
    runtime::gc::{gc::Heap, objects::ObjectInner, timer::Timer},
    scope::scope::{Scope, ScopeType},
};

use super::runtime::{GlobalVar, Model, Runtime};

impl Runtime {
    //

    pub fn structure(statements: Vec<StmtNode>) -> TwiResult<Self> {
        let mut rt = Self {
            models: BTreeMap::new(),
            program: Vec::new(),
            heap: Heap::new(),
            scopes: vec![],
            gc_interval: 0.8,
            gc_timer: Timer::new(),
            global_scope: Scope {
                scope_type: ScopeType::Global,
                vars: BTreeMap::new(),
                unnamed: Vec::new(),
            },
        };

        for stmt in statements {
            match stmt {
                StmtNode::FuncDef { name, params, body } => {
                    // main
                    if name == "main" {
                        rt.program = body;
                        return Ok(rt);
                    } else {
                        // bind in global scope
                        let func = rt.alloc(ObjectInner::Func { params, body });
                        rt.global_scope.add(name, func);
                    }
                }
                StmtNode::Model { name, fields } => {
                    rt.models.insert(name.clone(), Model { name, fields });
                }
                StmtNode::Let { ident, expr } => {
                    // bind in global scope
                    let obj = rt.eval(expr)?;
                    rt.global_scope.add(ident, obj);
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
        // try gc
        if self.gc_timer.elapsed() >= self.gc_interval {
            self.gc();
            self.gc_timer.reset();
        }

        // info!("scope.len={}", self.scopes.len());

        // exec
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
            StmtNode::Return { expr } => {
                // return value as error
                return Err(TwiError::Return(self.eval(expr)?));
            }
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
                info!("scope.len={}", self.scopes.len());
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
