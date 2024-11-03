use crate::{
    errors::TwiResult,
    runtime::{gc::objects::Object, runtime::runtime::Runtime},
};
use parse::ast::expr::ExprNode;

impl Runtime {
    pub fn eval(&mut self, expr: ExprNode) -> TwiResult<Object> {
        match expr {
            ExprNode::Literal(literal) => {
                //
                Ok(self.eval_literal(literal))
            }
            ExprNode::Identifer(ident) => {
                // track back stack
                self.getvar(ident)
            }
            ExprNode::New(typename) => {
                //
                self.eval_model(typename)
            }
            ExprNode::Member { base, members } => {
                let obj = self.getvar(base)?;
                self.heap.members(obj, members)
            }

            // boolean
            ExprNode::Eq { left, right } => self.eval_eq(*left, *right),
            ExprNode::Neq { left, right } => self.eval_ne(*left, *right),
            ExprNode::Gt { left, right } => self.eval_gt(*left, *right),
            ExprNode::Lt { left, right } => self.eval_lt(*left, *right),
            ExprNode::Geq { left, right } => self.eval_geq(*left, *right),
            ExprNode::Leq { left, right } => self.eval_leq(*left, *right),
            ExprNode::Not(expr) => self.eval_not(*expr),
            ExprNode::Neg(expr) => self.eval_neg(*expr),

            // arithmetic
            ExprNode::Add { left, right } => self.eval_add(*left, *right),
            ExprNode::Minus { left, right } => self.eval_minus(*left, *right),
            ExprNode::Mul { left, right } => self.eval_mul(*left, *right),
            ExprNode::Div { left, right } => self.eval_div(*left, *right),

            ExprNode::Packed(expr) => self.eval(*expr),

            // ExprNode::Call { name, args } => self.eval_call(name, args),
            ExprNode::Call { name, args } => {
                let return_val = self.eval_call(name, args)?;
                self.temp_ref(return_val);

                Ok(return_val)
            }
        }
    }
}
