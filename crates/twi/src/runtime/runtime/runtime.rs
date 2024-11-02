use std::collections::BTreeMap;

use parse::ast::{expr::ExprNode, root::Root, stmt::StmtNode};

use crate::{
    errors::TwiResult,
    runtime::gc::{gc::Heap, objects::Object},
    scope::scope::{Scope, ScopeType},
};

pub trait Eval {
    fn eval(&self) -> Object;
}

impl Eval for ExprNode {
    fn eval(&self) -> Object {
        // evaluate without context, reporting all identifier-like thing as runtime error.
        todo!()
    }
}

// ########## utils ###########################

pub struct Model {
    pub(crate) name: String,
    pub(crate) fields: Vec<String>,
}

pub struct GlobalVar {
    pub(crate) obj: Object,
}

// ########################################################
// ####################### Runtime ########################
// ########################################################

pub struct Runtime {
    pub(crate) models: BTreeMap<String, Model>,
    pub(crate) global_vars: BTreeMap<String, GlobalVar>,

    pub(crate) program: Vec<StmtNode>,

    pub(crate) heap: Heap,

    // runtime data
    pub(crate) scopes: Vec<Scope>,
}

impl Runtime {
    pub fn try_new(root: Root) -> TwiResult<Self> {
        let mut rt = Self::structure(root.statements)?;
        rt.scopes.push(Scope::block());
        Ok(rt)
    }

    pub fn cur_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().expect("No scope???")
    }

    /// To achieve RAII, we have to hold a reference of `Runtime`
    /// so we don't use RAII here.
    pub fn enter_scope(&mut self, type_: ScopeType) {
        self.scopes.push(Scope {
            scope_type: type_,
            vars: BTreeMap::new(),
        });
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }
}
