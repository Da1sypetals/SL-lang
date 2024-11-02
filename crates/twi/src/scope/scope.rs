use std::collections::BTreeMap;

use crate::runtime::gc::objects::Object;

#[derive(Debug, Clone, Copy)]
pub enum ScopeType {
    Call,
    Block,
}

#[derive(Debug)]
pub struct Scope {
    pub(crate) scope_type: ScopeType,
    pub(crate) vars: BTreeMap<String, Object>,
}

impl Scope {
    // constructors
    pub fn block() -> Self {
        Self {
            scope_type: ScopeType::Block,
            vars: BTreeMap::new(),
        }
    }

    pub fn call() -> Self {
        Self {
            scope_type: ScopeType::Call,
            vars: BTreeMap::new(),
        }
    }
}

impl Scope {
    pub fn add(&mut self, ident: String, obj: Object) {
        self.vars.insert(ident, obj);
    }

    pub fn get(&mut self, ident: &str) -> Option<Object> {
        self.vars.get(ident).map(|x| *x)
    }
}

impl Scope {
    #[inline(always)]
    pub fn is_call(&self) -> bool {
        match self.scope_type {
            ScopeType::Call => true,
            ScopeType::Block => false,
        }
    }

    #[inline(always)]
    pub fn is_block(&self) -> bool {
        match self.scope_type {
            ScopeType::Call => false,
            ScopeType::Block => true,
        }
    }
}
