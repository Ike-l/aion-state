use std::{collections::HashMap, hash::Hash};

use crate::prelude::{Waiter, sync::{Arc, Mutex}};

#[derive(Default)]
pub struct NotifyQueue<ValueId> {
    queue: HashMap<ValueId, Vec<Arc<Mutex<Waiter>>>>
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

    pub fn register(&mut self, value_id: ValueId) -> Arc<Mutex<Waiter>> {
        let waiter = Arc::new(Mutex::new(Waiter::default()));
        self.queue.entry(value_id).or_default().push(Arc::clone(&waiter));
        waiter
    }

    pub fn unregister(&mut self, value_id: &ValueId, waiter: &Arc<Mutex<Waiter>>) {
        if let Some(waiters) = self.queue.get_mut(value_id) {
            waiters.retain(|registered_waiter| {
                !Arc::ptr_eq(registered_waiter, waiter)
            });
        }
    }
}