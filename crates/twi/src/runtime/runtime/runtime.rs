use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    iter,
};

use colored::Colorize;
use log::warn;
use parse::ast::{
    expr::ExprNode,
    root::{self, Root},
    stmt::StmtNode,
};

use crate::{
    errors::TwiResult,
    runtime::gc::{
        gc::Heap,
        objects::{Object, ObjectInner},
        timer::Timer,
    },
    scope::{
        scope::{Scope, ScopeType},
        scope_guard::ScopeGuard,
    },
};

// ########## utils ###########################

#[derive(Clone)]
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

    pub(crate) program: Vec<StmtNode>,

    pub(crate) heap: Heap,

    // runtime data
    pub(crate) scopes: Vec<Scope>,
    pub(crate) global_scope: Scope,

    pub(crate) gc_interval: f64,
    pub(crate) gc_timer: Timer,
}

impl Runtime {
    pub fn try_new(root: Root, gc_interval: f64) -> TwiResult<Self> {
        let mut rt = Self::structure(root.statements)?;
        dbg!(&rt.global_scope.vars);
        rt.gc_interval = gc_interval;
        rt.scopes.push(Scope::call());
        Ok(rt)
    }

    pub fn cur_scope(&self) -> &Scope {
        if let Some(scope) = self.scopes.last() {
            scope
        } else {
            &self.global_scope
        }
    }

    pub fn cur_scope_mut(&mut self) -> &mut Scope {
        if let Some(scope) = self.scopes.last_mut() {
            scope
        } else {
            &mut self.global_scope
        }
    }

    /// Manage scope via RAII
    #[must_use]
    pub(crate) fn enter_scope(&mut self, type_: ScopeType) -> ScopeGuard {
        self.scopes.push(Scope {
            scope_type: type_,
            vars: BTreeMap::new(),
            unnamed: Vec::new(),
        });

        ScopeGuard {
            rt: self as *mut Runtime,
        }
    }

    pub(crate) fn alloc(&mut self, obj_inner: ObjectInner) -> Object {
        let obj = self.heap.alloc(obj_inner);
        self.cur_scope_mut().unnamed.push(obj);
        obj
    }

    // reference temporarily and annonymously
    pub(crate) fn temp_ref(&mut self, obj: Object) {
        self.cur_scope_mut().unnamed.push(obj);
    }

    pub(crate) fn bind(&mut self, name: String, obj_inner: ObjectInner) -> Object {
        let obj = self.heap.alloc(obj_inner);
        self.cur_scope_mut().vars.insert(name, obj);
        obj
    }

    pub fn gc(&mut self) {
        let mut roots = BTreeSet::new();
        for scope in self.scopes.iter().chain(iter::once(&self.global_scope)) {
            // for scope in self.scopes.iter() {
            for (_, &obj) in &scope.vars {
                roots.insert(obj);
            }
            for &obj in &scope.unnamed {
                roots.insert(obj);
            }
        }
        println!(
            "{}",
            format!(
                "[SL GC] Collecting unused objects, roots.len={}",
                roots.len()
            )
            .color("#888888")
        );
        self.heap.gc(roots.into_iter().collect());
    }

    pub fn global(&self) -> &Scope {
        &self.global_scope
    }

    pub fn global_mut(&mut self) -> &mut Scope {
        &mut self.global_scope
    }

    pub fn scopes_mut(&mut self) -> (&mut Vec<Scope>, &mut Scope) {
        (&mut self.scopes, &mut self.global_scope)
    }
}
