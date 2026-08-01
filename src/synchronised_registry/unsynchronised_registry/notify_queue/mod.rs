use std::{collections::HashMap, hash::Hash};

#[derive(Default)]
pub struct NotifyQueue<ValueId> {
    queue: HashMap<ValueId, Vec<crate::prelude::sync::Mutex<(Option<std::task::Waker>, bool)>>>
}

impl<ValueId: Eq + Hash> NotifyQueue<ValueId> {
    pub fn wake(&self, value_id: &ValueId) {
        if let Some(waiters) = self.queue.get(value_id) {
            for waiter in waiters {
                let mut waiter = waiter.lock();
                waiter.1 = true;
                if let Some(waker) = waiter.0.take() {
                    waker.wake();
                }
            }
        }
    }
}