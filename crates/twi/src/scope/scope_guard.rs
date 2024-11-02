use crate::runtime::runtime::runtime::Runtime;

pub struct ScopeGuard {
    pub(crate) rt: *mut Runtime,
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        let rt = unsafe { &mut *self.rt };
        rt.scopes.pop();
    }
}
