use std::{collections::HashMap, hash::Hash};

use aion_state::prelude::WrappedValue;
use tracing::{Level, event};

use crate::default::primitives::accesses::access_result::Transmutable;

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

#[derive(Debug, PartialEq)]
pub enum ResourceWrapper<'a, T> {
    Unique(&'a mut T),
    Shared(&'a T)
}

impl<'a, T> ResourceWrapper<'a, T> {
    pub fn new(value: &'a mut T) -> Self {
        Self::Unique(value)
    }
}

impl<'a, T> WrappedValue for ResourceWrapper<'a, T> {
    type Value = T;

    fn as_unique(&mut self) -> &mut Self::Value {
        let Self::Unique(value) = self else { unreachable!() };
        value
    }
}

impl<'a, T> Transmutable for ResourceWrapper<'a, T> {
    fn transmute(self) -> Self {
        let Self::Unique(value) = self else { unreachable!() };
        Self::Shared(value)
    }
}

impl<ResourceId: Eq + Hash, StoredResource> aion_state::prelude::RegistryStorage for RegistryStorage<ResourceId, StoredResource> {
    type ValueId = ResourceId;
    type OwnedValue = StoredResource;
    type ReferencedValue<'a> = ResourceWrapper<'a, StoredResource> where Self: 'a;

    fn keys(&self) -> impl Iterator<Item = &Self::ValueId> {
        self.inner.keys()
    }
    
    fn get_mut_wrapped(
        &mut self,
        value_id: &Self::ValueId
    ) -> Option<Self::ReferencedValue<'_>>
    {
        event!(Level::TRACE, "RegistryStorage get mut");

        self.inner.get_mut(value_id).map(ResourceWrapper::new)
    }

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::OwnedValue
    ) -> Option<Self::OwnedValue> {
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
    ) -> Option<Self::OwnedValue> {
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