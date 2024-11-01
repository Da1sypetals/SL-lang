use parse::ast::expr::ExprNode;

use crate::errors::{TwiError, TwiResult};

use super::runtime::Runtime;

impl Runtime {
    pub fn exec_let(&mut self, ident: String, expr: ExprNode) -> TwiResult<()> {
        if self.cur_scope().vars.contains_key(&ident) {
            return Err(TwiError::DuplicateLocalBind(ident));
        }
        let val = self.eval(expr);
        self.cur_scope().add(ident, val);

        Ok(())
    }
}
