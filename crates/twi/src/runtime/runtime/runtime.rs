use std::collections::BTreeMap;

use parse::ast::{expr::ExprNode, root::Root, stmt::StmtNode};

use crate::{
    errors::TwiResult,
    runtime::gc::{gc::Heap, objects::Object},
    scope::{
        scope::{Scope, ScopeType},
        scope_guard::ScopeGuard,
    },
};

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

    pub fn cur_scope(&self) -> &Scope {
        self.scopes.last().expect("No scope???")
    }

    pub fn cur_scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().expect("No scope???")
    }

    /// Manage scope via RAII
    #[must_use]
    pub(crate) fn enter_scope(&mut self, type_: ScopeType) -> ScopeGuard {
        self.scopes.push(Scope {
            scope_type: type_,
            vars: BTreeMap::new(),
        });

        ScopeGuard {
            rt: self as *mut Runtime,
        }
    }
}
