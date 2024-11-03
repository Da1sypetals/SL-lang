use std::collections::BTreeMap;

use crate::runtime::gc::objects::Object;

#[derive(Debug, Clone, Copy)]
pub enum ScopeType {
    Call,
    Block,
    Global,
}

#[derive(Debug)]
pub struct Scope {
    pub(crate) scope_type: ScopeType,
    pub(crate) vars: BTreeMap<String, Object>,
    pub(crate) unnamed: Vec<Object>,
}

impl Scope {
    // constructors
    pub fn block() -> Self {
        Self {
            scope_type: ScopeType::Block,
            vars: BTreeMap::new(),
            unnamed: Vec::new(),
        }
    }

    pub fn call() -> Self {
        Self {
            scope_type: ScopeType::Call,
            vars: BTreeMap::new(),
            unnamed: Vec::new(),
        }
    }
}

impl Scope {
    pub fn add(&mut self, ident: String, obj: Object) {
        self.vars.insert(ident, obj);
    }

    pub fn get(&self, ident: &str) -> Option<Object> {
        self.vars.get(ident).map(|x| *x)
    }
}

impl Scope {
    #[inline(always)]
    pub fn is_call(&self) -> bool {
        match self.scope_type {
            ScopeType::Call => true,
            _ => false,
        }
    }

    #[inline(always)]
    pub fn is_block(&self) -> bool {
        match self.scope_type {
            ScopeType::Block => true,
            _ => false,
        }
    }
}
