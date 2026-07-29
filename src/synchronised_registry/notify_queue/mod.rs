use std::collections::HashMap;

#[derive(Default)]
pub struct NotifyQueue<ValueId> {
    queue: HashMap<ValueId, Vec<crate::prelude::sync::Mutex<(Option<std::task::Waker>, bool)>>>
}

impl<ValueId> NotifyQueue<ValueId> {
    pub fn wake(&self, _value_id: &ValueId) {

    }
}