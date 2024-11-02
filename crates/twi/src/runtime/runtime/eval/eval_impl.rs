use std::collections::BTreeMap;

use parse::types::literal::Literal;

use crate::{
    errors::{TwiError, TwiResult},
    runtime::{
        gc::objects::{Object, ObjectInner},
        runtime::runtime::Runtime,
    },
};

impl Runtime {
    pub fn parse_literal(&mut self, lit: Literal) -> Object {
        self.heap.alloc(lit.into())
    }

    pub fn parse_model(&mut self, typename: String) -> TwiResult<Object> {
        if let Some(model) = self.models.get(&typename) {
            // initialize all fields to nil
            let instance_inner: BTreeMap<_, _> = model
                .fields
                .iter()
                .map(|field| {
                    let nil = self.heap.alloc(ObjectInner::Nil);
                    (field.clone(), nil)
                })
                .collect();

            Ok(self.heap.alloc(ObjectInner::Model {
                model_name: typename,
                fields: instance_inner,
            }))
        } else {
            Err(TwiError::ModelNotFound(typename))
        }
    }
}
