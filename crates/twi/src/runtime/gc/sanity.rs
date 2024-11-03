use log::{error, info};

use super::gc::Heap;

impl Heap {
    pub fn sanity_check(&self) {
        for &free in &self.free {
            if let Some(_) = self.objs[free] {
                error!("Heap internal error: mismatch on heap index hid={}", free);
                std::process::exit(1);
            }
        }
        info!("Sanity check OK");
    }
}
