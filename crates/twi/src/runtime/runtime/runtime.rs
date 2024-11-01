use parse::ast::{expr::ExprNode, root::Root, stmt::StmtNode};

use crate::{
    errors::TwiResult,
    runtime::gc::{gc::Heap, objects::Object},
    scope::scope::Scope,
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

pub struct GlobalFunc {
    pub(crate) name: String,
    pub(crate) body: Vec<StmtNode>,
}

pub struct GlobalVar {
    pub(crate) name: String,
    pub(crate) val: Object,
}

// ########################################################
// ####################### Runtime ########################
// ########################################################

pub struct Runtime {
    pub(crate) models: Vec<Model>,
    pub(crate) global_funcs: Vec<GlobalFunc>,
    pub(crate) global_vars: Vec<GlobalVar>,

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
}
