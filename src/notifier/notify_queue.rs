use std::{collections::HashMap, hash::Hash};

use crate::prelude::Waiter;

#[derive(Default)]
pub struct NotifyQueue<ValueId> {
    queue: HashMap<ValueId, Vec<crate::prelude::sync::Arc<crate::prelude::sync::Mutex<Waiter>>>>
}

impl<ValueId: Eq + Hash> NotifyQueue<ValueId> {
    pub fn wake(&self, value_id: &ValueId) {
        if let Some(waiters) = self.queue.get(value_id) {
            for waiter in waiters {
                let mut waiter = waiter.lock();
                waiter.set_ready_to_retry();
                waiter.wake();
            }
        }
    }
}