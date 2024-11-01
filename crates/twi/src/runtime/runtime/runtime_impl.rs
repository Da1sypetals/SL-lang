use parse::ast::stmt::StmtNode;

use crate::{
    errors::{TwiError, TwiResult},
    runtime::gc::gc::Heap,
};

use super::runtime::{Eval, GlobalFunc, GlobalVar, Model, Runtime};

impl Runtime {
    //

    pub fn structure(statements: Vec<StmtNode>) -> TwiResult<Self> {
        let mut models = Vec::new();
        let mut global_funcs = Vec::new();
        let mut global_vars = Vec::new();

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
                        global_funcs.push(GlobalFunc { name, body });
                    }
                }
                StmtNode::Model { name, fields } => {
                    models.push(Model { name, fields });
                }
                StmtNode::Let { ident, expr } => {
                    global_vars.push(GlobalVar {
                        name: ident,
                        val: expr.eval(),
                    });
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
            //
            match stmt {
                StmtNode::Let { ident, expr } => {
                    self.exec_let(ident, expr)?;
                }
                StmtNode::Expression { expr } => todo!(),
                StmtNode::Return { expr } => todo!(),
                StmtNode::Print { expr } => todo!(),
                StmtNode::For { iter, n_iter, body } => todo!(),
                StmtNode::While { cond, body } => todo!(),
                StmtNode::If { cond, body } => todo!(),
                StmtNode::IfElse {
                    cond,
                    if_body,
                    else_body,
                } => todo!(),
                StmtNode::Scope { body } => todo!(),
                StmtNode::FuncDef { name, params, body } => todo!(),
                StmtNode::Model { name, fields } => todo!(),
                StmtNode::Assign { target, expr } => todo!(),
            }
        }
        Ok(())
    }
}
