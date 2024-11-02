use crate::errors::{TwiError, TwiResult};

use super::{
    objects::{self, Object, ObjectHandle, ObjectInner},
    value::Value,
};
use lifo::{Deque, Lifo};
use std::{
    alloc::{dealloc, Layout},
    collections::BTreeSet,
    default,
};

pub type Collector = Deque<Object>;

#[derive(Debug)]
pub struct Heap {
    pub(crate) free: BTreeSet<usize>,
    pub(crate) objs: Vec<Option<ObjectHandle>>,
    pub(crate) col: Collector,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            free: BTreeSet::new(),
            objs: Vec::new(),
            col: Collector::new(),
        }
    }

    pub fn get_value(&self, obj: Object) -> Value {
        //
        let handle = self.objs[obj.hid].as_ref().unwrap();
        let obj = unsafe { &*handle.ptr };
        match obj {
            ObjectInner::Nil => Value::Nil,
            ObjectInner::Int(x) => Value::Int(*x),
            ObjectInner::Float(x) => Value::Float(*x),
            ObjectInner::Teer(x) => Value::Teer(*x),
            ObjectInner::Bool(x) => Value::Bool(*x),
            ObjectInner::String(x) => Value::String(x.clone()),
            ObjectInner::Func { params, body } => Value::Func {
                params: params.clone(),
                body: body.clone(),
            },
            ObjectInner::Model { model_name, fields } => Value::Model {
                name: model_name.clone(),
                fields: fields
                    .into_iter()
                    .map(|(name, &obj)| (name.clone(), self.get_value(obj)))
                    .collect(),
            },
        }
    }

    pub fn member(&self, obj: Object, member: String) -> TwiResult<(Object, *mut ObjectInner)> {
        let handle = self.objs[obj.hid].as_ref().unwrap();
        let obj = unsafe { &*handle.ptr };
        match obj {
            ObjectInner::Model {
                model_name: _,
                fields,
            } => {
                if let Some(&member_obj) = fields.get(&member) {
                    Ok((member_obj, handle.ptr))
                } else {
                    Err(TwiError::MemberNotFound(member))
                }
            }
            oi => Err(TwiError::CannotGetMember(format!("{:?}", oi))),
        }
    }

    pub fn members(&self, obj: Object, members: Vec<String>) -> TwiResult<Object> {
        let mut res = obj;
        for member in members {
            (res, _) = self.member(res, member)?;
        }
        Ok(res)
    }

    pub fn alloc(&mut self, obj_inner: ObjectInner) -> Object {
        let ptr = Box::into_raw(Box::new(obj_inner));

        // allocate heap id
        let hid = if let Some(&hid) = self.free.first() {
            // allocate with free list
            self.objs[hid] = Some(ObjectHandle { alive: false, ptr });
            hid
        } else {
            // push at end
            self.objs.push(Some(ObjectHandle { alive: false, ptr }));
            self.objs.len() - 1
        };

        Object { hid }
    }

    pub fn gc(&mut self, roots: Vec<Object>) {
        // mark
        self.col = Collector::from(roots);
        while let Some(cur) = self.col.pop() {
            cur.trace(self);
        }

        // sweep
        for obj_opt in &mut self.objs {
            if let Some(objhandle) = obj_opt {
                if objhandle.alive {
                    objhandle.alive = false;
                } else {
                    unsafe {
                        dealloc(objhandle.ptr as *mut u8, Layout::new::<ObjectInner>());
                    }
                    *obj_opt = None;
                }
            }
        }
    }
}

impl Object {
    pub fn trace(&self, heap: &mut Heap) {
        let objref = unsafe {
            let objhandle = heap.objs[self.hid].as_mut().unwrap();
            if objhandle.alive {
                // this node is already examined
                return;
            }
            objhandle.alive = true;
            &*objhandle.ptr
        };
        dbg!(&objref);
        if let ObjectInner::Model {
            model_name: _,
            fields,
        } = objref
        {
            for &field in fields.values() {
                heap.col.push(field);
            }
        }
    }

    pub fn pointer(&self, heap: &Heap) -> *mut ObjectInner {
        let handle = heap.objs[self.hid].as_ref().unwrap();
        handle.ptr
    }

    /// ### Short for `references`
    /// let the reference references other object
    pub fn refs(&mut self, heap: &mut Heap, members: Vec<String>, other: Object) -> TwiResult<()> {
        if members.is_empty() {
            self.hid = other.hid;
        }
        let mut res = *self;
        let mut ptr = std::ptr::null_mut();
        let key = members.last().unwrap().clone();
        for member in members {
            (res, ptr) = heap.member(res, member)?;
        }

        let obj_inner = unsafe { &mut *ptr };
        match obj_inner {
            ObjectInner::Model {
                model_name: _,
                fields,
            } => {
                *fields.get_mut(&key).unwrap() = other;
            }
            obj => {
                //
                return Err(TwiError::CannotGetMember(format!("{:?}", obj)));
            }
        }

        Ok(())
    }
}
