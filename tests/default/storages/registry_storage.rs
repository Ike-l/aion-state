use std::{collections::HashMap, hash::Hash};

use tracing::{Level, event};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegistryStorage<ResourceId: Hash + Eq, StoredResource> {
    inner: HashMap<ResourceId, StoredResource>,
    calculated_len: usize,
    capacity: usize
}

impl<ResourceId: Hash + Eq, StoragedResource> Default for RegistryStorage<ResourceId, StoragedResource> {
    fn default() -> Self {
        let capacity = 1000;
        Self::new(capacity)
    }
}

impl<ResourceId: Hash + Eq, StoredResource> RegistryStorage<ResourceId, StoredResource> {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
            calculated_len: 0,
            capacity
        }
    }
}

impl<ResourceId: Eq + Hash, StoredResource> aion_state::prelude::RegistryStorage for RegistryStorage<ResourceId, StoredResource> {
    type ValueId = ResourceId;
    type Value = StoredResource;

    fn keys(&self) -> impl Iterator<Item = &Self::ValueId> {
        self.inner.keys()
    }

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<&mut Self::Value> {
        event!(Level::TRACE, "RegistryStorage get mut");

        self.inner.get_mut(value_id)
    }

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::Value
    ) -> Option<Self::Value> {
        event!(Level::TRACE, "RegistryStorage insert");

        let r = self.inner.insert(value_id, value);

        if r.is_none() {
            self.calculated_len += 1;
        }

        r
    }

    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::Value> {
        event!(Level::TRACE, "RegistryStorage remove");

        let r = self.inner.remove(value_id);

        if r.is_some() {
            self.calculated_len -= 1;
        }

        r
    }

    fn contains_key(
        &self, 
        value_id: &Self::ValueId
    ) -> bool {
        event!(Level::TRACE, "RegistryStorage contains key");

        self.inner.contains_key(value_id)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    unsafe fn next_insert_may_reallocates(&self) -> bool {
        self.calculated_len >= self.capacity
    }

    unsafe fn next_removal_may_reallocates(&self) -> bool {
        false
    }
}