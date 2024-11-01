use parse::types::literal::Literal;

use crate::runtime::gc::objects::ObjectInner;

impl From<Literal> for ObjectInner {
    fn from(lit: Literal) -> Self {
        match lit {
            Literal::Int(x) => ObjectInner::Int(x),
            Literal::Float(x) => ObjectInner::Float(x),
            Literal::String(x) => ObjectInner::String(x),
            Literal::Bool(x) => ObjectInner::Bool(x),
            Literal::Teer(x) => ObjectInner::Teer(x),
            Literal::Nil => ObjectInner::Nil,
        }
    }
}
