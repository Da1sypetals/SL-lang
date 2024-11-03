use parse::ast::expr::ExprNode;

use crate::{
    errors::{TwiError, TwiResult},
    runtime::{
        gc::{
            objects::{Object, ObjectInner},
            value::Value,
        },
        runtime::runtime::Runtime,
    },
};

#[derive(Debug, Clone, Copy)]
enum Ordering {
    Less,
    Equal,
    Greater,
}

impl Ordering {
    fn lt(&self) -> bool {
        matches!(self, Ordering::Less)
    }

    fn gt(&self) -> bool {
        matches!(self, Ordering::Greater)
    }

    fn leq(&self) -> bool {
        matches!(self, Ordering::Less | Ordering::Equal)
    }

    fn geq(&self) -> bool {
        matches!(self, Ordering::Greater | Ordering::Equal)
    }
}

impl Runtime {
    #[inline(always)]
    fn _eq(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<bool> {
        let lobj = self.eval(left)?;
        let robj = self.eval(right)?;

        let lval = self.heap.get_value(lobj);
        let rval = self.heap.get_value(robj);

        let eq = match (lval, rval) {
            (Value::Func { hid: hid1, .. }, Value::Func { hid: hid2, .. }) => hid1 == hid2,
            (Value::Model { hid: hid1, .. }, Value::Model { hid: hid2, .. }) => hid1 == hid2,
            (Value::Nil, Value::Nil) => true,
            (Value::Int(i1), Value::Int(i2)) => i1 == i2,
            (Value::Float(f1), Value::Float(f2)) => f1 == f2,
            (Value::Teer(t1), Value::Teer(t2)) => t1 == t2,
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (l, r) => {
                return Err(TwiError::IncompatibleBinopType {
                    left: l.to_string(),
                    right: r.to_string(),
                })
            }
        };

        Ok(eq)
    }

    #[inline(always)]
    fn _order(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<Ordering> {
        let lobj = self.eval(left)?;
        let robj = self.eval(right)?;

        let lval = self.heap.get_value(lobj);
        let rval = self.heap.get_value(robj);

        let eq = match (lval, rval) {
            (Value::Int(i1), Value::Int(i2)) => {
                if i1 < i2 {
                    Ordering::Less
                } else if i1 == i2 {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            }
            (Value::Float(f1), Value::Float(f2)) => {
                if f1 < f2 {
                    Ordering::Less
                } else if f1 == f2 {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            }
            (l, r) => {
                return Err(TwiError::IncompatibleBinopType {
                    left: l.to_string(),
                    right: r.to_string(),
                })
            }
        };

        Ok(eq)
    }

    pub(crate) fn eval_eq(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<Object> {
        let eq = self._eq(left, right)?;
        Ok(self.alloc(ObjectInner::Bool(eq)))
    }

    pub(crate) fn eval_ne(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<Object> {
        let ne = !self._eq(left, right)?;
        Ok(self.alloc(ObjectInner::Bool(ne)))
    }

    pub(crate) fn eval_lt(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<Object> {
        let ord = self._order(left, right)?;
        Ok(self.alloc(ObjectInner::Bool(ord.lt())))
    }

    pub(crate) fn eval_gt(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<Object> {
        let ord = self._order(left, right)?;
        Ok(self.alloc(ObjectInner::Bool(ord.gt())))
    }

    pub(crate) fn eval_leq(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<Object> {
        let ord = self._order(left, right)?;
        Ok(self.alloc(ObjectInner::Bool(ord.leq())))
    }

    pub(crate) fn eval_geq(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<Object> {
        let ord = self._order(left, right)?;
        Ok(self.alloc(ObjectInner::Bool(ord.geq())))
    }

    pub(crate) fn eval_not(&mut self, expr: ExprNode) -> TwiResult<Object> {
        let obj = self.eval(expr)?;
        let val = self.heap.get_value(obj);

        if let Value::Bool(b) = val {
            Ok(self.alloc(ObjectInner::Bool(!b)))
        } else {
            Err(TwiError::IncompatibleUnopType(val.to_string()))
        }
    }

    pub(crate) fn eval_neg(&mut self, expr: ExprNode) -> TwiResult<Object> {
        let obj = self.eval(expr)?;
        let val = self.heap.get_value(obj);

        match val {
            Value::Int(x) => Ok(self.alloc(ObjectInner::Int(-x))),
            Value::Float(x) => Ok(self.alloc(ObjectInner::Float(-x))),
            _ => Err(TwiError::IncompatibleUnopType(val.to_string())),
        }
    }

    pub(crate) fn eval_add(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<Object> {
        let lobj = self.eval(left)?;
        let robj = self.eval(right)?;

        let lval = self.heap.get_value(lobj);
        let rval = self.heap.get_value(robj);

        match (lval, rval) {
            (Value::Int(i1), Value::Int(i2)) => {
                //
                Ok(self.alloc(ObjectInner::Int(i1 + i2)))
            }
            (Value::Float(f1), Value::Float(f2)) => Ok(self.alloc(ObjectInner::Float(f1 + f2))),
            (l, r) => Err(TwiError::IncompatibleBinopType {
                left: l.to_string(),
                right: r.to_string(),
            }),
        }
    }

    pub(crate) fn eval_minus(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<Object> {
        let lobj = self.eval(left)?;
        let robj = self.eval(right)?;

        let lval = self.heap.get_value(lobj);
        let rval = self.heap.get_value(robj);

        match (lval, rval) {
            (Value::Int(i1), Value::Int(i2)) => {
                //
                Ok(self.alloc(ObjectInner::Int(i1 - i2)))
            }
            (Value::Float(f1), Value::Float(f2)) => Ok(self.alloc(ObjectInner::Float(f1 - f2))),
            (l, r) => Err(TwiError::IncompatibleBinopType {
                left: l.to_string(),
                right: r.to_string(),
            }),
        }
    }

    pub(crate) fn eval_mul(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<Object> {
        let lobj = self.eval(left)?;
        let robj = self.eval(right)?;

        let lval = self.heap.get_value(lobj);
        let rval = self.heap.get_value(robj);

        match (lval, rval) {
            (Value::Int(i1), Value::Int(i2)) => {
                //
                Ok(self.alloc(ObjectInner::Int(i1 * i2)))
            }
            (Value::Float(f1), Value::Float(f2)) => Ok(self.alloc(ObjectInner::Float(f1 * f2))),
            (l, r) => Err(TwiError::IncompatibleBinopType {
                left: l.to_string(),
                right: r.to_string(),
            }),
        }
    }

    pub(crate) fn eval_div(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<Object> {
        let lobj = self.eval(left)?;
        let robj = self.eval(right)?;

        let lval = self.heap.get_value(lobj);
        let rval = self.heap.get_value(robj);

        match (lval, rval) {
            (Value::Int(i1), Value::Int(i2)) => {
                if i2 == 0 {
                    return Err(TwiError::DivisionByZero);
                }
                Ok(self.alloc(ObjectInner::Int(i1 / i2)))
            }
            (Value::Float(f1), Value::Float(f2)) => {
                if f2 == 0.0 {
                    return Err(TwiError::DivisionByZero);
                }
                Ok(self.alloc(ObjectInner::Float(f1 / f2)))
            }
            (l, r) => Err(TwiError::IncompatibleBinopType {
                left: l.to_string(),
                right: r.to_string(),
            }),
        }
    }

    pub(crate) fn eval_mod(&mut self, left: ExprNode, right: ExprNode) -> TwiResult<Object> {
        let lobj = self.eval(left)?;
        let robj = self.eval(right)?;

        let lval = self.heap.get_value(lobj);
        let rval = self.heap.get_value(robj);

        match (lval, rval) {
            (Value::Int(i1), Value::Int(i2)) => {
                if i2 == 0 {
                    return Err(TwiError::DivisionByZero);
                }
                Ok(self.alloc(ObjectInner::Int(i1 % i2)))
            }
            (l, r) => Err(TwiError::IncompatibleBinopType {
                left: l.to_string(),
                right: r.to_string(),
            }),
        }
    }
}
