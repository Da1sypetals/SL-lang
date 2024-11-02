use std::collections::BTreeMap;

use parse::ast::stmt::StmtNode;

use crate::{
    errors::{TwiError, TwiResult},
    runtime::gc::gc::Heap,
    scope::scope::ScopeType,
};

use super::runtime::{Eval, GlobalFunc, GlobalVar, Model, Runtime};

impl Runtime {
    //

    pub fn structure(statements: Vec<StmtNode>) -> TwiResult<Self> {
        let mut models = Vec::new();
        let mut global_funcs = BTreeMap::new();
        let mut global_vars = BTreeMap::new();

        for stmt in statements {
            match stmt {
                StmtNode::FuncDef { name, params, body } => {
                    // main
                    if name == "main" {
                        return Ok(Self {
                            models,
                            global_funcs,
                            global_vars,
                            program: body,
                            heap: Heap::new(),
                            scopes: Vec::new(),
                        });
                    } else {
                        global_funcs.insert(name, GlobalFunc { body });
                    }
                }
                StmtNode::Model { name, fields } => {
                    models.push(Model { name, fields });
                }
                StmtNode::Let { ident, expr } => {
                    global_vars.insert(ident, GlobalVar { obj: expr.eval() });
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

    /// todo: print, 
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
            StmtNode::While { cond, body } => todo!(),
            StmtNode::Expression { expr } => todo!(),
            StmtNode::Return { expr } => todo!(),
            StmtNode::If { cond, body } => {
                //
                self.enter_scope(ScopeType::Block);
                self.exec_if_else(cond, body, Vec::new())?;
                self.exit_scope();
            }
            StmtNode::IfElse {
                cond,
                if_body,
                else_body,
            } => {
                self.enter_scope(ScopeType::Block);
                self.exec_if_else(cond, if_body, else_body)?;
                self.exit_scope();
            }
            StmtNode::Scope { body } => {
                self.enter_scope(ScopeType::Block);
                for stmt in body {
                    self.exec_stmt(stmt.clone())?;
                }
                self.exit_scope();
            }
            StmtNode::FuncDef { name, params, body } => todo!(),
            StmtNode::Model { name, fields } => {
                return Err(TwiError::UnexpectedStatement("Model".into()))
            }
            StmtNode::Assign { target, expr } => todo!(),
        }
        Ok(())
    }
}
